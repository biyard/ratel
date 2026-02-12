use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client as S3Client,
    error::{ProvideErrorMetadata, SdkError},
    primitives::ByteStream,
};
use lambda_runtime::Error as LambdaError;
use tracing::error;

pub struct S3Object {
    pub etag: Option<String>,
    pub data: Vec<u8>,
}

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

pub async fn get_object_if_exists(
    s3: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<S3Object>, LambdaError> {
    match s3.get_object().bucket(bucket).key(key).send().await {
        Ok(out) => {
            let etag = out.e_tag().map(|value| value.to_string());
            let data = out
                .body
                .collect()
                .await
                .map_err(|e| {
                    error!("failed to read s3 object body: {e}");
                    LambdaError::from("s3 get_object body failed")
                })?
                .into_bytes()
                .to_vec();
            Ok(Some(S3Object { etag, data }))
        }
        Err(err) => {
            if let SdkError::ServiceError(service_err) = &err {
                if service_err.err().is_no_such_key() {
                    return Ok(None);
                }
            }
            error!("failed to read s3 object: {err}");
            Err(LambdaError::from("s3 get_object failed"))
        }
    }
}

pub async fn upload_object_if_match(
    s3: &S3Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
    etag: Option<&str>,
) -> Result<bool, LambdaError> {
    let mut req = s3
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type);

    if let Some(etag) = etag {
        req = req.if_match(etag);
    } else {
        req = req.if_none_match("*");
    }

    match req.send().await {
        Ok(_) => Ok(true),
        Err(err) => {
            if let SdkError::ServiceError(service_err) = &err {
                if service_err.err().code() == Some("PreconditionFailed") {
                    return Ok(false);
                }
            }
            error!("failed to upload snapshot object: {err}");
            Err(LambdaError::from("s3 put_object failed"))
        }
    }
}
