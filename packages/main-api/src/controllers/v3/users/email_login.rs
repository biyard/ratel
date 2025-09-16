use bdk::prelude::*;
use by_axum::axum::{Extension, Json};
use dto::by_axum::auth::UserSession;
use dto::{Error, JsonSchema, Result, aide};
use tower_sessions::Session;
use validator::Validate;

use crate::config;
use crate::models::dynamo_tables::main::user::User as DynamoUser;
use crate::utils::aws::dynamo::DynamoClient;

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
pub struct UserV3LoginRequest {
    #[schemars(description = "User's email address")]
    #[validate(email)]
    pub email: String,

    #[schemars(description = "User's password")]
    #[validate(length(min = 1))]
    pub password: String,
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
pub struct UserV3LoginResponse {
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

pub async fn v3_login_with_password_handler(
    Extension(session): Extension<Session>,
    Json(req): Json<UserV3LoginRequest>,
) -> Result<Json<UserV3LoginResponse>> {
    // Validate the request
    req.validate().map_err(|_| Error::BadRequest)?;
    let conf = config::get();
    let dynamo_client = DynamoClient::new(&conf.dual_write.table_name);

    // Find user by email using DynamoDB GSI
    let (users, _) = DynamoUser::find_by_email(
        &dynamo_client.client,
        &req.email,
        crate::models::dynamo_tables::main::user::UserQueryOption::builder(),
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to find user by email: {:?}", e);
        Error::NotFound
    })?;

    let dynamo_user = users.first().ok_or(Error::NotFound)?;

    // Verify password (in production, this should use proper password hashing)
    if dynamo_user.password != req.password {
        tracing::error!("Password mismatch for user: {}", req.email);
        return Err(Error::Unauthorized);
    }

    // Get the user's principal from UserPrincipal table
    let principals = crate::models::dynamo_tables::main::user::UserMetadata::query(
        &dynamo_client.client,
        dynamo_user.pk.clone(),
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to find user principal: {:?}", e);
        Error::NotFound
    })?;

    let user_principal = principals
        .into_iter()
        .find(|p| {
            matches!(
                p,
                crate::models::dynamo_tables::main::user::UserMetadata::UserPrincipal(_)
            )
        })
        .and_then(|p| match p {
            crate::models::dynamo_tables::main::user::UserMetadata::UserPrincipal(up) => {
                Some(up.principal)
            }
            _ => None,
        })
        .ok_or(Error::NotFound)?;

    // Create user session (similar to V1)
    let user_session = UserSession {
        user_id: 0, // DynamoDB doesn't use integer IDs, using 0 as placeholder
        principal: user_principal.clone(),
        email: dynamo_user.email.clone(),
    };

    // Store session
    session
        .insert("user_session", &user_session)
        .await
        .map_err(|e| {
            tracing::error!("Failed to store session: {:?}", e);
            Error::DatabaseException(e.to_string())
        })?;

    // Return the response with basic user information
    let response = UserV3LoginResponse {
        id: dynamo_user.pk.clone(),
        nickname: dynamo_user.nickname.clone(),
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

    fn create_valid_login_request() -> UserV3LoginRequest {
        UserV3LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }
    }

    #[tokio::test]
    async fn test_login_validation() {
        // Test that validation catches basic issues
        let invalid_requests = vec![
            UserV3LoginRequest {
                email: "not-an-email".to_string(),
                password: "password123".to_string(),
            },
            UserV3LoginRequest {
                email: "test@example.com".to_string(),
                password: "".to_string(),
            },
        ];

        for req in invalid_requests {
            // Only test validation, not the full handler
            assert!(
                req.validate().is_err(),
                "Expected validation error for invalid request"
            );
        }
    }

    #[tokio::test]
    async fn test_valid_request_structure() {
        let req = create_valid_login_request();

        // Test that valid requests pass validation
        assert!(
            req.validate().is_ok(),
            "Valid request should pass validation"
        );
    }
}
