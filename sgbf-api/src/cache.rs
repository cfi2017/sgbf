use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::{bail, Context};
use axum::headers::authorization::Credentials;
use chrono::NaiveDate;
use firestore::FirestoreDb;
use onesignal_rust_api::apis;
use onesignal_rust_api::apis::configuration::Configuration;
use onesignal_rust_api::models::{Notification, StringMap};
use tokio::select;
use tokio::sync::{mpsc, RwLock};
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};
use sgbf_client::model::{Day, DayOverview, RosterEntryType};
use crate::config::CacheConfig;

const REGISTERED_PILOTS_THRESHOLD: u32 = 10;

#[derive(Debug, Default, Clone)]
pub struct Calendar {
    pub day_overviews: Vec<sgbf_client::model::DayOverview>,
    pub days: HashMap<NaiveDate, (Instant, Day)>
}

impl Calendar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_dirty(&self, day: NaiveDate) -> bool {
        let overview = self.day_overviews.iter().find(|overview| overview.date == day);
        let day = self.days.get(&day);
        match (overview, day) {
            (Some(overview), Some((expiry, day))) => {
                if Instant::now() > *expiry {
                    return true;
                }
                let stats = &overview.registered_pilots;
                let registered = day.entries.iter().filter(|entry| entry.entry_type == RosterEntryType::Definite).count();
                let tentative = day.entries.iter().filter(|entry| entry.entry_type == RosterEntryType::Tentative).count();
                stats.definitive as usize != registered || stats.tentative as usize != tentative
            },
            (Some(_), None) => true,
            (None, Some(_)) => true,
            (None, None) => true
        }
    }
}

pub type CacheRef = Arc<Cache>;

#[derive(Debug, Clone)]
pub struct Cache {
    pub last_update: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
    pub inner: Arc<RwLock<Calendar>>,
    db: FirestoreDb,
    credentials: (String, String),
    tx_handle: mpsc::Sender<()>,
    rx_handle: Arc<RwLock<mpsc::Receiver<()>>>,
    notifications: Arc<Option<Configuration>>
}

impl Cache {

    pub fn new(db: FirestoreDb, config: &CacheConfig, notifications: Option<Configuration>) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            last_update: Arc::new(RwLock::new(chrono::Utc::now())),
            inner: Arc::new(RwLock::new(Default::default())),
            credentials: (config.username.to_owned(), config.password.to_owned()),
            db,
            tx_handle: tx,
            rx_handle: Arc::new(RwLock::new(rx)),
            notifications: Arc::new(notifications)
        }
    }

    pub async fn mark_dirty(&self) {
        info!("explicitly updating cache");
        self.tx_handle.send(()).await.unwrap();
    }

    #[instrument(skip(self))]
    pub async fn start_polling(&self) {
        loop {
            debug!("updating cache");
            let result = self.update().await;
            if result.is_err() {
                error!("failed to update cache {}", result.err().unwrap());
            } else {
                info!("cache updated");
            }
            let mut rx = self.rx_handle.write().await;
            // drain the receiver if something happened during an update
            while rx.try_recv().is_ok() {}

            _ = timeout(Duration::from_secs(60 * 5), rx.recv()).await;
        }
    }

    async fn update(&self) -> anyhow::Result<()> {
        let client = sgbf_client::Client::from_credentials(&self.credentials.0, &self.credentials.1).await.context("failed to create client")?;
        // update calendar
        let calendar = client.get_calendar().await.context("failed to update calendar")?;
        let mut inner = self.inner.write().await;
        let old_calendar = inner.clone();
        // todo: compare old calendar to new one, send notifications for changes
        inner.day_overviews = calendar.clone();
        let mut guard = self.last_update.write().await;
        *guard = chrono::Utc::now();
        // only keep cached days in current period
        inner.days.retain(|date, (_, _)| {
            calendar.iter().any(|overview| overview.date == *date)
        });
        // check if any day caches are dirty or expired, update if necessary
        let days = inner.days.clone();
        for date in days.keys() {
            if inner.is_dirty(*date) {
                let day = client.get_day(*date).await.context("failed to update day cache")?;
                inner.days.insert(*date, (Instant::now() + Duration::from_secs(600 * 3), day));
                // todo: compare old day to new one, send notifications for changes
            }
        }
        let new_calendar = inner.clone();
        self.compare_calendars(old_calendar, new_calendar).await?;
        Ok(())
    }

    async fn compare_calendars(&self, mut old: Calendar, mut new: Calendar) -> anyhow::Result<()> {
        if old.day_overviews.is_empty() || new.day_overviews.is_empty() {
            return Ok(());
        }
        // transform each day into vector of changes
        if old.day_overviews.first().map(|overview| overview.date) != new.day_overviews.first().map(|overview| overview.date) {
            // remove first of old days
            old.day_overviews.remove(0);
        }
        if old.day_overviews.last().map(|overview| overview.date) != new.day_overviews.last().map(|overview| overview.date) {
            // remove last of old days
            new.day_overviews.pop();
        }

        if old.day_overviews.len() != new.day_overviews.len() {
            bail!("calendar length mismatch, cannot compare calendars");
        }

        // zip old and new days together
        let mut overviews = old.day_overviews.into_iter().zip(new.day_overviews.into_iter());
        for (old_overview, new_overview) in overviews {
            if old_overview.date != new_overview.date {
                bail!("calendar date mismatch, cannot compare calendars");
            }
            let relevant_pilots = new.days.get(&new_overview.date)
                .map(|(_, day)| day.entries.iter()
                    .map(|entry| entry.name.clone()).collect::<Vec<_>>())
                .unwrap_or_default();
            // change in registered pilots
            if old_overview.registered_pilots.definitive < new_overview.registered_pilots.definitive
                && new_overview.registered_pilots.definitive == REGISTERED_PILOTS_THRESHOLD {
                // todo: notification for interested pilots
                info!("{} pilot threshold reached for {}", REGISTERED_PILOTS_THRESHOLD, new_overview.date);
                self.send_notification(&format!("{} pilot threshold reached for {}", REGISTERED_PILOTS_THRESHOLD, new_overview.date)).await?;
            }

            let old_entries = old_overview.entries;
            let new_entries = new_overview.entries;
            // get entries in new entries that are not in old entries
            let new_entries = new_entries.into_iter().filter(|new_entry| {
                !old_entries.iter().any(|old_entry| old_entry.name == new_entry.name)
            }).collect::<Vec<_>>();
            for new_entry in new_entries {
                // todo: notification for interested pilots
                info!("new entry {} for {} (type {:?})", new_entry.name, new_overview.date, new_entry.entry_type);
                self.send_notification(&format!("new entry {} for {} (type {:?})", new_entry.name, new_overview.date, new_entry.entry_type)).await?;
            }
        }
        Ok(())
    }

    async fn send_notification(&self, text: &str) -> anyhow::Result<()> {
        if let Some(config) = self.notifications.as_ref() {
            // todo: make app id configurable
            let mut notification = Notification::new(String::from("597019c4-d476-4efa-9832-34791456301c"));
            let mut contents = StringMap::new();
            contents.en = Some(text.to_owned());
            notification.contents = Some(Box::new(contents));
            // todo: configure users dynamically
            notification.include_external_user_ids = Some(vec![self.credentials.0.clone()]);
            apis::default_api::create_notification(config, notification).await.context("failed to send notification")?;
        }
        Ok(())
    }

    async fn compare_days(&self, old: Day, new: Day) {

    }
}
