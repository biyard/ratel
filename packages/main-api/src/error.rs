use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DynamoDB error: {0}")]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),

    #[error("Serde DynamoDB error: {0}")]
    SerdeDynamo(#[from] serde_dynamo::Error),

    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid user")]
    InvalidUser,

    #[error("Unknown error: {0}")]
    Unknown(String),
}
