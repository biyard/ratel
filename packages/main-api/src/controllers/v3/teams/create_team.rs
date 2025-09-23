use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamOwner},
        user::UserTeam,
    },
    utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamRequest {
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
    pub html_contents: String,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamResponse {
    pub team_pk: String,
}

pub async fn create_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<Json<CreateTeamResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;
    let (team, _) =
        Team::find_by_username_prefix(&dynamo.client, &req.username, Default::default()).await?;

    if !team.is_empty() {
        return Err(Error2::Duplicate("Username already taken".into()));
    }
    let team = Team::new(
        req.nickname,
        req.profile_url,
        req.username,
        req.html_contents,
    );
    team.create(&dynamo.client).await?;
    let user_pk = user.pk.clone();
    TeamOwner::new(team.pk.clone(), user)
        .create(&dynamo.client)
        .await?;
    let team_pk = team.pk.clone();
    UserTeam::new(user_pk, team).create(&dynamo.client).await?;
    Ok(Json(CreateTeamResponse {
        team_pk: team_pk.to_string(),
    }))
}
