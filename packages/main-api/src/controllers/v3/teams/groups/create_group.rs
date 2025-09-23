use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamGroup},
        user::UserTeamGroup,
    },
    types::{EntityType, TeamGroupPermission, TeamGroupPermissions},
    utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct CreateGroupPathParams {
    pub team_id: String,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub permissions: Vec<TeamGroupPermission>,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateGroupResponse {
    pub group_pk: String,
    pub group_sk: String,
}

pub async fn create_group_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<CreateGroupPathParams>,
    Json(req): Json<CreateGroupRequest>,
) -> Result<Json<CreateGroupResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    let team = Team::get(
        &dynamo.client,
        params.team_id.clone(),
        Some(EntityType::Team),
    )
    .await?;
    if team.is_none() {
        return Err(Error2::NotFound("Team not found".into()));
    }
    let team = team.unwrap();
    let user_pk = user.pk.clone();
    let group = TeamGroup::new(
        team.pk,
        req.name,
        req.description,
        TeamGroupPermissions(req.permissions),
    );

    group.create(&dynamo.client).await?;
    let group_pk = group.pk.clone();
    let group_sk = group.sk.clone();
    UserTeamGroup::new(user_pk, group)
        .create(&dynamo.client)
        .await?;

    Ok(Json(CreateGroupResponse {
        group_pk: group_pk.to_string(),
        group_sk: group_sk.to_string(),
    }))
}
