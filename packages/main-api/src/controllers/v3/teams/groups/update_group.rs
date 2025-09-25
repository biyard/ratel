use crate::models::team::TeamGroup;
use crate::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::utils::security::{RatelResource, check_any_permission};
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

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
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<UpdateGroupPathParams>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<(), Error2> {
    let required_permissions = if req.permissions.is_some() {
        vec![TeamGroupPermission::TeamAdmin]
    } else {
        vec![
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::TeamEdit,
        ]
    };
    check_any_permission(
        &dynamo.client,
        auth,
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
