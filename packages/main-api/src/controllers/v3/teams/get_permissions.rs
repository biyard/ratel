use crate::models::user::user_team_group::{UserTeamGroup, UserTeamGroupQueryOption};
use crate::types::{Partition, TeamGroupPermission, TeamGroupPermissions};
use crate::AppState;
use crate::models::user::User;
use aide::NoApi;
use axum::extract::{Query, State};
use axum::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetPermissionsQuery {
    #[schemars(description = "Team username")]
    pub team_username: String,
    #[schemars(description = "Permission to check")]
    pub permission: String,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetPermissionsResponse {
    #[schemars(description = "Whether the user has the permission")]
    pub has_permission: bool,
}

pub async fn get_permissions_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(GetPermissionsQuery { team_username, permission }): Query<GetPermissionsQuery>,
) -> Result<Json<GetPermissionsResponse>, crate::Error2> {
    tracing::debug!("Checking permission: team_username={}, permission={}, user={:?}", team_username, permission, user);

    // If no user is logged in, they have no permissions
    let user = match user {
        Some(u) => u,
        None => {
            return Ok(Json(GetPermissionsResponse {
                has_permission: false,
            }));
        }
    };

    // If the username matches the user's username, they have all permissions on their own account
    if user.username == team_username {
        return Ok(Json(GetPermissionsResponse {
            has_permission: true,
        }));
    }

    // Check if the user is a member of any team groups for this team
    let team_pk = Partition::Team(team_username.clone());
    let user_team_groups = UserTeamGroup::find_by_team_pk(
        &dynamo.client,
        team_pk.to_string(),
        UserTeamGroupQueryOption::builder().limit(100),
    )
    .await?;

    // If no team groups found, user has no permissions
    if user_team_groups.0.is_empty() {
        return Ok(Json(GetPermissionsResponse {
            has_permission: false,
        }));
    }

    // Check if any of the user's team groups have the requested permission
    let permission_flag = match permission.as_str() {
        "write_posts" => TeamGroupPermission::PostWrite,
        "delete_posts" => TeamGroupPermission::PostDelete,
        _ => {
            return Ok(Json(GetPermissionsResponse {
                has_permission: false,
            }));
        }
    };

    for user_team_group in user_team_groups.0 {
        let permissions = TeamGroupPermissions::from(user_team_group.team_group_permissions);
        if permissions.0.contains(&permission_flag) {
            return Ok(Json(GetPermissionsResponse {
                has_permission: true,
            }));
        }
    }

    Ok(Json(GetPermissionsResponse {
        has_permission: false,
    }))
}
