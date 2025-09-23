use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamOwner},
        user::UserTeam,
    },
    utils::{
        dynamo_extractor::extract_user,
        validator::{validate_description, validate_image_url, validate_nickname},
    },
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
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamRequest {
    #[schemars(description = "Team name. SHOULD be unique")]
    pub username: String,

    #[schemars(description = "Team display name. (3 ~ 10 Characters)")]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: String,

    #[schemars(description = "Team profile URL to update")]
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: String,
    #[schemars(description = "Team description. Max length: 160 characters")]
    #[validate(custom(function = "validate_description"))]
    pub description: String,
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
    let team = Team::new(req.nickname, req.profile_url, req.username, req.description);
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
