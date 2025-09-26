use crate::Error2 as Error;
use crate::types::Membership;
use crate::utils::dynamo_extractor::extract_user;
use crate::models::dynamo_tables::main::user::UserMembership;
use bdk::prelude::by_axum::auth::Authorization;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Error>;

/// Check if the authenticated user has admin permissions using shared DDB client
pub async fn check_admin_permission_shared_ddb(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<()> {
    let user = extract_user(ddb, auth).await?;
    let user_pk_str = user.pk.to_string();
    
    let membership = UserMembership::get(ddb, &user_pk_str, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            let user_id = user_pk_str.strip_prefix("USER#").unwrap_or(&user_pk_str);
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });
    
    match membership.membership_type {
        Membership::Admin => Ok(()),
        _ => Err(Error::Unauthorized(
            "User does not have admin permissions".to_string(),
        )),
    }
}

/// Check if the authenticated user has admin permissions
pub async fn check_admin_permission(
    ddb: &aws_sdk_dynamodb::Client,
    auth: Option<Authorization>,
) -> Result<()> {
    let user = extract_user(ddb, auth).await?;
    let user_pk_str = user.pk.to_string();
    
    let membership = UserMembership::get(ddb, &user_pk_str, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            let user_id = user_pk_str.strip_prefix("USER#").unwrap_or(&user_pk_str);
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });
    
    match membership.membership_type {
        Membership::Admin => Ok(()),
        _ => Err(Error::Unauthorized(
            "User does not have admin permissions".to_string(),
        )),
    }
}

/// Check if a user has admin permissions by user ID
pub async fn check_user_is_admin(
    ddb: &aws_sdk_dynamodb::Client,
    user_id: &str,
) -> Result<bool> {
    let user_pk = format!("USER#{}", user_id);
    let membership = UserMembership::get(ddb, &user_pk, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });
    
    Ok(membership.membership_type == Membership::Admin)
}
