use super::dto::*;
use crate::{AppState, Error2, models::{team::TeamMetadata, user::User}};
use dto::by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetTeamPathParams {
    pub team_pk: String,
}

pub type GetTeamResponse = TeamDetailResponse;

pub async fn get_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(path): Path<GetTeamPathParams>,
) -> Result<Json<GetTeamResponse>, Error2> {
    let team = TeamMetadata::query(&dynamo.client, path.team_pk).await?;
    if team.is_empty() {
        return Err(Error2::NotFound("Team not found".into()));
    }
    let team = TeamDetailResponse::from(team);
    Ok(Json(team))
}
