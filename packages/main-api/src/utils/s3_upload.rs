/// Depprecated: use `S3Client::get_put_object_uri` instead.
/// This function will be removed in future versions.
use crate::{Error, Result, by_types::AwsConfig};

use crate::config::BucketConfig;

pub struct PresignedUrl {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
    pub total_count: i32,
}

pub async fn get_put_object_uri(
    aws_config: &AwsConfig,
    aws_bucket_config: &BucketConfig,
    total_count: Option<i32>,
) -> Result<PresignedUrl> {
    use aws_sdk_s3::presigning::PresigningConfig;
    use uuid::Uuid;

    use aws_config::BehaviorVersion;
    use aws_config::{Region, defaults};
    use aws_sdk_s3::config::Credentials;
    let total_count = total_count.unwrap_or(1);
    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(aws_config.region))
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "ratel",
        ));

    let config = config.load().await;

    let client = aws_sdk_s3::Client::new(&config);

    let mut presigned_uris = vec![];
    let mut uris = vec![];

    for _ in 0..total_count {
        let id = Uuid::new_v4();
        let key = format!("{}/{}", aws_bucket_config.asset_dir, id);

        let presigned_request = client
            .put_object()
            .bucket(aws_bucket_config.name)
            .key(key.clone())
            .presigned(
                PresigningConfig::expires_in(std::time::Duration::from_secs(
                    aws_bucket_config.expire,
                ))
                .map_err(|e| {
                    tracing::error!("Failed to set expired time {}", e.to_string());
                    Error::AssetError(e.to_string())
                })?,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to put object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        presigned_uris.push(presigned_request.uri().to_string());
        uris.push(format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            aws_bucket_config.name, aws_bucket_config.region, key
        ));
    }

    Ok(PresignedUrl {
        presigned_uris,
        uris,
        total_count,
    })
}
