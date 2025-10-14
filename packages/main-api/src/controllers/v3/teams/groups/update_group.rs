use crate::models::{team::TeamGroup, user::User};
use crate::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::utils::security::{RatelResource, check_any_permission_with_user};
use crate::{AppState, Error2};
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::Deserialize;
use bdk::prelude::*;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_sk: String,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupRequest {
    #[schemars(description = "Group name to update")]
    pub name: Option<String>,
    #[schemars(description = "Group description to update")]
    pub description: Option<String>,
    #[schemars(description = "Group permissions to update")]
    pub permissions: Option<Vec<TeamGroupPermission>>,
}

pub async fn update_group_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(params): Path<UpdateGroupPathParams>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<(), Error2> {
    let user = user.ok_or(Error2::Unauthorized("Authentication required".into()))?;

    let required_permissions = if req.permissions.is_some() {
        vec![TeamGroupPermission::TeamAdmin]
    } else {
        vec![
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::GroupEdit,
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

    let mut need_update_user_permissions = false;

    let mut updater = TeamGroup::updater(params.team_pk.clone(), params.group_sk.clone());

    if let Some(name) = req.name {
        updater = updater.with_name(name);
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description);
    }
    if let Some(permissions) = req.permissions {
        //FIXME: Permission change should be restricted to team owners only
        updater = updater.with_permissions(TeamGroupPermissions(permissions).into());
        need_update_user_permissions = true;
    }
    updater.execute(&dynamo.client).await?;

    if need_update_user_permissions {
        //FIXME: Update user permissions
    }
    Ok(())
}
