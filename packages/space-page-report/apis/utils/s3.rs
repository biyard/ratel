use aws_config::BehaviorVersion;
use aws_sdk_s3::{error::SdkError, Client as S3Client};
use common::serde_json::Value;
use dioxus::prelude::ServerFnError;

pub async fn build_s3_client() -> Result<S3Client, ServerFnError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(S3Client::new(&aws_config))
}

pub async fn load_snapshot_json(
    s3: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<String, ServerFnError> {
    let response = s3
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("s3 get_object failed: {e:?}")))?;

    let body = response
        .body
        .collect()
        .await
        .map_err(|e| ServerFnError::new(format!("s3 read body failed: {e:?}")))?;

    let bytes = body.into_bytes();
    String::from_utf8(bytes.to_vec())
        .map_err(|e| ServerFnError::new(format!("snapshot json utf8 decode failed: {e}")))
}

pub async fn load_json_optional(
    s3: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<Value>, ServerFnError> {
    match s3.get_object().bucket(bucket).key(key).send().await {
        Ok(response) => {
            let body = response
                .body
                .collect()
                .await
                .map_err(|e| ServerFnError::new(format!("s3 read body failed: {e:?}")))?;
            let bytes = body.into_bytes();
            let value = common::serde_json::from_slice::<Value>(&bytes)
                .map_err(|e| ServerFnError::new(format!("json decode failed: {e}")))?;
            Ok(Some(value))
        }
        Err(err) => {
            if let SdkError::ServiceError(service_err) = &err {
                if service_err.err().is_no_such_key() {
                    return Ok(None);
                }
            }
            Err(ServerFnError::new(format!("s3 get_object failed: {err:?}")))
        }
    }
}
