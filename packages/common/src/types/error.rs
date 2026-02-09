use crate::*;
pub use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown: {0}")]
    Unknown(String),

    // NOTE: Built-in errors for Some macros
    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Not supported: {0}")]
    NotSupported(String),

    #[error("Unauthorized access")]
    UnauthorizedAccess,

    // NOTE: Required by DynamoEntity derive macro generated code
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Invalid bookmark")]
    InvalidBookmark,
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::error::BuildError> for Error {
    fn from(e: aws_sdk_dynamodb::error::BuildError) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::InternalServerError(e.to_string())
    }
}
