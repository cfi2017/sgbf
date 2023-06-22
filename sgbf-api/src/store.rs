use anyhow::Context;
use axum::extract::{FromRef, State};
use axum::http;
use axum::http::StatusCode;
use firestore::FirestoreDb;
use firestore::struct_path::paths;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use tracing::debug;
use sgbf_client::client::axum::{AuthCache, AuthState};
use crate::server::ServerError::Unknown;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBinding {
    pub user_id: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
}

pub async fn store_token(db: &FirestoreDb, token: &str, user_id: &str) -> anyhow::Result<TokenBinding> {
    let mut hasher = Sha256::default();
    hasher.update(token);
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);
    let result = db.fluent()
        .update()
        .in_col("tokens")
        .document_id(&hash)
        .object(&TokenBinding {
            user_id: user_id.to_string(),
            expiry: chrono::Utc::now() + chrono::Duration::hours(48),
        })
        .execute::<TokenBinding>()
        .await;
    result.context("could not save token binding")
}

pub async fn get_uid_for_token(db: &FirestoreDb, token: &str) -> anyhow::Result<Option<String>> {
    let mut hasher = Sha256::default();
    hasher.update(token);
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);
    let result = db.fluent()
        .select()
        .by_id_in("tokens")
        .obj()
        .one(&hash)
        .await;
    let binding: Option<TokenBinding> = result.context("could not get token binding")?;
    Ok(binding.map(|binding| binding.user_id))
}

pub struct Uid(pub String);

pub async fn with_uid<B, S>(
    State(s): State<S>,
    mut req: http::Request<B>,
    next: axum::middleware::Next<B>
) -> Result<axum::response::Response, StatusCode>
    where FirestoreDb: FromRef<S> {
    let db = FirestoreDb::from_ref(&s);
    let auth_state = req.extensions().get::<AuthState>();
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

pub async fn store_user(db: &FirestoreDb, user: &User) -> anyhow::Result<User> {
    let result = db.fluent()
        .update()
        .in_col("users")
        .document_id(&user.id.to_string())
        .object(user)
        .execute::<User>()
        .await;
    result.context("could not save user")
}

pub async fn get_user(db: &FirestoreDb, user_id: &str) -> anyhow::Result<Option<User>> {
    let result = db.fluent()
        .select()
        .by_id_in("users")
        .obj()
        .one(&user_id.to_string())
        .await;
    let user: Option<User> = result.context("could not get user")?;
    Ok(user)
}
