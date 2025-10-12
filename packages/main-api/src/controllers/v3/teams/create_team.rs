use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamOwner},
        user::{User, UserTeam},
    },
    utils::{
        validator::{validate_description, validate_image_url, validate_username},
    },
};
use dto::by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateTeamRequest {
    #[schemars(description = "Team name. SHOULD be unique")]
    #[validate(custom(function = "validate_username"))]
    pub username: String,

    #[schemars(description = "Team display name. (3 ~ 10 Characters)")]
    pub nickname: String,

    #[schemars(description = "Team profile URL to update")]
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: String,
    #[schemars(description = "Team description. Max length: 160 characters")]
    #[validate(custom(function = "validate_description"))]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateTeamResponse {
    pub team_pk: String,
}

pub async fn create_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<Json<CreateTeamResponse>, Error2> {
    let user = user.ok_or(Error2::Unauthorized("Authentication required".into()))?;
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
