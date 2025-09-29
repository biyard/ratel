use bdk::prelude::*;
use by_axum::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error, aide::OperationIo)]
pub enum Error {
    #[error("DynamoDB error: {0}")]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),
    #[error("AWS Ses error: {0}")]
    SesServiceError(#[from] crate::utils::aws::ses::SesServiceError),
    #[error("SerdeDynamo error: {0}")]
    SerdeDynamo(#[from] serde_dynamo::Error),
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("Session error")]
    SessionError(#[from] tower_sessions::session::Error),

    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Item already exists: {0}")]
    AlreadyExists(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("Aws chime error: {0}")]
    AwsChimeError(String),
    #[error("Other error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    // /v3/auth endpoints
    #[error("Exceeded maximum attempt for email verification")]
    ExceededAttemptEmailVerification,
    #[error("Failed to send email via AWS SES: {0}")]
    AwsSesSendEmailException(String),
    #[error("Verification code not found or expired")]
    NotFoundVerificationCode,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let msg = self.to_string();

        match self {
            Error::Unauthorized(_) => (StatusCode::UNAUTHORIZED, msg).into_response(),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, msg).into_response(),
            Error::AlreadyExists(_) | Error::Duplicate(_) => {
                (StatusCode::CONFLICT, msg).into_response()
            }
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Error::InternalServerError(_)
            | Error::DynamoDbError(_)
            | Error::SerdeDynamo(_)
            | Error::SerdeJson(_)
            | Error::SesServiceError(_)
            | Error::SessionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            _ => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}
