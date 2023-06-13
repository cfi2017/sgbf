use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use axum::headers::authorization::Credentials;
use chrono::NaiveDate;
use firestore::FirestoreDb;
use tokio::select;
use tokio::sync::{mpsc, RwLock};
use tokio::time::timeout;
use tracing::info;
use sgbf_client::model::{Day, RosterEntryType};

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
    rx_handle: Arc<RwLock<mpsc::Receiver<()>>>
}

impl Cache {

    pub fn new(db: FirestoreDb, username: &str, password: &str) -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            last_update: Arc::new(RwLock::new(chrono::Utc::now())),
            inner: Arc::new(RwLock::new(Default::default())),
            credentials: (username.to_owned(), password.to_owned()),
            db,
            tx_handle: tx,
            rx_handle: Arc::new(RwLock::new(rx)),
        }
    }

    pub async fn mark_dirty(&self) {
        info!("explicitly updating cache");
        self.tx_handle.send(()).await.unwrap();
    }

    pub async fn start_polling(&self) {
        loop {
            self.update().await;

            let mut rx = self.rx_handle.write().await;
            // drain the receiver if something happened during an update
            while rx.try_recv().is_ok() {}

            _ = timeout(Duration::from_secs(60 * 5), rx.recv()).await;
        }
    }

    async fn update(&self) {
        info!("updating cache");
        let client = sgbf_client::Client::from_credentials(&self.credentials.0, &self.credentials.1).await;
        let calendar = client.get_calendar().await.unwrap();
        let mut inner = self.inner.write().await;
        inner.day_overviews = calendar.clone();
        let mut guard = self.last_update.write().await;
        *guard = chrono::Utc::now();
    }
}
