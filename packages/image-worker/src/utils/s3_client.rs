// FIXME: This code is copy of `packages/main-api/src/utils/s3_upload.rs`
// Please remove this file and use the one from `packages/common` instead.

use std::sync::Arc;

use dto::{Error, Result};

use aws_config::{BehaviorVersion, Region, defaults};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::presigning::PresigningConfig;

use crate::{config, s3_config::S3Config};

pub struct PresignedUrl {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
}

pub struct S3Client {
    pub client: aws_sdk_s3::Client,
}

impl S3Client {
    pub async fn new() -> Result<Arc<Self>> {
        let conf = crate::config::get();
        let aws_conf = defaults(BehaviorVersion::latest())
            .region(Region::new(conf.aws.region))
            .credentials_provider(Credentials::new(
                conf.aws.access_key_id,
                conf.aws.secret_access_key,
                None,
                None,
                "ratel",
            ));

        let sdk_config = aws_conf.load().await;

        let client = aws_sdk_s3::Client::new(&sdk_config);

        Ok(Arc::new(Self { client }))
    }

    pub async fn get_put_object_uri(&self, total_count: i32) -> Result<PresignedUrl> {
        use uuid::Uuid;

        let mut presigned_uris = vec![];
        let mut uris = vec![];
        let S3Config {
            name,
            asset_dir,
            expire,
        } = config::get().bucket;

        for _ in 0..total_count {
            let id = Uuid::new_v4();
            let key = format!("{}/{}", asset_dir, id);

            let presigned_request =
                self.client
                    .put_object()
                    .bucket(name)
                    .key(key.clone())
                    .presigned(
                        PresigningConfig::expires_in(std::time::Duration::from_secs(expire))
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
            uris.push(config::get().bucket.get_url(&key));
        }

        Ok(PresignedUrl {
            presigned_uris,
            uris,
        })
    }
}

// pub async fn get_put_object_uri(
//     aws_config: &AwsConfig,
//     aws_bucket_config: &BucketConfig,
//     total_count: Option<i32>,
// ) -> Result<PresignedUrl> {
//     use aws_sdk_s3::presigning::PresigningConfig;
//     use uuid::Uuid;

//     use aws_config::BehaviorVersion;
//     use aws_config::{Region, defaults};
//     use aws_sdk_s3::config::Credentials;
//     let total_count = total_count.unwrap_or(1);
//     let config = defaults(BehaviorVersion::latest())
//         .region(Region::new(aws_config.region))
//         .credentials_provider(Credentials::new(
//             aws_config.access_key_id,
//             aws_config.secret_access_key,
//             None,
//             None,
//             "ratel",
//         ));

//     let config = config.load().await;

//     let client = aws_sdk_s3::Client::new(&config);

//     let mut presigned_uris = vec![];
//     let mut uris = vec![];

//     for _ in 0..total_count {
//         let id = Uuid::new_v4();
//         let key = format!("{}/{}", aws_bucket_config.asset_dir, id);

//         let presigned_request = client
//             .put_object()
//             .bucket(aws_bucket_config.name)
//             .key(key.clone())
//             .presigned(
//                 PresigningConfig::expires_in(std::time::Duration::from_secs(
//                     aws_bucket_config.expire,
//                 ))
//                 .map_err(|e| {
//                     tracing::error!("Failed to set expired time {}", e.to_string());
//                     Error::AssetError(e.to_string())
//                 })?,
//             )
//             .await
//             .map_err(|e| {
//                 tracing::error!("Failed to put object {}", e.to_string());
//                 Error::AssetError(e.to_string())
//             })?;
//         presigned_uris.push(presigned_request.uri().to_string());
//         uris.push(format!("https://{}/{}", aws_bucket_config.name, key));
//     }

//     Ok(PresignedUrl {
//         presigned_uris,
//         uris,
//         total_count,
//     })
// }
