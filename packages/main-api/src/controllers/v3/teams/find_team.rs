use crate::{
    AppState, Error2,
    models::team::{Team, TeamResponse},
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

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
    Extension(_auth): Extension<Option<Authorization>>,
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
