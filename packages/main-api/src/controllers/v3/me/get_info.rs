use crate::{
    AppState, Error2,
    constants::SESSION_KEY_USER_ID,
    models::user::{UserDetailResponse, UserMetadata},
    types::Partition,
};
use by_macros::openapi;
use dto::by_axum::axum::{Extension, Json, extract::State};

#[openapi(tag = "ME")]
pub async fn get_info_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
) -> Result<Json<UserDetailResponse>, Error2> {
    let user_pk: Partition = session
        .get(SESSION_KEY_USER_ID)
        .await?
        .ok_or(Error2::Unauthorized("no session".to_string()))?;
    let user = UserMetadata::query(&dynamo.client, user_pk).await?;
    Ok(Json(user.into()))
}
