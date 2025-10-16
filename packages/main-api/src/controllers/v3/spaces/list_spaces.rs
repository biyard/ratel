// use super::*;
use crate::models::user::User;
use crate::models::{SpaceCommon, SpaceCommonQueryOption};
use crate::types::{ListItemsResponse, SpaceVisibility};
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ListSpacesResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn list_spaces_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
) -> Result<Json<ListItemsResponse<SpaceCommon>>, Error2> {
    let cli = &dynamo.client;

    let spaces = SpaceCommon::find_by_visibility(
        cli,
        SpaceCommon::generate_pk_for_find_by_visibility(
            crate::types::SpacePublishState::Published,
            SpaceVisibility::Public,
        ),
        SpaceCommonQueryOption::builder().limit(10),
    )
    .await?;

    Ok(Json(spaces.into()))
}
