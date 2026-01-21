use crate::{Error, Result};
use aws_config::Region;
#[cfg(test)]
use aws_config::SdkConfig;
use aws_sdk_s3::{
    Client, Config,
    config::Credentials,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
};

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

pub enum S3ContentType {
    Jpeg,
    Png,
    Pdf,
}

impl From<&str> for S3ContentType {
    fn from(s: &str) -> Self {
        match s {
            "image/jpeg" => S3ContentType::Jpeg,
            "image/jpg" => S3ContentType::Jpeg,
            "image/png" => S3ContentType::Png,
            "application/pdf" => S3ContentType::Pdf,
            _ => S3ContentType::Png,
        }
    }
}

pub struct S3Object {
    pub key: String,
    pub data: Vec<u8>,
    pub content_type: Option<S3ContentType>,
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
            .region(Region::new(conf.s3.region))
            .behavior_version_latest()
            .build();
        let client = Client::from_conf(aws_config);
        S3Client {
            client,
            bucket_name: bucket_name.to_string(),
        }
    }

    pub fn bucket_name(&self) -> &str {
        &self.bucket_name
    }

    pub async fn upload_object(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        let conf = config::get();
        let url = conf.s3.get_url(key);
        Ok(url)
    }
    pub async fn get_object_bytes(&self, key: &str) -> Result<S3Object> {
        let res = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to get object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        let content_type = res.content_type().as_deref().map(S3ContentType::from);
        let data = res.body.collect().await.map_err(|e| {
            tracing::error!("Failed to read object body {}", e.to_string());
            Error::AssetError(e.to_string())
        })?;
        Ok(S3Object {
            key: key.to_string(),
            data: data.to_vec(),
            content_type,
        })
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

            let conf = config::get();
            result.push(PutObjectResult {
                presigned_uri: presigned_request.uri().to_string(),
                uri: conf.s3.get_url(&key),
                key,
            });
        }

        Ok(result)
    }

    pub async fn presign_download(
        &self,
        key: &str,
        filename: &str,
        expire: u64,
    ) -> Result<String> {
        use aws_sdk_s3::presigning::PresigningConfig;

        let safe_name = filename.replace('"', "");
        let disposition = format!("attachment; filename=\"{}\"", safe_name);

        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .response_content_disposition(disposition)
            .presigned(
                PresigningConfig::expires_in(std::time::Duration::from_secs(expire)).map_err(
                    |e| {
                        tracing::error!("Failed to set expired time {}", e.to_string());
                        Error::AssetError(e.to_string())
                    },
                )?,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to presign download {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;

        Ok(presigned.uri().to_string())
    }
    pub async fn delete_objects(&self, keys: Vec<String>) -> Result<()> {
        let objects = keys
            .into_iter()
            .filter_map(|key| match ObjectIdentifier::builder().key(key).build() {
                Ok(obj) => Some(obj),
                Err(e) => {
                    tracing::error!("Failed to create ObjectIdentifier {}", e.to_string());
                    None
                }
            })
            .collect::<Vec<_>>();
        let delete = Delete::builder()
            .set_objects(Some(objects))
            .build()
            .map_err(|e| {
                tracing::error!("Failed to create Delete {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        self.client
            .delete_objects()
            .bucket(&self.bucket_name)
            .delete(delete)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;

        Ok(())
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> Self {
        Self {
            client: Client::from_conf(Config::from(&config)),
            bucket_name: "test-bucket".to_string(),
        }
    }
}
