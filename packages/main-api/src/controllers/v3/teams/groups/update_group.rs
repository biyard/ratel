//NOTE: REMOVE
#![allow(unused)]
use crate::types::TeamGroupPermission;
use crate::{AppState, Error2};
use dto::{
    GroupGetResponse,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension,
            extract::{Json, Path, State},
        },
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_id: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_id: String,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupRequest {
    #[schemars(description = "Group name to update")]
    pub name: Option<String>,
    #[schemars(description = "Group description to update")]
    pub description: Option<String>,
    #[schemars(description = "Group image URL to update")]
    pub image_url: Option<String>,
    #[schemars(description = "Group permissions to update")]
    pub permissions: Option<Vec<TeamGroupPermission>>,
}

pub async fn update_group_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<UpdateGroupPathParams>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<(), Error2> {
    Err(Error2::InternalServerError("Not implemented".into()))
}
