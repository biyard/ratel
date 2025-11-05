use crate::{
    AppState, Error,
    models::{
        TeamGroup, UserTeamGroup,
        team::{Team, TeamOwner},
        user::{User, UserTeam},
    },
    types::Partition,
    utils::validator::{validate_description, validate_image_url, validate_username},
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{Json, extract::State},
};
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateTeamResponse {
    pub team_pk: Partition,
}

pub async fn create_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<Json<CreateTeamResponse>, Error> {
    let user = user.ok_or(Error::Unauthorized("Authentication required".into()))?;
    let (team, _) =
        Team::find_by_username_prefix(&dynamo.client, &req.username, Default::default()).await?;

    if !team.is_empty() {
        return Err(Error::Duplicate("Username already taken".into()));
    }

    let team_pk = Team::create_new_team(
        &user,
        &dynamo.client,
        req.nickname,
        req.profile_url,
        req.username,
        req.description,
    )
    .await?;

    Ok(Json(CreateTeamResponse { team_pk }))
}
