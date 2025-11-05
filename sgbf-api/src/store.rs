use std::fmt::Display;
use anyhow::Context;
use axum::extract::{FromRef, State};
use axum::http;
use axum::http::StatusCode;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use tracing::{debug, warn};
use sgbf_client::client::axum::{AuthCache, AuthState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub settings: UserSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub notifications: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub enabled: bool,
    pub flight_instructors: bool,
    pub potential_flight_instructors: bool,
    pub flight_instructor_requests: bool,
    pub tow_pilots: bool,
    pub potential_tow_pilots: bool,
    pub tow_pilot_requests: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TokenBinding {
    pub id: String,
    pub user_id: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
}

pub async fn clean_expired_tokens(db: &PgPool) -> anyhow::Result<()> {
    let result = sqlx::query(
        "DELETE FROM tokens WHERE expiry < $1"
    )
    .bind(Utc::now())
    .execute(db)
    .await
    .context("could not delete expired tokens")?;

    debug!("deleted {} expired tokens", result.rows_affected());
    Ok(())
}

pub async fn store_token(db: &PgPool, token: &str, user_id: &str) -> anyhow::Result<TokenBinding> {
    let mut hasher = Sha256::default();
    hasher.update(token);
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);
    let expiry = chrono::Utc::now() + chrono::Duration::hours(48);

    sqlx::query(
        "INSERT INTO tokens (id, user_id, expiry) VALUES ($1, $2, $3)
         ON CONFLICT (id) DO UPDATE SET user_id = $2, expiry = $3"
    )
    .bind(&hash)
    .bind(user_id)
    .bind(expiry)
    .execute(db)
    .await
    .context("could not save token binding")?;

    Ok(TokenBinding {
        id: hash,
        user_id: user_id.to_string(),
        expiry,
    })
}

pub async fn get_uid_for_token(db: &PgPool, token: &str) -> anyhow::Result<Option<String>> {
    let mut hasher = Sha256::default();
    hasher.update(token);
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);

    let binding: Option<TokenBinding> = sqlx::query_as(
        "SELECT id, user_id, expiry FROM tokens WHERE id = $1"
    )
    .bind(&hash)
    .fetch_optional(db)
    .await
    .context("could not get token binding")?;

    Ok(binding.map(|binding| binding.user_id))
}

#[derive(Debug, Clone)]
pub struct Uid(pub String);

impl Display for Uid {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub async fn with_uid<B, S>(
    State(s): State<S>,
    mut req: http::Request<B>,
    next: axum::middleware::Next<B>
) -> Result<axum::response::Response, StatusCode>
    where PgPool: FromRef<S> {
    let db = PgPool::from_ref(&s);
    let auth_state = req.extensions().get::<AuthState>();
    if let Err(err) = clean_expired_tokens(&db).await {
        warn!("could not clean expired tokens: {}", err);
    }
    if let Some(auth_state) = auth_state {
        if let Some((token, _)) = &auth_state.0 {
            let uid = get_uid_for_token(&db, token).await;
            if let Ok(Some(uid)) = uid {
                debug!("uid: {}", uid);
                req.extensions_mut().insert(Uid(uid));
                return Ok(next.run(req).await)
            }
        }
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn store_user(db: &PgPool, user: &User) -> anyhow::Result<User> {
    let settings_json = serde_json::to_value(&user.settings)
        .context("could not serialize user settings")?;

    sqlx::query(
        "INSERT INTO users (id, name, settings) VALUES ($1, $2, $3)
         ON CONFLICT (id) DO UPDATE SET name = $2, settings = $3"
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&settings_json)
    .execute(db)
    .await
    .context("could not save user")?;

    Ok(user.clone())
}

pub async fn get_user(db: &PgPool, user_id: &str) -> anyhow::Result<Option<User>> {
    let row: Option<(String, String, serde_json::Value)> = sqlx::query_as(
        "SELECT id, name, settings FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
    .context("could not get user")?;

    Ok(row.map(|(id, name, settings)| {
        let settings: UserSettings = serde_json::from_value(settings)
            .unwrap_or_default();
        User {
            id,
            name,
            settings,
        }
    }))
}
