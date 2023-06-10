use anyhow::Context;
use axum::{extract, Json};
use axum_macros::debug_handler;
use opentelemetry::trace::SpanKind::Server;
use sgbf_client::model::{Day, DayOverview, RosterEntry};
use serde::{Serialize, Deserialize};
use crate::server::{ServerError, UnknownServerError};

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
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, UnknownServerError> {
    let client = sgbf_client::Client::from_credentials(&payload.username, &payload.password).await;
    let token = client.get_token();
    Ok(Json(LoginResponse { token }))
}

pub async fn get_calendar(
    client: sgbf_client::Client
) -> Result<Json<Vec<DayOverview>>, ServerError> {
    let calendar = client.get_calendar().await;
    if let Err(sgbf_client::client::ClientError::InvalidToken) = calendar {
        return Err(ServerError::InvalidToken);
    }
    let calendar = calendar.context("failed to get calendar")?;
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
