mod reservations;
mod calendar;

use std::time::Duration;
use anyhow::Context;
use axum::extract::{FromRef, State};
use axum::{extract, Json};
use axum_macros::debug_handler;
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
pub use calendar::get_calendar;
pub use calendar::get_day;
pub use calendar::update_day;
pub use reservations::get_reservations;
use sgbf_client::client::axum::AuthCache;
use crate::server::UnknownServerError;
use crate::state::SharedState;
use crate::store::{get_user, store_token, store_user, Uid, User};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[debug_handler]
#[instrument(skip(state, payload), fields(user = %payload.username))]
pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, UnknownServerError> {
    let client = sgbf_client::Client::from_credentials(&payload.username, &payload.password).await.context("failed to create client")?;
    let token = client.get_token();
    let auth_cache = AuthCache::from_ref(&state);
    let user = client.get_user().await?;
    if let Some(user) = &user {
        info!(user.name = %user, user.id = %payload.username, "user logged in");
        let db = FirestoreDb::from_ref(&state);
        let id = payload.username;
        if get_user(&db, &id).await?.is_none() {
            store_user(&db, &User {
                name: user.to_owned(),
                id: id.to_string(),
                settings: Default::default()
            }).await?;
        }
        store_token(&db, &token, &id).await?;
    }
    auth_cache.add_token(token.clone(), Duration::from_secs(60 * 60 * 4), user);
    Ok(Json(LoginResponse { token }))
}

#[debug_handler]
#[instrument(skip(state), fields(user = %uid))]
pub async fn me(
    State(state): State<SharedState>,
    extract::Extension(Uid(uid)): extract::Extension<Uid>
) -> Result<Json<User>, UnknownServerError> {
    let db = FirestoreDb::from_ref(&state);
    let user = get_user(&db, &uid).await?.context("failed to get user")?;
    Ok(Json(user))
}
