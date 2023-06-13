use std::time::Duration;
use anyhow::Context;
use axum::{extract, Json};
use axum::extract::State;
use axum_macros::debug_handler;
use log::info;
use opentelemetry::trace::SpanKind::Server;
use sgbf_client::model::{Day, DayOverview, RosterEntry};
use serde::{Serialize, Deserialize};
use crate::server::{ServerError, UnknownServerError};
use crate::state::SharedState;

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
    let auth_cache = state.inner.read().unwrap().auth_cache.clone();
    let user = client.get_user().await?;
    if let Some(user) = &user {
        info!("logged in as {} (uid {})", user, payload.username);
    }
    auth_cache.add_token(token.clone(), Duration::from_secs(60 * 60 * 4), user);
    Ok(Json(LoginResponse { token }))
}

pub async fn get_calendar(
    _client: sgbf_client::Client,
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
    extract::Query(query): extract::Query<GetDayQuery>,
    extract::Json(payload): extract::Json<Day>
) -> Result<(), UnknownServerError> {
    client.update_day(query.date, payload).await;
    Ok(())
}
