use axum_macros::debug_handler;
use tracing::instrument;
use axum::{extract, Json};
use axum::extract::State;
use serde::Deserialize;
use sgbf_client::model::{Day, DayOverview, Overlaps};
use crate::server::{ServerError, UnknownServerError};
use crate::state::SharedState;
use crate::store::Uid;

fn default_calendar_limit() -> usize {
    31
}

#[derive(Deserialize, Debug, Clone)]
pub struct CalendarQuery {
    #[serde(default = "default_calendar_limit")]
    limit: usize
}

#[debug_handler]
#[instrument(skip(state), fields(limit = %query.limit, user = %_uid))]
pub async fn get_calendar(
    // _client: sgbf_client::Client,
    extract::Query(query): extract::Query<CalendarQuery>,
    State(state): State<SharedState>,
    extract::Extension(Uid(_uid)): extract::Extension<Uid>
) -> Result<Json<Vec<DayOverview>>, ServerError> {
    let cache = state.inner.read().unwrap().cache.clone();
    let calendar = cache.inner.read().await.day_overviews.clone();
    // only the first `limit` days
    let calendar: Vec<_> = calendar.into_iter().take(query.limit).collect();
    let reservations = cache.inner.read().await.reservations.clone();

    // for each day find reservations and add them to the day
    let calendar = calendar.into_iter().map(|day| {
        let reservations = reservations.iter().filter(|reservation| {
            reservation.period.overlaps(&day.date)
        }).cloned().collect();
        DayOverview {
            reservations: Some(reservations),
            ..day
        }
    }).collect();

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
    extract::Query(query): extract::Query<GetDayQuery>,
    State(state): State<SharedState>,
) -> Result<Json<Day>, ServerError> {
    let mut day = client.get_day(query.date).await?;
    let cache = state.inner.read().unwrap().cache.clone();
    let reservations = cache.inner.read().await.reservations.clone();
    day.reservations = Some(reservations.iter().filter(|reservation| {
        reservation.period.overlaps(&query.date)
    }).cloned().collect::<Vec<_>>());
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
