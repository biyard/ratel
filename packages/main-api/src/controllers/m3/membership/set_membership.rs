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
    types::{Membership, MembershipInfo},
    utils::admin::check_admin_permission_shared_ddb,
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

    // Get the target user
    let user_pk = format!("USER#{}", user_id);
    let mut user = User::get(&ddb, user_pk, Some("USER".to_string()))
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?
        .ok_or(Error::InvalidUser)?;

    // Create new membership info
    let new_membership_info = match payload.membership_type {
        Membership::Enterprise => {
            let custom_capabilities = payload.custom_capabilities.unwrap_or_else(|| {
                // Default enterprise capabilities
                let caps = HashMap::new();

                caps
            });
            MembershipInfo::new_enterprise(custom_capabilities)
        }
        other_type => MembershipInfo::from_membership(other_type),
    };

    // Update user's membership
    user.membership_info = new_membership_info;

    // Update user in DynamoDB using the generated update method
    User::updater(user.pk.clone(), user.sk.clone())
        .with_membership_info(user.membership_info.clone())
        .execute(&ddb)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user membership: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;

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
