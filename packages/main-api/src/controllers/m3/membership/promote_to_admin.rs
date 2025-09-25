use std::collections::HashMap;

use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, aide,
    by_axum::{
        auth::Authorization,
        axum::{extract::Path, Extension, Json},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    models::dynamo_tables::main::user::{User, UserMembership},
    types::Membership,
    utils::{
        admin::check_admin_permission_shared_ddb,
        users_dynamo::{get_user_membership_by_user_id, update_user_membership},
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

/// Admin endpoint to promote a user to admin
/// POST /m3/admin/users/{user_id}/promote
pub async fn promote_user_to_admin(
    by_axum::axum::extract::State(ddb): by_axum::axum::extract::State<
        std::sync::Arc<aws_sdk_dynamodb::Client>,
    >,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
) -> Result<Json<SetMembershipResponse>> {
    // Check admin permission using shared DDB client
    check_admin_permission_shared_ddb(&ddb, auth).await?;

    // Get the target user (to verify they exist)
    let user_pk = format!("USER#{}", user_id);
    let _user = User::get(&ddb, &user_pk, Some(&crate::types::EntityType::User.to_string()))
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?
        .ok_or(Error::InvalidUser)?;

    // Get or create user membership using the builder pattern
    let mut membership = match get_user_membership_by_user_id(&ddb, &user_id).await {
        Ok(Some(membership)) => membership,
        Ok(None) => {
            // Create new admin membership using builder pattern
            UserMembership::builder(user_id.clone()).with_admin().build()
        }
        Err(e) => {
            tracing::error!("Failed to get user membership: {:?}", e);
            return Err(Error::Unknown(format!("Failed to get user membership: {}", e)));
        }
    };

    // Update to admin membership
    membership.membership_type = Membership::Admin;
    membership.updated_at = crate::utils::time::get_now_timestamp_millis();

    // Save the updated membership
    if get_user_membership_by_user_id(&ddb, &user_id).await?.is_some() {
        // Update existing membership
        update_user_membership(&ddb, &membership).await.map_err(|e| {
            tracing::error!("Failed to update user to admin: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    } else {
        // Create new membership
        membership.create(&ddb).await.map_err(|e| {
            tracing::error!("Failed to create admin membership: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    }

    Ok(Json(SetMembershipResponse {
        success: true,
        message: format!("Successfully promoted user {} to admin", user_id),
        user_id,
        new_membership: Membership::Admin,
    }))
}
