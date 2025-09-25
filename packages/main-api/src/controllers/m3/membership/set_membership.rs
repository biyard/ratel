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

/// Admin endpoint to set membership for a specific user
/// POST /m3/admin/users/{user_id}/membership
pub async fn set_user_membership(
    by_axum::axum::extract::State(ddb): by_axum::axum::extract::State<
        std::sync::Arc<aws_sdk_dynamodb::Client>,
    >,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(payload): Json<SetMembershipRequest>,
) -> Result<Json<SetMembershipResponse>> {
    // Check admin permission using shared DDB client
    check_admin_permission_shared_ddb(&ddb, auth).await?;

    // Get the target user (to verify they exist)
    let user_pk = format!("USER#{}", user_id);
    let _user = User::get(
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

    // Get or create user membership using the builder pattern
    let mut membership = match get_user_membership_by_user_id(&ddb, &user_id).await {
        Ok(Some(existing_membership)) => existing_membership,
        Ok(None) => {
            // Create new membership using builder pattern
            match payload.membership_type {
                Membership::Enterprise => UserMembership::builder(user_id.clone())
                    .with_enterprise(payload.custom_capabilities.clone())
                    .build(),
                Membership::Free => UserMembership::builder(user_id.clone()).with_free().build(),
                Membership::Pro => UserMembership::builder(user_id.clone()).with_pro().build(),
                Membership::Max => UserMembership::builder(user_id.clone()).with_max().build(),
                Membership::VIP => UserMembership::builder(user_id.clone()).with_vip().build(),
                Membership::Admin => UserMembership::builder(user_id.clone())
                    .with_admin()
                    .build(),
            }
        }
        Err(e) => {
            tracing::error!("Failed to get user membership: {:?}", e);
            return Err(Error::Unknown(format!(
                "Failed to get user membership: {}",
                e
            )));
        }
    };

    // Update membership type and reset capabilities
    membership.membership_type = payload.membership_type;
    membership.updated_at = crate::utils::time::get_now_timestamp_millis();

    // Set custom capabilities for Enterprise if provided
    if payload.membership_type == Membership::Enterprise {
        if let Some(custom_capabilities) = payload.custom_capabilities {
            membership.set_space_capabilities(custom_capabilities);
        } else {
            // Reset to default enterprise capabilities
            membership.reset_monthly_quotas();
        }
    } else {
        // Reset to default capabilities for the membership type
        membership.reset_monthly_quotas();
    }

    // Save the updated membership
    if get_user_membership_by_user_id(&ddb, &user_id)
        .await?
        .is_some()
    {
        // Update existing membership
        update_user_membership(&ddb, &membership)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update user membership: {:?}", e);
                Error::Unknown(format!("DynamoDB error: {}", e))
            })?;
    } else {
        // Create new membership
        membership.create(&ddb).await.map_err(|e| {
            tracing::error!("Failed to create user membership: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    }

    Ok(Json(SetMembershipResponse {
        success: true,
        message: format!(
            "Successfully updated membership to {:?}",
            payload.membership_type
        ),
        user_id,
        new_membership: payload.membership_type,
    }))
}
