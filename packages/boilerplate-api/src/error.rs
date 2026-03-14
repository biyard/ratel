use crate::*;
pub use thiserror::Error;

#[derive(Debug, Error, RestError, OperationIo)]
pub enum Error {
    #[error("Unknown: {0}")]
    #[rest_error(code = 1)]
    Unknown(String),

    // NOTE: Built-in errors for Some macros
    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),

    #[error("Bookmark is invalid")]
    InvalidBookmark,

    #[error("DynamoDB error: {0}")]
    #[rest_error(code = 100)]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),

    #[error("SerdeDynamo error: {0}")]
    #[rest_error(status = 500)]
    SerdeDynamo(#[from] serde_dynamo::Error),

    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    // TODO: Define custom errors
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
