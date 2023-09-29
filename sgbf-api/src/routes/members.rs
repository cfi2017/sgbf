use axum::{extract, Json};
use axum::extract::State;
use axum_macros::debug_handler;
use tracing::instrument;
use sgbf_client::model::{Member, Reservation};
use crate::server::ServerError;
use crate::state::SharedState;
use crate::store::Uid;

#[debug_handler]
#[instrument(skip(state), fields(user = %_uid))]
pub async fn get_members(
    // _client: sgbf_client::Client,
    State(state): State<SharedState>,
    extract::Extension(Uid(_uid)): extract::Extension<Uid>
) -> Result<Json<Vec<Member>>, ServerError> {
    let cache = state.inner.read().unwrap().cache.clone();
    let members = cache.inner.read().await.members.clone();
    Ok(Json(members))
}
