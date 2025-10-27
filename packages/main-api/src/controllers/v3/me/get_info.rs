use crate::{
    AppState, Error,
    constants::SESSION_KEY_USER_ID,
    models::user::{UserDetailResponse, UserMetadata},
    types::Partition,
};
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};

pub type GetInfoResponse = UserDetailResponse;

pub async fn get_info_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
) -> Result<Json<GetInfoResponse>, Error> {
    let user_pk: Partition = session
        .get(SESSION_KEY_USER_ID)
        .await?
        .ok_or(Error::Unauthorized("no session".to_string()))?;
    tracing::debug!("get_info_handler: user_pk = {}", user_pk);
    let user = UserMetadata::query(&dynamo.client, user_pk).await?;
    Ok(Json(user.into()))
}
