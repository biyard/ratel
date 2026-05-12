pub use uuid;

pub use axum::{
    extract::{Extension, FromRef, FromRequest, FromRequestParts, Request, State},
    http::request::Parts,
};
pub use dioxus::fullstack::axum;

pub use aws_sdk_dynamodb;
pub use ethers;

// Re-export for DynamoEntity
pub use base64;
pub use serde_dynamo;

pub use tokio;

pub type DynamoClient = aws_sdk_dynamodb::Client;
