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

    // Get the target user
    let user_pk = format!("USER#{}", user_id);
    let user = User::get(&ddb, user_pk, Some("USER".to_string()))
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?
        .ok_or(Error::InvalidUser)?;

    // Update user's membership to admin using the generated update method
    User::updater(user.pk.clone(), user.sk.clone())
        .with_membership_info(MembershipInfo::from_membership(Membership::Admin))
        .execute(&ddb)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user to admin: {:?}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;

    Ok(Json(SetMembershipResponse {
        success: true,
        message: format!("Successfully promoted user {} to admin", user_id),
        user_id,
        new_membership: Membership::Admin,
    }))
}
