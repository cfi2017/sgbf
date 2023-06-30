use std::sync::Arc;
use anyhow::Context;
use reqwest::cookie;
use reqwest::cookie::CookieStore;
use serde::{Serialize};
use thiserror::Error;
use tracing::instrument;
use crate::parsing;
use crate::model::{Day, DayOverview, EditAction, ParticipantType, Reservation, RosterEntryType};
use crate::parsing::Parser;

pub struct Client {
    inner: reqwest::Client,
    cookie_provider: Arc<cookie::Jar>,
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("invalid token")]
    InvalidToken,
    #[error("unknown error")]
    Unknown(#[from] anyhow::Error)
}

impl Client {
    #[instrument(skip(password))]
    pub async fn from_credentials(username: &str, password: &str) -> anyhow::Result<Self> {
        let provider = Arc::new(cookie::Jar::default());
        let client = Self {
            inner: reqwest::Client::builder().cookie_provider(provider.clone()).build().unwrap(),
            cookie_provider: provider,
        };
        client.login(username, password).await.context("failed to login")?;
        Ok(client)
    }

    #[instrument(skip(token))]
    pub async fn from_token(token: &str) -> Self {
        let jar = cookie::Jar::default();
        jar.add_cookie_str(format!("PHPSESSID={}", token).as_str(), &reqwest::Url::parse(BASE_URL).unwrap());
        let provider = Arc::new(jar);
        

        Self {
            inner: reqwest::Client::builder().cookie_provider(provider.clone()).build().unwrap(),
            cookie_provider: provider,
        }
    }

    pub fn get_token(&self) -> String {
        let jar = self.cookie_provider.as_ref();
        let cookie = jar.cookies(&reqwest::Url::parse(BASE_URL).unwrap()).unwrap();
        let cookie = cookie.to_str().unwrap();
        let cookie = cookie.split(';').next().unwrap();
        let cookie = cookie.split('=').nth(1).unwrap();
        cookie.to_string()
    }

    #[instrument(skip(self, password))]
    async fn login(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let url = format!("{}{}", BASE_URL, PATH_LOGIN);
        let body = LoginBody {
            username: username.to_string(),
            password: password.to_string(),
            save: "Login".to_string(),
        };
        let request = self.inner.post(url)
            .form(&body)
            .build()
            .context("failed to build request")?;
        let _ = self.inner.execute(request).await.context("failed to execute request")?;
        Ok(())
    }

    pub async fn get_user(&self) -> Result<Option<String>> {
        let url = format!("{}{}", BASE_URL, PATH_MENU);
        let request = self.inner.get(url)
            .build()
            .context("Failed to build request")?;
        let response = self.inner.execute(request).await.context("Failed to execute request")?;
        let body = response.text().await.context("Failed to read response body")?;
        if body.contains("logout") {
            let username = parsing::Parser::default().parse_menu(body)?;
            return Ok(Some(username));
        }
        Ok(None)
    }

    #[instrument(skip(self))]
    pub async fn get_calendar(&self) -> Result<Vec<DayOverview>> {
        let url = format!("{}{}", BASE_URL, PATH_CALENDAR);
        let request = self.inner.post(url)
            .form(&[("timebracket", "-9"), ("event_type", "0")])
            .build()
            .context("Failed to build request")?;
        let response = self.inner.execute(request).await.context("Failed to execute request")?;
        // body is html
        let body = response.text().await.context("Failed to read response body")?;
        if body.contains("Eintrag vorhanden") {
            return Err(ClientError::InvalidToken);
        }
        // parse
        Parser::default().parse_calendar(body).map_err(|e| e.into())
    }

    #[instrument(skip(self))]
    pub async fn get_reservations(&self) -> Result<Vec<Reservation>> {
        let url = format!("{}{}", BASE_URL, PATH_RESERVATIONS);
        let request = self.inner.post(url)
            // Dacft=all&Dtimeframe=-1
            .form(&[("Dacft", "all"), ("Dtimeframe", "-1")])
            .build()
            .context("Failed to build request")?;
        let response = self.inner.execute(request).await.context("Failed to execute request")?;
        // body is html
        let body = response.text().await.context("Failed to read response body")?;
        // parse
        Parser::default().parse_reservations(body).map_err(|e| e.into())
    }

    #[instrument(skip(self))]
    pub async fn get_day(&self, date: chrono::NaiveDate) -> anyhow::Result<Day> {
        let url = format!("{}{}", BASE_URL, PATH_DAY);
        let request = self.inner.get(url)
            // fe_t=participant_sf&select_date=2023-06-04&fe_f=text
            .query(&[("fe_t", "participant_sf"), ("select_date", date.format("%Y-%m-%d").to_string().as_ref()), ("fe_f", "text")])
            .build()
            .unwrap();
        let response = self.inner.execute(request).await.unwrap();
        // body is html
        let body = response.text().await.unwrap();
        // parse
        Parser::default().parse_day(body)
    }

    #[instrument(skip(self))]
    pub async fn update_day(&self, date: chrono::NaiveDate, day: Day) {
        let url = format!("{}{}", BASE_URL, PATH_DAY_UPDATE);
        let remarks = day.remarks.unwrap_or_default();
        let date = date.format("%Y-%m-%d").to_string();
        let mut form: Vec<(&str, &str)> = vec![
            ("RB_status", match day.entry_type {
                Some(t) => match t {
                    RosterEntryType::Definite => "2",
                    RosterEntryType::Tentative => "1",
                    RosterEntryType::Unavailable => "-1",
                },
                None => "0",
            }),
            ("TAfe_fsn", remarks.as_str()),
            ("Bfieldedit_send", "Speichern"),
            ("T_My_Action", match day.action {
                EditAction::Edit => "edit",
                EditAction::Add => "add",
            }),
            ("Tfe_t", match day.participant_type {
                ParticipantType::GliderPilot => "participant_sf",
            }),
            ("Tfe_f", day.format.as_str()),
            ("T_My_Date", date.as_ref())
        ];
        let id = day.id.unwrap_or_default();
        let id = id.to_string();
        if day.id.is_some() {
            form.push(("Tfe_r", id.as_ref()));
        }
        let request = self.inner.post(url)
            .form(&form)
            .build()
            .unwrap();
        let _ = self.inner.execute(request).await.unwrap();
    }
}

const BASE_URL: &str = "https://schlepppiloten.ch";
const PATH_MENU: &str = "/menu.php";
const PATH_LOGIN: &str = "/edit/login_check.php";
const PATH_CALENDAR: &str = "/roster/list_roster_new.php";
const PATH_DAY: &str = "/roster/participant_edit.php";
const PATH_DAY_UPDATE: &str = "/roster/participant_update.php";
const PATH_RESERVATIONS: &str = "/roster/reservation_aircraft.php";

#[derive(Debug, Clone, Serialize)]
struct LoginBody {
    #[serde(rename = "login_user")]
    username: String,
    #[serde(rename = "login_password")]
    password: String,
    save: String,
}

#[cfg(feature = "axum")]
pub mod axum {
    use axum::extract::{FromRef, FromRequestParts, State};
    use axum::{async_trait, http};
    use axum::http::StatusCode;
    use axum::http::request::Parts;

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    
    use tokio::time::{sleep};
    
    use tracing::{debug, info};

    #[async_trait]
    impl <S> FromRequestParts<S> for super::Client
        where AuthCache: FromRef<S>,
              S: Send + Sync
    {
        type Rejection = StatusCode;
        async fn from_request_parts(parts: &mut Parts, _s: &S) -> Result<Self, Self::Rejection> {
            // get AuthState from extensions
            let auth_state = parts.extensions.get::<AuthState>();
            if let Some(auth_state) = auth_state {
                if let Some((token, username)) = &auth_state.0 {
                    debug!("user {}", username);
                    return Ok(super::Client::from_token(token).await)
                }
            }
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct AuthCache {
        pub tokens: Arc<Mutex<HashMap<String, (Instant, Option<String>)>>>
    }

    impl AuthCache {
        pub fn new() -> Self {
            Default::default()
        }

        pub(crate) fn is_token_invalid(&self, token: &str) -> bool {
            let guard = self.tokens.lock().unwrap();
            let token = guard.get(token);
            match token {
                Some((_, ok)) => ok.is_some(),
                None => false
            }
        }

        // ensure that the token is valid and not expired
        pub fn is_token_authenticated(&self, token: &str) -> bool {
            let now = Instant::now();
            let guard = self.tokens.lock().unwrap();
            let token = guard.get(token);
            match token {
                Some((time, ok)) => time > &now && ok.is_some(),
                None => false
            }
        }

        pub fn get_state(&self, token: &str) -> Option<String> {
            let guard = self.tokens.lock().unwrap();
            let token = guard.get(token);
            match token {
                Some((_, ok)) => ok.clone(),
                None => None
            }
        }

        pub fn add_token(&self, token: String, duration: Duration, result: Option<String>) {
            let now = Instant::now();
            let expires = now + duration;
            self.tokens.lock().unwrap().insert(token, (expires, result));
        }

        pub fn remove_expired_tokens(&self) {
            let now = Instant::now();
            let mut tokens = self.tokens.lock().unwrap();
            tokens.retain(|_, (expires, _)| expires > &mut now.to_owned());
        }

        pub async fn start_polling(&self) {
            loop {
                debug!("updating auth cache");
                self.remove_expired_tokens();
                info!("auth cache updated");
                sleep(Duration::from_secs(60 * 5)).await;
            }
        }
    }

    // token, username
   #[derive(Clone)]
    pub struct AuthState(pub Option<(String, String)>);

    pub async fn auth<B, S>(
        State(s): State<S>,
        mut req: http::Request<B>,
        next: axum::middleware::Next<B>
    ) -> Result<axum::response::Response, StatusCode>
        where AuthCache: FromRef<S> {
        let header = req.headers().get("Authorization");
        if let Some(header) = header {
            if let Ok(header) = header.to_str() {
                if let Some(token) = header.strip_prefix("Bearer ") {
                    // get token
                    let cache = AuthCache::from_ref(&s);

                    let token = token.to_owned();
                    // create client
                    if cache.is_token_authenticated(&token) {
                        // return client
                        let state = cache.get_state(&token);
                        req.extensions_mut()
                            .insert(AuthState(state.map(|s| (token.to_owned(), s))));
                        return Ok(next.run(req).await);
                    }
                    if cache.is_token_invalid(&token) {
                        // return error
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                    let client = super::Client::from_token(&token).await;
                    let result = client.get_user().await;
                    if let Ok(result) = result {
                        cache.add_token(token.to_owned(), Duration::from_secs( 60 * 10), result.clone());
                        if result.is_some() {
                            req.extensions_mut()
                                .insert(AuthState(result.map(|s| (token.to_owned(), s))));
                            return Ok(next.run(req).await)
                        }
                    }
                    // return client
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
        Err(StatusCode::UNAUTHORIZED)
    }

}