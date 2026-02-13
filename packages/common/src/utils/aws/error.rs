use thiserror::Error;

#[derive(Debug, Error)]
pub enum AwsError {
    #[error("DynamoDB error: {0}")]
    DynamoDb(#[from] aws_sdk_dynamodb::Error),

    #[error("S3 error: {0}")]
    S3(#[from] aws_sdk_s3::Error),

    #[error("SerdeDynamo error: {0}")]
    SerdeDynamo(#[from] serde_dynamo::Error),
}
