use bdk::prelude::*;

use dto::{
    GroupPermission, RatelResource, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::Query, extract::State},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::security::check_perm_without_error;

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
pub struct HasPostPermissionQuery {
    #[schemars(description = "Team ID")]
    pub team_id: Option<i64>,
    #[schemars(description = "Group Permission")]
    pub permission: Option<GroupPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct HasPostPermissionResponse {
    pub has_permission: bool,
}

pub async fn has_team_permission_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(HasPostPermissionQuery {
        team_id,
        permission,
    }): Query<HasPostPermissionQuery>,
) -> Result<Json<HasPostPermissionResponse>> {
    if team_id.is_none() || team_id.unwrap() == 0 {
        return Ok(Json(HasPostPermissionResponse {
            has_permission: false,
        }));
    }

    match check_perm_without_error(
        &pool,
        auth,
        RatelResource::Team {
            team_id: team_id.unwrap_or_default(),
        },
        permission.unwrap_or_default(),
    )
    .await
    {
        Ok(v) => {
            if v.id == 0 {
                Ok(Json(HasPostPermissionResponse {
                    has_permission: false,
                }))
            } else {
                Ok(Json(HasPostPermissionResponse {
                    has_permission: true,
                }))
            }
        }
        Err(_) => Ok(Json(HasPostPermissionResponse {
            has_permission: false,
        })),
    }
}
