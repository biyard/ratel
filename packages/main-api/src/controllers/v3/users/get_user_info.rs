use crate::{
    AppState, Error2, models::user::UserMetadata, utils::dynamo_extractor::extract_user_metadata,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
pub type GetUserInfoResponse = Vec<UserMetadata>;

pub async fn get_user_info_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
) -> Result<Json<GetUserInfoResponse>, Error2> {
    let user: GetUserInfoResponse = extract_user_metadata(&dynamo.client, auth).await?;

    Ok(Json(user))
}
