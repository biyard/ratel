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

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Bookmark is invalid")]
    InvalidBookmark,

    #[error("No session found")]
    NoSessionFound,

    #[cfg(feature = "server")]
    #[error("AWS error: {0}")]
    Aws(#[from] crate::utils::aws::error::AwsError),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Unknown(e.to_string())
    }
}

#[cfg(feature = "server")]
impl bdk::prelude::axum::response::IntoResponse for Error {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::http::StatusCode;
        use bdk::prelude::axum::response::IntoResponse;

        let status = match &self {
            Error::UnauthorizedAccess | Error::NoSessionFound => StatusCode::UNAUTHORIZED,
            Error::InvalidPartitionKey(_) | Error::NotSupported(_) | Error::InvalidBookmark => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::Aws(crate::utils::aws::error::AwsError::from(e))
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::Aws(crate::utils::aws::error::AwsError::from(e))
    }
}
