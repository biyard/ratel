use std::collections::HashMap;

use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension,
            extract::{Json, Path, State},
        },
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Error2, models::dynamo_tables::main::user::UserMembership, types::Membership,
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
    State(AppState { dynamo, .. }): State<AppState>,
    Path(user_id): Path<String>,
    Extension(auth): Extension<Option<Authorization>>,
) -> Result<Json<UserInfo>, Error2> {
    // Check admin permissions
    check_admin_permission(&dynamo.client, auth).await?;

    // Get the target user
    let user_pk = format!("USER#{}", user_id);
    let user = crate::models::user::User::get(
        &dynamo.client,
        &user_pk,
        Some(crate::types::EntityType::User),
    )
    .await?
    .ok_or(Error2::NotFound("User not found".into()))?;

    // Get user membership
    let membership = UserMembership::get(&dynamo.client, &user_pk, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // If no membership found, create default Free membership using builder pattern
            UserMembership::builder(user_id.clone()).with_free().build()
        });

    Ok(Json(UserInfo {
        user_id,
        email: user.email,
        display_name: user.display_name,
        membership_type: membership.membership_type,
        membership_active: membership.is_active(),
    }))
}
