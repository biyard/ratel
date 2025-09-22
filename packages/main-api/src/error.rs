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
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized(_) => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),

            _ => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}
