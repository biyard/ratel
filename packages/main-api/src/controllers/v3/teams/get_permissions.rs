use crate::{
    AppState, Error2,
    models::user::User,
    types::TeamGroupPermission,
    utils::security::{RatelResource, check_any_permission_from_user},
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::extract::{Json, Query, State},
};

use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct HasTeamPermissionQuery {
    #[schemars(description = "Team ID (string)")]
    pub team_pk: Option<String>,
    #[schemars(description = "Team Group Permission")]
    pub permission: Option<TeamGroupPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct GetPermissionsResponse {
    pub has_permission: bool,
}

pub async fn get_permissions_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(HasTeamPermissionQuery {
        team_pk,
        permission,
    }): Query<HasTeamPermissionQuery>,
) -> Result<Json<GetPermissionsResponse>, Error2> {
    // Early return if no team_pk or permission provided
    if team_pk.is_none() || team_pk.as_ref().unwrap().is_empty() || permission.is_none() {
        return Ok(Json(GetPermissionsResponse {
            has_permission: false,
        }));
    }

    // If no user is authenticated, return false
    if user.is_none() {
        return Ok(Json(GetPermissionsResponse {
            has_permission: false,
        }));
    }

    let team_pk = team_pk.unwrap();
    let permission = permission.unwrap();
    let user = user.unwrap();

    // Use v3 permission checking with DynamoDB
    match check_any_permission_from_user(
        &dynamo.client,
        user.pk.to_string(),
        RatelResource::Team { team_pk },
        vec![permission],
    )
    .await
    {
        Ok(()) => Ok(Json(GetPermissionsResponse {
            has_permission: true,
        })),
        Err(_) => Ok(Json(GetPermissionsResponse {
            has_permission: false,
        })),
    }
}
