use aws_config::Region;
use aws_sdk_s3::{Client, Config, config::Credentials};
use dto::{Error, Result};

use crate::config;

pub struct PutObjectResult {
    pub presigned_uri: String,
    pub uri: String,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct S3Client {
    pub client: Client,
    bucket_name: String,
}

impl S3Client {
    pub fn new(bucket_name: &str) -> Self {
        let conf = config::get();
        let aws_config = Config::builder()
            .credentials_provider(Credentials::new(
                conf.aws.access_key_id,
                conf.aws.secret_access_key,
                None,
                None,
                "ratel",
            ))
            .region(Region::new(conf.aws.region))
            .behavior_version_latest()
            .build();
        let client = Client::from_conf(aws_config);
        S3Client {
            client,
            bucket_name: bucket_name.to_string(),
        }
    }

    pub async fn get_put_object_uri(
        &self,
        total_count: Option<i32>,
        prefix: Option<&str>,
        expire: Option<u64>,
    ) -> Result<Vec<PutObjectResult>> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use uuid::Uuid;

        let total_count = total_count.unwrap_or(1);
        let mut result: Vec<PutObjectResult> = vec![];
        let expire_time = expire.unwrap_or(3600);
        for _ in 0..total_count {
            let id = Uuid::new_v4();
            let key = if let Some(p) = prefix {
                format!("{}/{}", p.trim_end_matches('/'), id)
            } else {
                id.to_string()
            };
            let presigned_request = self
                .client
                .put_object()
                .bucket(&self.bucket_name)
                .key(key.clone())
                .presigned(
                    PresigningConfig::expires_in(std::time::Duration::from_secs(expire_time))
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

            result.push(PutObjectResult {
                presigned_uri: presigned_request.uri().to_string(),
                uri: format!("https://{}/{}", &self.bucket_name, key),
                key,
            });
        }

        Ok(result)
    }
    pub async fn delete_object(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        Ok(())
    }
}
