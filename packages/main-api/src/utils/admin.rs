use crate::Error2 as Error;
use crate::types::Membership;
use crate::utils::users_dynamo::extract_user;
use bdk::prelude::by_axum::auth::Authorization;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Error>;

/// Check if the authenticated user has admin permissions using shared DDB client
pub async fn check_admin_permission_shared_ddb(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<()> {
    let user = extract_user(ddb, auth).await?;

    match user.membership_info.membership_type {
        Membership::Admin => Ok(()),
        _ => Err(Error::Unauthorized(
            "User does not have admin permissions".to_string(),
        )),
    }
}

/// Check if the authenticated user has admin permissions
pub async fn check_admin_permission(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<()> {
    let user = extract_user(ddb, auth).await?;

    match user.membership_info.membership_type {
        Membership::Admin => Ok(()),
        _ => Err(Error::Unauthorized(
            "User does not have admin permissions".to_string(),
        )),
    }
}

/// Check if a user has admin permissions by user ID
pub async fn check_user_is_admin(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    user_id: &str,
) -> Result<bool> {
    // Get user by ID using the users_dynamo utility
    use crate::utils::users_dynamo::get_user_by_pk;

    if let Some(user) = get_user_by_pk(ddb, &format!("USER#{}", user_id)).await? {
        Ok(matches!(
            user.membership_info.membership_type,
            Membership::Admin
        ))
    } else {
        Ok(false)
    }
}
