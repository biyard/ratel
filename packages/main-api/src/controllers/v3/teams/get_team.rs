use crate::{
    AppState, Error,
    models::{
        TeamMetadata,
        team::{Team, TeamGroup, TeamGroupQueryOption, TeamOwner},
        user::User,
    },
    types::EntityType,
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::Deserialize;

use super::dto::TeamDetailResponse;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetTeamPathParams {
    pub team_pk: String,
}

pub type GetTeamResponse = TeamDetailResponse;

pub async fn get_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(path): Path<GetTeamPathParams>,
) -> Result<Json<GetTeamResponse>, Error> {
    // Get team
    let team = TeamMetadata::query(&dynamo.client, path.team_pk).await?;
    if team.is_empty() {
        return Err(Error::NotFound("Team not found".into()));
    }
    let team = TeamDetailResponse::from(team);

    Ok(Json(team))
}
