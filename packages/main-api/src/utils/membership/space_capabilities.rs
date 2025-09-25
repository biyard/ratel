use crate::Error2 as Error;
use crate::utils::aws::dynamo::DynamoClient;
use crate::utils::users_dynamo::extract_user;
use bdk::prelude::by_axum::auth::Authorization;

type Result<T> = std::result::Result<T, Error>;

/// Check if a user has the capability to create a space with the given booster type
///
/// # Arguments
/// * `dynamo_client` - DynamoDB client
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
/// check_space_creation_capability(&dynamo_client, auth, 10).await?; // Check for 10x booster
///
/// // Create the space...
///
/// // After successful creation, consume the quota
/// consume_space_creation_quota(&dynamo_client, auth, 10).await?;
/// ```
pub async fn check_space_creation_capability(
    dynamo_client: &DynamoClient,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<()> {
    // If booster_type is 0 (no boost), always allow creation
    if booster_type == 0 {
        return Ok(());
    }

    // Get the user from auth
    let user = extract_user(dynamo_client, auth).await?;

    // Check if membership is active
    if !user.membership_info.is_active() {
        return Err(Error::Unknown("Membership has expired".to_string()));
    }

    // Check if user can create space with this booster type
    if user.membership_info.can_create_space(booster_type) {
        Ok(())
    } else {
        match user.membership_info.membership_type {
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
/// * `dynamo_client` - DynamoDB client
/// * `auth` - User authorization
/// * `booster_type` - Booster type as integer (0=no boost, 2=2x, 10=10x, 100=100x, etc.)
///
/// # Returns
/// * `Ok(())` - Quota consumed successfully
/// * `Err(Error)` - Failed to consume quota
pub async fn consume_space_creation_quota(
    dynamo_client: &DynamoClient,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<()> {
    // If booster_type is 0 (no boost), no quota to consume
    if booster_type == 0 {
        return Ok(());
    }

    let mut user = extract_user(dynamo_client, auth).await?;

    // Consume the quota
    if user.membership_info.consume_space_quota(booster_type) {
        // Update the user in DynamoDB
        update_user_membership_info(dynamo_client, &user).await?;
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
/// * `dynamo_client` - DynamoDB client
/// * `auth` - User authorization
/// * `booster_type` - Booster type as integer (0=no boost, 1=1x, 2=2x, 10=10x, 100=100x, etc.)
///
/// # Returns
/// * Remaining quota count
pub async fn get_remaining_quota(
    dynamo_client: &DynamoClient,
    auth: Option<Authorization>,
    booster_type: u32,
) -> Result<i32> {
    // If booster_type is 0 (no boost), return unlimited (represented as -1)
    if booster_type == 0 {
        return Ok(-1);
    }

    let user = extract_user(dynamo_client, auth).await?;

    let remaining = user
        .membership_info
        .space_capabilities
        .get(&booster_type)
        .copied()
        .unwrap_or(0);

    Ok(remaining)
}

/// Helper function to update user membership info in DynamoDB
async fn update_user_membership_info(
    dynamo_client: &DynamoClient,
    user: &crate::models::dynamo_tables::main::user::user::User,
) -> Result<()> {
    use aws_sdk_dynamodb::types::AttributeValue;
    use serde_dynamo::to_item;
    use std::collections::HashMap;

    // Serialize the updated user
    let item: HashMap<String, AttributeValue> = to_item(user).map_err(|e| Error::SerdeDynamo(e))?;

    dynamo_client
        .put_item(item)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to update user: {}", e)))?;
    Ok(())
}
