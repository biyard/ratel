use aws_sdk_s3::types::CompletedPart;
use by_axum::axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};

use by_types::AwsConfig;
use dto::*;

use crate::config::BucketConfig;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AssetController {
    config: Arc<AwsConfig>,
    bucket_name: &'static str,
    asset_dir: &'static str,
    expire: u64,
}

impl AssetController {
    pub fn new(
        config: &AwsConfig,
        &BucketConfig {
            name,
            asset_dir,
            expire,
        }: &BucketConfig,
    ) -> Self {
        let config = Arc::new(AwsConfig {
            region: config.region,
            access_key_id: config.access_key_id,
            secret_access_key: config.secret_access_key,
        });

        Self {
            config,
            bucket_name: name,
            asset_dir,
            expire,
        }
    }
    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(Router::new()
            .route("/", get(Self::get_put_object_uri))
            .route("/complete", post(Self::complete_multipart_upload))
            .with_state(self.clone()))
    }

    pub async fn complete_multipart_upload(
        State(ctrl): State<AssetController>,
        Json(req): Json<CompleteMultipartUploadRequest>,
    ) -> Result<Json<String>> {
        use aws_config::{BehaviorVersion, Region, defaults};
        use aws_sdk_s3::types::CompletedMultipartUpload;
        use aws_sdk_s3::{Client, config::Credentials};

        let config = defaults(BehaviorVersion::latest())
            .region(Region::new(ctrl.config.region))
            .credentials_provider(Credentials::new(
                ctrl.config.access_key_id,
                ctrl.config.secret_access_key,
                None,
                None,
                "ratel",
            ));

        let config = config.load().await;
        let client = Client::new(&config);

        let completed_parts: Vec<CompletedPart> = req
            .parts
            .iter()
            .map(|part| {
                CompletedPart::builder()
                    .set_part_number(Some(part.part_number))
                    .set_e_tag(Some(part.etag.clone()))
                    .build()
            })
            .collect();

        let upload = CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        client
            .complete_multipart_upload()
            .bucket(ctrl.bucket_name)
            .key(&req.key)
            .upload_id(&req.upload_id)
            .multipart_upload(upload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to complete multipart upload: {}", e);
                Error::AssetError(e.to_string())
            })?;

        let final_url = format!("https://{}/{}", ctrl.bucket_name, req.key);
        Ok(Json(final_url))
    }

    pub async fn get_put_object_uri(
        State(ctrl): State<AssetController>,
        Query(req): Query<AssetPresignedUrisReadAction>,
    ) -> Result<Json<AssetPresignedUris>> {
        use aws_config::{BehaviorVersion, Region, defaults};
        use aws_sdk_s3::{config::Credentials, presigning::PresigningConfig};
        use std::time::Duration;
        use uuid::Uuid;

        let config = defaults(BehaviorVersion::latest())
            .region(Region::new(ctrl.config.region))
            .credentials_provider(Credentials::new(
                ctrl.config.access_key_id,
                ctrl.config.secret_access_key,
                None,
                None,
                "ratel",
            ));

        let config = config.load().await;
        let client = aws_sdk_s3::Client::new(&config);

        tracing::debug!("/aws/s3/put-uri: {:?}", req);

        let total_count = req.total_count.unwrap_or(1);
        let id = Uuid::new_v4();
        let key = format!("{}/{}", ctrl.asset_dir, id);

        // 1. Initiate multipart upload
        let upload_resp = client
            .create_multipart_upload()
            .bucket(ctrl.bucket_name)
            .key(&key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to initiate multipart upload: {}", e);
                Error::AssetError(e.to_string())
            })?;

        let upload_id = upload_resp
            .upload_id()
            .ok_or_else(|| Error::AssetError("Upload ID missing".to_string()))?
            .to_string();

        // 2. Generate presigned URL for each part
        let mut presigned_uris = vec![];
        for part_number in 1..=total_count {
            let req = client
                .upload_part()
                .bucket(ctrl.bucket_name)
                .key(&key)
                .upload_id(&upload_id)
                .part_number(part_number as i32);

            let presigned = req
                .presigned(
                    PresigningConfig::expires_in(Duration::from_secs(ctrl.expire)).map_err(
                        |e| {
                            tracing::error!("Failed to set expiration: {}", e);
                            Error::AssetError(e.to_string())
                        },
                    )?,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to create presigned part URL: {}", e);
                    Error::AssetError(e.to_string())
                })?;

            presigned_uris.push(presigned.uri().to_string());
        }

        let public_url = format!("https://{}/{}", ctrl.bucket_name, key);

        Ok(Json(AssetPresignedUris {
            presigned_uris,
            uris: vec![public_url],
            total_count,
            file_type: FileType::None,
            upload_id: Some(upload_id),
            key: Some(key),
        }))
    }

    // pub async fn get_put_object_uri(
    //     State(ctrl): State<AssetController>,
    //     Query(req): Query<AssetPresignedUrisReadAction>,
    // ) -> Result<Json<AssetPresignedUris>> {
    //     use aws_sdk_s3::presigning::PresigningConfig;
    //     use uuid::Uuid;

    //     use aws_config::BehaviorVersion;
    //     use aws_config::{Region, defaults};
    //     use aws_sdk_s3::config::Credentials;

    //     let config = defaults(BehaviorVersion::latest())
    //         .region(Region::new(ctrl.config.region))
    //         .credentials_provider(Credentials::new(
    //             ctrl.config.access_key_id,
    //             ctrl.config.secret_access_key,
    //             None,
    //             None,
    //             "ratel",
    //         ));

    //     let config = config.load().await;

    //     let client = aws_sdk_s3::Client::new(&config);

    //     tracing::debug!("/aws/s3/put-uri: {:?}", req);
    //     let mut presigned_uris = vec![];
    //     let mut uris = vec![];
    //     let total_count = req.total_count.unwrap_or(1);
    //     for _ in 0..total_count {
    //         let id = Uuid::new_v4();
    //         let key = format!("{}/{}", ctrl.asset_dir, id);

    //         let presigned_request = client
    //             .put_object()
    //             .bucket(ctrl.bucket_name)
    //             .key(key.clone())
    //             .presigned(
    //                 PresigningConfig::expires_in(std::time::Duration::from_secs(ctrl.expire))
    //                     .map_err(|e| {
    //                         tracing::error!("Failed to set expired time {}", e.to_string());
    //                         Error::AssetError(e.to_string())
    //                     })?,
    //             )
    //             .await
    //             .map_err(|e| {
    //                 tracing::error!("Failed to put object {}", e.to_string());
    //                 Error::AssetError(e.to_string())
    //             })?;
    //         presigned_uris.push(presigned_request.uri().to_string());
    //         uris.push(format!("https://{}/{}", ctrl.bucket_name, key));
    //     }

    //     Ok(Json(AssetPresignedUris {
    //         presigned_uris,
    //         uris,
    //         total_count,
    //         file_type: FileType::None,
    //     }))
    // }
}
