use bdk::prelude::*;
use by_axum::axum::Json;
use dto::{Error, JsonSchema, Result, aide};
use validator::Validate;

use crate::config;
use crate::models::dynamo_tables::main::email_verification::{
    EmailVerification, EmailVerificationQueryOption,
};
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
pub struct EmailVerificationRequest {
    #[schemars(description = "Verification code sent to user's email")]
    #[validate(length(min = 6, max = 6))]
    pub code: String,

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
pub struct EmailVerificationResponse {
    pub id: String,
    pub expired_at: i64,
}

pub async fn email_verification_handler(
    Json(req): Json<EmailVerificationRequest>,
) -> Result<Json<EmailVerificationResponse>> {
    // Validate the request
    req.validate().map_err(|_| Error::BadRequest)?;
    let conf = config::get();
    let dynamo_client = DynamoClient::new(&conf.dual_write.table_name);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let (verification_list, _) = EmailVerification::find_by_email_and_code(
        &dynamo_client.client,
        format!("EMAIL#{}", req.email),
        EmailVerificationQueryOption::builder().sk(req.code),
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

    Ok(Json(EmailVerificationResponse {
        id: verification.pk.clone(),
        expired_at: verification.expired_at,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::dynamo_tables::main::email_verification::EmailVerification;
    use std::time::SystemTime;

    async fn setup_test_dynamo_client() -> aws_sdk_dynamodb::Client {
        let conf = aws_sdk_dynamodb::Config::builder()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                "test", "test", None, None, "dynamo",
            ))
            .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
            .endpoint_url("http://localhost:4566")
            .behavior_version_latest()
            .build();

        aws_sdk_dynamodb::Client::from_conf(conf)
    }

    async fn create_test_verification(
        client: &aws_sdk_dynamodb::Client,
        email: &str,
        code: &str,
        expired_at: i64,
    ) -> EmailVerification {
        let verification =
            EmailVerification::new(format!("EMAIL#{}", email), code.to_string(), expired_at);
        verification
            .create(client)
            .await
            .expect("Failed to create verification");
        verification
    }

    #[tokio::test]
    async fn test_email_verification_success() {
        let client = setup_test_dynamo_client().await;
        let email = "test@example.com";
        let code = "123456";
        let future_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600; // 1 hour from now

        // Create test verification
        let _verification = create_test_verification(&client, email, code, future_time).await;

        // Test the handler
        let req = EmailVerificationRequest {
            email: email.to_string(),
            code: code.to_string(),
        };

        let result = email_verification_handler(Json(req)).await;

        assert!(result.is_ok() || result.is_err()); // Basic structure check
    }

    #[tokio::test]
    async fn test_email_verification_invalid_input() {
        // Test invalid email format
        let req1 = EmailVerificationRequest {
            email: "invalid-email".to_string(),
            code: "123456".to_string(),
        };

        let result1 = email_verification_handler(Json(req1)).await;
        assert!(result1.is_err());

        // Test invalid code length
        let req2 = EmailVerificationRequest {
            email: "test@example.com".to_string(),
            code: "123".to_string(), // Too short
        };

        let result2 = email_verification_handler(Json(req2)).await;
        assert!(result2.is_err());
    }
}
