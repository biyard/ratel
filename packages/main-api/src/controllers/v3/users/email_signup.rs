use std::time::SystemTime;

use bdk::prelude::*;
use by_axum::auth::Authorization;
use by_axum::axum::{Extension, Json};
use dto::{Error, JsonSchema, Result, aide};
use validator::Validate;

use crate::models::dynamo_tables::main::email::*;
use crate::models::dynamo_tables::main::user::*;
use crate::types::UserType;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
    Validate,
)]
pub struct UserV3SignupRequestWithEmail {
    #[schemars(description = "User's display name")]
    #[validate(length(min = 1, max = 50))]
    pub nickname: String,

    #[schemars(description = "User's email address")]
    #[validate(email)]
    pub email: String,

    #[schemars(description = "User's profile image URL")]
    pub profile_url: String,

    #[schemars(description = "Whether user agreed to terms of service")]
    pub term_agreed: bool,

    #[schemars(description = "Whether user agreed to privacy policy")]
    pub informed_agreed: bool,

    #[schemars(description = "User's unique username")]
    #[validate(length(min = 3, max = 30))]
    pub username: String,

    #[schemars(description = "User's password (should be hashed)")]
    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[schemars(description = "Email verification code")]
    #[validate(length(min = 6, max = 6))]
    pub verification_code: String,
}

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
pub struct UserV3SignupResponse {
    #[schemars(description = "User's unique identifier")]
    pub id: String,

    #[schemars(description = "User's display name")]
    pub nickname: String,

    #[schemars(description = "User's email address")]
    pub email: String,

    #[schemars(description = "User's unique username")]
    pub username: String,

    #[schemars(description = "User's profile image URL")]
    pub profile_url: String,

    #[schemars(description = "User creation timestamp")]
    pub created_at: i64,
}

pub async fn v3_email_signup_handler(
    by_axum::axum::extract::State(ddb): by_axum::axum::extract::State<
        std::sync::Arc<aws_sdk_dynamodb::Client>,
    >,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<UserV3SignupRequestWithEmail>,
) -> Result<Json<UserV3SignupResponse>> {
    req.validate().map_err(|_| Error::BadRequest)?;

    if req.term_agreed == false {
        return Err(Error::BadRequest);
    }

    // Principal extraction removed for now
    let principal = match auth {
        Some(Authorization::Session(session)) => session.principal.clone(),
        Some(Authorization::UserSig(sig)) => sig.principal().map_err(|e| {
            tracing::error!("failed to get principal: {:?}", e);
            Error::Unauthorized
        })?,
        Some(Authorization::Bearer { claims }) => claims.sub.clone(),
        _ => uuid::Uuid::new_v4().to_string(),
    };

    // Verify the email verification code
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let (verification_list, _) = EmailVerification::find_by_email_and_code(
        &ddb,
        format!("EMAIL#{}", req.email),
        EmailVerificationQueryOption::builder().sk(req.verification_code),
    )
    .await
    .map_err(|e| {
        tracing::error!("Verification Error: {:?}", e);
        Error::InvalidVerificationCode
    })?;

    let verification = verification_list
        .first()
        .ok_or(Error::InvalidVerificationCode)?;

    if verification.expired_at <= now {
        return Err(Error::InvalidVerificationCode);
    }

    // Create new DynamoDB user
    let dynamo_user = User::new(
        req.nickname,
        req.email,
        req.profile_url,
        req.term_agreed,
        req.informed_agreed,
        UserType::Individual,
        None,
        req.username,
        req.password,
    );

    // Save user to DynamoDB
    dynamo_user.create(&ddb).await.map_err(|e| {
        tracing::error!("DynamoDB User Creation Error: {:?}", e);
        Error::DynamoDbError(e.to_string())
    })?;

    // Create UserPrincipal mapping
    let user_principal = UserPrincipal::new(dynamo_user.pk.clone(), principal.clone());
    user_principal.create(&ddb).await.map_err(|e| {
        tracing::error!("DynamoDB UserPrincipal Creation Error: {:?}", e);
        Error::DynamoDbError(e.to_string())
    })?;

    // Return the response with basic user information
    let response = UserV3SignupResponse {
        id: dynamo_user.pk.to_string(),
        nickname: dynamo_user.display_name.clone(),
        email: dynamo_user.email.clone(),
        username: dynamo_user.username.clone(),
        profile_url: dynamo_user.profile_url.clone(),
        created_at: dynamo_user.created_at,
    };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_signup_request() -> UserV3SignupRequestWithEmail {
        UserV3SignupRequestWithEmail {
            nickname: "TestUser".to_string(),
            email: "test@example.com".to_string(),
            profile_url: "https://example.com/profile.jpg".to_string(),
            term_agreed: true,
            informed_agreed: true,
            username: "testuser123".to_string(),
            password: "password123".to_string(),
            verification_code: "123456".to_string(),
        }
    }

    // FIXME: use oneshot instead of direct call
    // #[tokio::test]
    // async fn test_signup_success() {
    //     let req = create_valid_signup_request();
    //     let auth = None;

    //     // Note: This test would require proper mocking of:
    //     // - DynamoClient
    //     // - EmailVerification lookup
    //     // - DynamoDB user creation
    //     // - UserPrincipal creation
    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;

    //     // For now, we test the structure
    //     assert!(result.is_ok() || result.is_err());
    // }

    // #[tokio::test]
    // async fn test_signup_terms_not_agreed() {
    //     let mut req = create_valid_signup_request();
    //     req.term_agreed = false;
    //     let auth = None;

    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;
    //     assert!(result.is_err());
    // }

    // #[tokio::test]
    // async fn test_signup_invalid_email() {
    //     let mut req = create_valid_signup_request();
    //     req.email = "invalid-email".to_string();
    //     let auth = None;

    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;
    //     assert!(result.is_err());
    // }

    // #[tokio::test]
    // async fn test_signup_invalid_username() {
    //     let mut req = create_valid_signup_request();
    //     req.username = "ab".to_string(); // Too short
    //     let auth = None;

    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;
    //     assert!(result.is_err());
    // }

    // #[tokio::test]
    // async fn test_signup_invalid_password() {
    //     let mut req = create_valid_signup_request();
    //     req.password = "1234567".to_string(); // Too short
    //     let auth = None;

    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;
    //     assert!(result.is_err());
    // }

    // #[tokio::test]
    // async fn test_signup_invalid_verification_code() {
    //     let mut req = create_valid_signup_request();
    //     req.verification_code = "123".to_string(); // Too short
    //     let auth = None;

    //     let result = v3_email_signup_handler(Extension(auth), Json(req)).await;
    //     assert!(result.is_err());
    // }
}
