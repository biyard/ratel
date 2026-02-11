use aws_config::BehaviorVersion;
use aws_sdk_s3::{primitives::ByteStream, Client as S3Client};
use lambda_runtime::Error as LambdaError;
use tracing::error;

pub async fn build_s3_client() -> Result<S3Client, LambdaError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(S3Client::new(&aws_config))
}

pub async fn upload_object(
    s3: &S3Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<(), LambdaError> {
    s3.put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            error!("failed to upload snapshot object: {e}");
            LambdaError::from("s3 put_object failed")
        })?;
    Ok(())
}
