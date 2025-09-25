use std::collections::HashMap;

use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, aide,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::Path},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    models::dynamo_tables::main::user::User,
    types::Membership,
    utils::{
        admin::check_admin_permission_shared_ddb, users_dynamo::get_user_membership_by_user_id,
    },
};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct SetMembershipRequest {
    pub membership_type: Membership,
    /// Custom capabilities for Enterprise membership (optional)
    pub custom_capabilities: Option<HashMap<u32, i32>>,
}

#[derive(Debug, Serialize, aide::OperationIo, JsonSchema)]
pub struct SetMembershipResponse {
    pub success: bool,
    pub message: String,
    pub user_id: String,
    pub new_membership: Membership,
}

#[derive(Debug, Serialize, aide::OperationIo, JsonSchema)]
pub struct ListUsersResponse {
    pub users: Vec<UserInfo>,
    pub total_count: usize,
}

#[derive(Debug, Serialize, aide::OperationIo, JsonSchema)]
pub struct UserInfo {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub membership_type: Membership,
    pub membership_active: bool,
}

/// Admin endpoint to get user membership details
/// GET /m3/admin/users/{user_id}/membership
pub async fn get_user_membership(
    by_axum::axum::extract::State(ddb): by_axum::axum::extract::State<
        std::sync::Arc<aws_sdk_dynamodb::Client>,
    >,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
) -> Result<Json<UserInfo>> {
    // Check admin permission using shared DDB client
    check_admin_permission_shared_ddb(&ddb, auth).await?;

    // Get the target user
    let user_pk = format!("USER#{}", user_id);
    let user = User::get(
        &ddb,
        &user_pk,
        Some(&crate::types::EntityType::User.to_string()),
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to get user: {:?}", e);
        Error::Unknown(format!("DynamoDB error: {}", e))
    })?
    .ok_or(Error::InvalidUser)?;

    // Get user membership
    let membership = get_user_membership_by_user_id(&ddb, &user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user membership: {:?}", e);
            Error::Unknown(format!("Failed to get user membership: {}", e))
        })?
        .unwrap_or_else(|| {
            // If no membership found, assume Free membership using builder pattern
            crate::models::dynamo_tables::main::user::UserMembership::builder(user_id.clone())
                .with_free()
                .build()
        });

    Ok(Json(UserInfo {
        user_id,
        email: user.email,
        display_name: user.display_name,
        membership_type: membership.membership_type,
        membership_active: membership.is_active(),
    }))
}
