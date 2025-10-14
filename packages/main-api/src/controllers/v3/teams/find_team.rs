use crate::{AppState, Error2, models::{team::Team, user::User}};
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Query, State},
    },
};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use super::dto::TeamResponse;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct FindTeamQueryParams {
    #[schemars(description = "Search by username")]
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct FindTeamResponse {
    pub teams: Vec<TeamResponse>,
}

pub async fn find_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Query(params): Query<FindTeamQueryParams>,
) -> Result<Json<FindTeamResponse>, Error2> {
    if let Some(username) = params.username {
        let (teams, _) =
            Team::find_by_username_prefix(&dynamo.client, &username, Default::default()).await?;

        Ok(Json(FindTeamResponse {
            teams: teams.into_iter().map(TeamResponse::from).collect(),
        }))
    } else {
        Err(Error2::BadRequest("No search parameters provided".into()))
    }
}
