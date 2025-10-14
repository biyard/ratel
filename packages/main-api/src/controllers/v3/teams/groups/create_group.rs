use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamGroup},
        user::{User, UserTeamGroup},
    },
    types::{EntityType, TeamGroupPermission, TeamGroupPermissions},
    utils::security::{RatelResource, check_any_permission_with_user},
};
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::{Deserialize, Serialize};
use bdk::prelude::*;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct CreateGroupPathParams {
    pub team_pk: String,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
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
    NoApi(user): NoApi<Option<User>>,
    Path(params): Path<CreateGroupPathParams>,
    Json(req): Json<CreateGroupRequest>,
) -> Result<Json<CreateGroupResponse>, Error2> {
    let user = user.ok_or(Error2::Unauthorized("Authentication required".into()))?;
    // If Admin permissions are requested, require TeamAdmin
    let required_permissions = if req
        .permissions
        .iter()
        .any(|p| matches!(p, TeamGroupPermission::TeamAdmin))
    {
        vec![TeamGroupPermission::TeamAdmin]
    } else {
        vec![
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::TeamEdit,
        ]
    };

    check_any_permission_with_user(
        &dynamo.client,
        &user,
        RatelResource::Team {
            team_pk: params.team_pk.clone(),
        },
        required_permissions,
    )
    .await?;

    let team = Team::get(
        &dynamo.client,
        params.team_pk.clone(),
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

    // Add creator to the group
    UserTeamGroup::new(user_pk, group)
        .create(&dynamo.client)
        .await?;

    TeamGroup::updater(&group_pk, &group_sk)
        .increase_members(1)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(CreateGroupResponse {
        group_pk: group_pk.to_string(),
        group_sk: group_sk.to_string(),
    }))
}
