use std::time::SystemTime;

use aws_sdk_sesv2::types::Content;
use bdk::prelude::*;
use by_axum::axum::Json;
use dto::{Error, JsonSchema, Result, aide};
use rand::distr::Alphanumeric;
use validator::Validate;

use crate::models::dynamo_tables::main::email::*;
use crate::utils::email::send_email;

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
pub struct RequestVerificationCodeRequest {
    #[schemars(description = "User's email address")]
    #[validate(email)]
    pub email: String,
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
pub struct RequestVerificationCodeResponse {
    pub id: String,
    pub expired_at: i64,
}

pub async fn request_verification_code_handler(
    by_axum::axum::extract::State(ddb): by_axum::axum::extract::State<
        std::sync::Arc<aws_sdk_dynamodb::Client>,
    >,
    Json(req): Json<RequestVerificationCodeRequest>,
) -> Result<Json<RequestVerificationCodeResponse>> {
    req.validate().map_err(|_| Error::BadRequest)?;

    use rand::{Rng, rng};

    let code: String = rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    send_email(
        req.email.clone(),
        Content::builder()
            .data("Please finish to sign up within 30 minutes with your verification code")
            .build()
            .unwrap(),
        Content::builder()
            .data(format!("Verification code: {:?}", code))
            .build()
            .unwrap(),
    )
    .await
    .map_err(|e| {
        tracing::error!("Email Send Error: {:?}", e);
        Error::SESServiceError(e.to_string())
    })?;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let verification_expiration = 60 * 30; // 30 minutes
    let expired_at = now + verification_expiration;

    let email_verification = EmailVerification::new(req.email, code, expired_at);

    email_verification.create(&ddb).await.map_err(|e| {
        tracing::error!("DynamoDB Error: {:?}", e);
        Error::DynamoDbError(e.to_string())
    })?;

    Ok(Json(RequestVerificationCodeResponse {
        id: email_verification.pk.to_string(),
        expired_at,
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // FIXME: use oneshot
//     // #[tokio::test]
//     // async fn test_request_verification_code_success() {
//     //     let req = RequestVerificationCodeRequest {
//     //         email: "test@example.com".to_string(),
//     //     };

//     //     // Note: This test would require proper mocking of:
//     //     // - DynamoClient::new()
//     //     // - send_email function
//     //     // - EmailVerification::create
//     //     let result = request_verification_code_handler(Json(req)).await;

//     //     // For now, we test the structure
//     //     assert!(result.is_ok() || result.is_err());
//     // }

//     // #[tokio::test]
//     // async fn test_request_verification_code_invalid_email() {
//     //     let req = RequestVerificationCodeRequest {
//     //         email: "invalid-email".to_string(),
//     //     };

//     //     let result = request_verification_code_handler(Json(req)).await;
//     //     assert!(result.is_err());
//     // }

//     // #[tokio::test]
//     // async fn test_request_verification_code_empty_email() {
//     //     let req = RequestVerificationCodeRequest {
//     //         email: "".to_string(),
//     //     };

//     //     let result = request_verification_code_handler(Json(req)).await;
//     //     assert!(result.is_err());
//     // }
// }
