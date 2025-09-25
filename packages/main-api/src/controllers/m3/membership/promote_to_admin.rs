use std::collections::HashMap;

use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::{
        auth::Authorization,
        axum::{extract::{Path, State}, Extension, Json},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Error2,
    models::{
        dynamo_tables::main::user::UserMembership,
        user::User,
    },
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

/// Admin endpoint to promote a user to admin
/// POST /m3/admin/users/{user_id}/promote
pub async fn promote_user_to_admin(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
) -> Result<Json<SetMembershipResponse>, Error2> {
    // Check admin permission
    check_admin_permission(&dynamo.client, auth).await?;

    // Get the target user (to verify they exist)
    let user_pk = format!("USER#{}", user_id);
    let _user = User::get(&dynamo.client, &user_pk, Some(crate::types::EntityType::User))
        .await?
        .ok_or(Error2::NotFound("User not found".into()))?;

    // Get or create user membership using the builder pattern
    let mut membership = match UserMembership::get(&dynamo.client, &user_pk, Some("MEMBERSHIP")).await? {
        Some(membership) => membership,
        None => {
            // Create new admin membership using builder pattern
            UserMembership::builder(user_id.clone()).with_admin().build()
        }
    };

    // Update to admin membership
    membership.membership_type = Membership::Admin;
    membership.updated_at = crate::utils::time::get_now_timestamp_millis();

    // Save the updated membership
    membership.create(&dynamo.client).await?;

    Ok(Json(SetMembershipResponse {
        success: true,
        message: format!("Successfully promoted user {} to admin", user_id),
        user_id,
        new_membership: Membership::Admin,
    }))
}
