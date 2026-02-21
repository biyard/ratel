use aws_config::SdkConfig;
pub use aws_sdk_s3::Client;
use aws_sdk_s3::{
    Config,
    presigning::PresigningConfig,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
};
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct S3Client {
    pub client: Client,
    bucket_name: String,
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Jpeg,
    Png,
    Pdf,
}

impl From<&str> for ContentType {
    fn from(s: &str) -> Self {
        match s {
            "image/jpeg" | "image/jpg" => ContentType::Jpeg,
            "image/png" => ContentType::Png,
            "application/pdf" => ContentType::Pdf,
            _ => ContentType::Png,
        }
    }
}

pub struct S3Object {
    pub key: String,
    pub data: Vec<u8>,
    pub content_type: Option<ContentType>,
}

pub struct PutObjectResult {
    pub presigned_uri: String,
    pub key: String,
}

impl S3Client {
    pub fn new(config: &SdkConfig, bucket_name: String) -> S3Client {
        let aws_config = Config::from(config);
        let client = Client::from_conf(aws_config);
        S3Client {
            client,
            bucket_name,
        }
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> S3Client {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);
        S3Client {
            client,
            bucket_name: "common".to_string(),
        }
    }
}

impl S3Client {
    pub async fn upload(&self, key: &str, data: Vec<u8>, content_type: &str) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload object: {}", e);
                Error::InternalServerError(e.to_string())
            })?;
        Ok(())
    }

    pub async fn get_object(&self, key: &str) -> Result<S3Object> {
        let res = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to get object: {}", e);
                Error::InternalServerError(e.to_string())
            })?;
        let content_type = res.content_type().as_deref().map(ContentType::from);
        let data = res.body.collect().await.map_err(|e| {
            tracing::error!("Failed to read object body: {}", e);
            Error::InternalServerError(e.to_string())
        })?;
        Ok(S3Object {
            key: key.to_string(),
            data: data.to_vec(),
            content_type,
        })
    }

    pub async fn presign_upload(
        &self,
        total_count: Option<i32>,
        prefix: Option<&str>,
        expire: Option<u64>,
    ) -> Result<Vec<PutObjectResult>> {
        let total_count = total_count.unwrap_or(1);
        let expire_time = expire.unwrap_or(3600);
        let mut result: Vec<PutObjectResult> = vec![];

        for _ in 0..total_count {
            let id = Uuid::new_v4();
            let key = if let Some(p) = prefix {
                format!("{}/{}", p.trim_end_matches('/'), id)
            } else {
                id.to_string()
            };
            let presigned = self
                .client
                .put_object()
                .bucket(&self.bucket_name)
                .key(key.clone())
                .presigned(
                    PresigningConfig::expires_in(std::time::Duration::from_secs(expire_time))
                        .map_err(|e| {
                            tracing::error!("Failed to set expire time: {}", e);
                            Error::InternalServerError(e.to_string())
                        })?,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to presign upload: {}", e);
                    Error::InternalServerError(e.to_string())
                })?;

            result.push(PutObjectResult {
                presigned_uri: presigned.uri().to_string(),
                key,
            });
        }

        Ok(result)
    }

    pub async fn presign_download(&self, key: &str, filename: &str, expire: u64) -> Result<String> {
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
                        tracing::error!("Failed to set expire time: {}", e);
                        Error::InternalServerError(e.to_string())
                    },
                )?,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to presign download: {}", e);
                Error::InternalServerError(e.to_string())
            })?;

        Ok(presigned.uri().to_string())
    }

    pub async fn delete_objects(&self, keys: Vec<String>) -> Result<()> {
        let objects = keys
            .into_iter()
            .filter_map(|key| match ObjectIdentifier::builder().key(key).build() {
                Ok(obj) => Some(obj),
                Err(e) => {
                    tracing::error!("Failed to create ObjectIdentifier: {}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        let delete = Delete::builder()
            .set_objects(Some(objects))
            .build()
            .map_err(|e| {
                tracing::error!("Failed to build Delete: {}", e);
                Error::InternalServerError(e.to_string())
            })?;

        self.client
            .delete_objects()
            .bucket(&self.bucket_name)
            .delete(delete)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete objects: {}", e);
                Error::InternalServerError(e.to_string())
            })?;

        Ok(())
    }
}
