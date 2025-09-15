use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, UserV2, aide,
    by_axum::axum::{
        Json,
        extract::Query,
    },
    DynamoUser,
};
use tracing::{debug, error};
use crate::{
    utils::aws::dynamo::DynamoClient,
    config,
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct UserQuery {
    #[schemars(description = "Username")]
    pub username: Option<String>,
    #[schemars(description = "Phone Number")]
    #[serde(rename = "phone-number")]
    pub phone_number: Option<String>,
    #[schemars(description = "Email")]
    pub email: Option<String>,
}

pub async fn find_user_handler(
    Query(UserQuery {
        username,
        phone_number,
        email,
    }): Query<UserQuery>,
) -> Result<Json<UserV2>> {
    let conf = config::get();
    let dynamo_client = DynamoClient::new(&conf.dual_write.table_name);

    if let Some(username) = username {
        debug!("Finding user by username: {:?}", username);

        let user = find_user_by_username(&dynamo_client, &username).await?;
        return Ok(Json(user));
    }

    if let Some(phone_number) = phone_number {
        debug!("Finding user by phone number: {:?}", phone_number);

        let user = find_user_by_phone_number(&dynamo_client, &phone_number).await?;
        return Ok(Json(user));
    }

    if let Some(email) = email {
        debug!("Finding user by email: {:?}", email);

        let cleaned_email = email.replace(' ', "+");
        let user = find_user_by_email(&dynamo_client, &cleaned_email).await?;
        return Ok(Json(user));
    }

    Err(Error::InvalidUserQuery(
        "username, phone-number, or email query is required".to_string(),
    ))
}

/// Find user by username using GSI1
async fn find_user_by_username(client: &DynamoClient, username: &str) -> Result<UserV2> {
    let gsi1_pk = format!("USERNAME#{}", username);
    
    let result = client.client
        .query()
        .table_name(&client.table_name)
        .index_name("GSI1")
        .key_condition_expression("GSI1_PK = :pk")
        .expression_attribute_values(":pk", aws_sdk_dynamodb::types::AttributeValue::S(gsi1_pk))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to query user by username: {}", e);
            Error::NotFound
        })?;

    if let Some(items) = result.items {
        if let Some(item) = items.into_iter().next() {
            let dynamo_user = DynamoUser::from_dynamo_item(item)?;
            return Ok(convert_dynamo_user_to_userv2(dynamo_user));
        }
    }

    Err(Error::NotFound)
}

/// Find user by phone number - we'll need to scan for this since it's not indexed
async fn find_user_by_phone_number(client: &DynamoClient, phone_number: &str) -> Result<UserV2> {
    // Note: This is not efficient for large datasets. In production, consider adding a GSI for phone_number
    let result = client.client
        .scan()
        .table_name(&client.table_name)
        .filter_expression("begins_with(PK, :pk_prefix) AND telegram_id = :phone")
        .expression_attribute_values(":pk_prefix", aws_sdk_dynamodb::types::AttributeValue::S("USER#".to_string()))
        .expression_attribute_values(":phone", aws_sdk_dynamodb::types::AttributeValue::S(phone_number.to_string()))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to scan user by phone number: {}", e);
            Error::NotFound
        })?;

    if let Some(items) = result.items {
        if let Some(item) = items.into_iter().next() {
            let dynamo_user = DynamoUser::from_dynamo_item(item)?;
            return Ok(convert_dynamo_user_to_userv2(dynamo_user));
        }
    }

    Err(Error::NotFound)
}

/// Find user by email - we'll need to scan for this since it's not indexed
async fn find_user_by_email(_client: &DynamoClient, _email: &str) -> Result<UserV2> {
    // Note: Email search is not implemented in the current DynamoUser schema
    // In production, you would need to:
    // 1. Add email field to DynamoUser
    // 2. Create a GSI for email lookup
    // 3. Implement proper email-based querying
    
    error!("Email search not implemented for DynamoDB schema");
    Err(Error::NotFound)
}

/// Convert DynamoUser to UserV2
fn convert_dynamo_user_to_userv2(dynamo_user: DynamoUser) -> UserV2 {
    let telegram_id_clone = dynamo_user.telegram_id.clone();
    UserV2 {
        id: dynamo_user.user_id,
        username: dynamo_user.username,
        created_at: dynamo_user.created_at,
        evm_address: dynamo_user.evm_address.unwrap_or_default(),
        // Set default values for fields not available in DynamoUser
        updated_at: dynamo_user.created_at, // Use created_at as fallback
        nickname: String::new(), // Default empty string
        principal: String::new(), // Default empty string
        email: String::new(), // Not available in current DynamoUser schema - use empty string
        profile_url: String::new(), // Default empty string
        term_agreed: true, // Default assumption
        informed_agreed: true, // Default assumption
        user_type: dto::UserType::Individual,
        parent_id: None,
        password: String::new(),
        membership: dto::Membership::Free,
        theme: None,
        referral_code: String::new(),
        phone_number: dynamo_user.telegram_id,
        telegram_id: telegram_id_clone.and_then(|s| s.parse().ok()),
        followers_count: 0,
        followings_count: 0,
        groups: Vec::new(),
        teams: Vec::new(),
        html_contents: String::new(),
        followers: Vec::new(),
        followings: Vec::new(),
        badges: Vec::new(),
        bookmarked_feeds: Vec::new(),
        points: 0,
    }
}
