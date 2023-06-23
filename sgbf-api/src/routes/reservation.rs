use std::time::Duration;
use anyhow::Context;
use axum::{extract, Json};
use axum::extract::{FromRef, State};
use axum_macros::debug_handler;
use firestore::FirestoreDb;
use log::info;
use opentelemetry::trace::SpanKind::Server;
use sgbf_client::model::{Day, DayOverview, RosterEntry, RosterEntryType};
use serde::{Serialize, Deserialize};
use sgbf_client::client::axum::{AuthCache, AuthState};
use crate::server::{ServerError, UnknownServerError};
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
pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, UnknownServerError> {
    let client = sgbf_client::Client::from_credentials(&payload.username, &payload.password).await;
    let token = client.get_token();
    let auth_cache = AuthCache::from_ref(&state);
    let user = client.get_user().await?;
    if let Some(user) = &user {
        info!("logged in as {} (uid {})", user, payload.username);
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
pub async fn me(
    State(state): State<SharedState>,
    extract::Extension(Uid(uid)): extract::Extension<Uid>
) -> Result<Json<User>, UnknownServerError> {
    let db = FirestoreDb::from_ref(&state);
    let user = get_user(&db, &uid).await?.context("failed to get user")?;
    Ok(Json(user))
}

pub async fn get_calendar(
    // _client: sgbf_client::Client,
    State(state): State<SharedState>
) -> Result<Json<Vec<DayOverview>>, ServerError> {
    let cache = state.inner.read().unwrap().cache.clone();
    let calendar = cache.inner.read().await.day_overviews.clone();
    // let calendar = client.get_calendar().await;
    // if let Err(sgbf_client::client::ClientError::InvalidToken) = calendar {
    //     return Err(ServerError::InvalidToken);
    // }
    // let calendar = calendar.context("failed to get calendar")?;
    Ok(Json(calendar))
}

#[derive(Deserialize)]
pub struct GetDayQuery {
    date: chrono::NaiveDate,
}

pub async fn get_day(
    client: sgbf_client::Client,
    extract::Query(query): extract::Query<GetDayQuery>
) -> Result<Json<Day>, ServerError> {
    let day = client.get_day(query.date).await?;
    Ok(Json(day))
}

pub async fn update_day(
    client: sgbf_client::Client,
    State(state): State<SharedState>,
    extract::Query(query): extract::Query<GetDayQuery>,
    extract::Json(payload): extract::Json<Day>
) -> Result<Json<Day>, UnknownServerError> {
    // check and update notification settings
    client.update_day(query.date, payload).await;
    let day = client.get_day(query.date).await?;
    Ok(Json(day))
}
