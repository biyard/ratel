use crate::Error2 as Error;
use crate::utils::dynamo_extractor::extract_user;
use crate::models::dynamo_tables::main::user::UserMembership;
use bdk::prelude::by_axum::auth::Authorization;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Error>;

/// Check if a user has the capability to create a space with the given booster type
///
/// # Arguments
/// * `ddb` - DynamoDB client
/// * `auth` - User authorization
/// * `booster_type` - Booster type as integer (0=no boost, 2=2x, 10=10x, 100=100x, 1000=1000x, etc.)
///
/// # Returns
/// * `Ok(())` - User can create the space
/// * `Err(Error)` - User cannot create the space
///
/// # Example Usage
/// ```rust
/// // In a space creation endpoint
/// check_space_creation_capability(&ddb, auth, 10).await?; // Check for 10x booster
///
/// // Create the space...
///
/// // After successful creation, consume the quota
/// consume_space_creation_quota(&ddb, auth, 10).await?;
/// ```
pub async fn check_space_creation_capability(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<()> {
    // If booster_type is 0 (no boost), always allow creation
    if booster_type == 0 {
        return Ok(());
    }

    // Get the user from auth
    let user = extract_user(ddb, auth).await?;
    let user_pk_str = user.pk.to_string();
    
    // Get user membership
    let membership = UserMembership::get(ddb, &user_pk_str, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            let user_id = user_pk_str.strip_prefix("USER#").unwrap_or(&user_pk_str);
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });

    // Check if membership is active
    if !membership.is_active() {
        return Err(Error::Unknown("Membership has expired".to_string()));
    }

    // Check if user can create space with this booster type
    if membership.can_create_space(booster_type) {
        Ok(())
    } else {
        match membership.membership_type {
            crate::types::Membership::Free => Err(Error::Unknown(
                "Free tier users can only create basic (no boost) spaces".to_string(),
            )),
            _ => Err(Error::Unknown(format!(
                "You have reached your limit for {}x booster spaces this month",
                booster_type
            ))),
        }
    }
}

/// Consume a space creation quota for the given booster type
/// This should be called after a space is successfully created
///
/// # Arguments
/// * `ddb` - DynamoDB client
/// * `auth` - User authorization
/// * `booster_type` - Booster type as integer (0=no boost, 2=2x, 10=10x, 100=100x, etc.)
///
/// # Returns
/// * `Ok(())` - Quota consumed successfully
/// * `Err(Error)` - Failed to consume quota
pub async fn consume_space_creation_quota(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<()> {
    // If booster_type is 0 (no boost), no quota to consume
    if booster_type == 0 {
        return Ok(());
    }

    let user = extract_user(ddb, auth).await?;
    let user_pk_str = user.pk.to_string();
    
    // Get user membership
    let mut membership = UserMembership::get(ddb, &user_pk_str, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            let user_id = user_pk_str.strip_prefix("USER#").unwrap_or(&user_pk_str);
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });

    // Consume the quota
    if membership.consume_space_quota(booster_type) {
        // Update the membership in DynamoDB
        membership.create(ddb).await?;
        Ok(())
    } else {
        Err(Error::Unknown(
            "Failed to consume space quota - no quota available".to_string(),
        ))
    }
}

/// Get remaining quota for a booster type
///
/// # Arguments
/// * `ddb` - DynamoDB client
/// * `auth` - User authorization
/// * `booster_type` - Booster type as integer (0=no boost, 1=1x, 2=2x, 10=10x, 100=100x, etc.)
///
/// # Returns
/// * Remaining quota count
pub async fn get_remaining_quota(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<i32> {
    // If booster_type is 0 (no boost), return unlimited (represented as -1)
    if booster_type == 0 {
        return Ok(-1);
    }

    let user = extract_user(ddb, auth).await?;
    let user_pk_str = user.pk.to_string();
    
    // Get user membership
    let membership = UserMembership::get(ddb, &user_pk_str, Some("MEMBERSHIP"))
        .await?
        .unwrap_or_else(|| {
            // Default to Free membership if none found
            let user_id = user_pk_str.strip_prefix("USER#").unwrap_or(&user_pk_str);
            UserMembership::builder(user_id.to_string())
                .with_free()
                .build()
        });

    let capabilities = membership.get_space_capabilities();
    let remaining = capabilities
        .get(&booster_type)
        .copied()
        .unwrap_or(0);

    Ok(remaining)
}
