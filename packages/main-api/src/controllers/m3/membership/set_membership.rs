use std::collections::HashMap;

use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, State},
        },
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Error2,
    models::{dynamo_tables::main::user::UserMembership, user::User},
    types::Membership,
    utils::admin::check_admin_permission,
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
    State(AppState { dynamo, .. }): State<AppState>,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(payload): Json<SetMembershipRequest>,
) -> Result<Json<SetMembershipResponse>, Error2> {
    // Check admin permission
    check_admin_permission(&dynamo.client, auth).await?;

    // Get the target user (to verify they exist)
    let user_pk = format!("USER#{}", user_id);
    let _user = User::get(
        &dynamo.client,
        &user_pk,
        Some(crate::types::EntityType::User),
    )
    .await?
    .ok_or(Error2::NotFound("User not found".into()))?;

    // Get or create user membership using the builder pattern
    let mut membership = match UserMembership::get(&dynamo.client, &user_pk, Some("MEMBERSHIP"))
        .await?
    {
        Some(existing_membership) => existing_membership,
        None => {
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
    membership.create(&dynamo.client).await?;

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
