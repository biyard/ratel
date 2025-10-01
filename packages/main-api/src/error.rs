use bdk::prelude::*;
use thiserror::Error;

#[derive(Debug, Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("DynamoDB error: {0}")]
    #[rest_error(code = 100)]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),
    #[error("AWS Ses error: {0}")]
    #[rest_error(status = 500)]
    SesServiceError(#[from] crate::utils::aws::ses::SesServiceError),
    #[error("SerdeDynamo error: {0}")]
    #[rest_error(status = 500)]
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
    #[rest_error(status = 404)]
    NotFound(String),
    #[error("Item already exists: {0}")]
    AlreadyExists(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    #[rest_error(status = 401)]
    Unauthorized(String),
    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    #[error("Aws chime error: {0}")]
    AwsChimeError(String),
    #[error("Other error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    // Authorization errors 400 ~
    #[error("No session found")]
    #[rest_error(status = 401, code = 400)]
    NoSessionFound,
    #[error("No user found in session")]
    #[rest_error(status = 401, code = 401)]
    NoUserFound,

    // /v3/auth endpoints 1000 ~
    #[error("Exceeded maximum attempt for email verification")]
    #[rest_error(code = 1000)]
    ExceededAttemptEmailVerification,
    #[error("Failed to send email via AWS SES: {0}")]
    AwsSesSendEmailException(String),
    #[error("Verification code not found or expired")]
    NotFoundVerificationCode,
    #[error("Verification code has expired")]
    ExpiredVerification,
    #[error("Invalid verification code")]
    InvalidVerificationCode,

    // /v3/posts endpoints 2000 ~
    #[error("Post visibility is incorrectly configured: {0}")]
    #[rest_error(code = 2000)]
    IncorrectConfiguredVisibility(String),

    // /v3/posts endpoints 2000 ~
    #[error("Post not found")]
    NotFoundPost,

    // /v3/spaces endpoints 3000 ~
    #[error("Space not found")]
    #[rest_error(code = 3000)]
    NotFoundSpace,
    // /v3/spaces/deliberations endpoints 3100 ~

    // /v3/spaces/poll endpoints 3200 ~
    #[rest_error(code = 3200)]
    #[error("Poll has already ended")]
    NotFoundPollSpace,
    #[error("Space is not in progress")]
    SpaceNotInProgress,
    #[error("Answers do not match with questions")]
    AnswersMismatchQuestions,
    #[error("Space cannot be updated in its current status")]
    ImmutablePollSpaceState,
}
