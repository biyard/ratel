use bdk::prelude::*;
use thiserror::Error;

#[derive(Debug, Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("Unknown")]
    #[rest_error(code = 1)]
    Unknown(String),

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
    #[error("Validation errors: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error("Decoding error: {0}")]
    Utf8Decoding(#[from] std::str::Utf8Error),
    #[error("Misconfiguration: {0}")]
    Misconfiguration(String),
    #[error("Operation not supported: {0}")]
    NotSupported(String),
    #[error("The item has dependencies and cannot be deleted: {0:?}")]
    HasDependencies(Vec<String>),
    #[error("Bookmark is invalid")]
    InvalidBookmark,
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    // Authorization errors 400 ~
    #[error("No session found")]
    #[rest_error(status = 401, code = 400)]
    NoSessionFound,
    #[error("No user found in session")]
    #[rest_error(status = 401, code = 401)]
    NoUserFound,
    #[error("No permission to access this resource")]
    #[rest_error(status = 401, code = 403)]
    NoPermission,

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
    PostIncorrectConfiguredVisibility(String),
    #[error("Post not found")]
    #[rest_error(status = 404)]
    PostNotFound,
    #[error("You do not have permission to access this post")]
    PostLikeError,
    #[error("Failed to comment on the post")]
    PostCommentError,
    #[error("Failed to reply to the comment")]
    PostReplyError,

    // /v3/spaces endpoints 3000 ~
    #[error("Space not found")]
    #[rest_error(code = 3000)]
    NotFoundSpace,
    #[error("InvalidTimeRange")]
    InvalidTimeRange,
    // /v3/spaces/deliberations endpoints 3100 ~

    // /v3/spaces/poll endpoints 3200 ~
    #[rest_error(code = 3200)]
    #[error("Poll space not found")]
    NotFoundPollSpace,
    #[error("Space is not in progress")]
    SpaceNotInProgress,
    #[error("Answers do not match with questions")]
    AnswersMismatchQuestions,
    #[error("Space cannot be updated in its current status")]
    ImmutablePollSpaceState,

    // teams 4000 ~
    #[error("Team not found")]
    #[rest_error(status = 404, code = 4000)]
    TeamNotFound,

    // web 1,000,000 ~
    #[error("Web error: {0}")]
    #[rest_error(code = 1_000_000)]
    WebError(#[from] askama::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
