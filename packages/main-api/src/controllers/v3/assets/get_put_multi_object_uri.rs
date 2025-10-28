use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{
    AppState, Error, controllers::v3::assets::get_put_object_uri::AssetPresignedUris,
    types::file_type::FileType,
};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPutObjectUriQueryParams {
    #[schemars(description = "total object count")]
    pub total_count: Option<usize>,
    #[schemars(description = "file type")]
    pub file_type: FileType,
}

pub async fn get_put_multi_object_uri(
    State(AppState { .. }): State<AppState>,
    Query(req): Query<GetPutObjectUriQueryParams>,
) -> Result<Json<AssetPresignedUris>, crate::Error> {
    use aws_config::{BehaviorVersion, Region, defaults};
    use aws_sdk_s3::{config::Credentials, presigning::PresigningConfig};
    use std::time::Duration;
    use uuid::Uuid;

    let config = crate::config::get();
    let aws_config = &config.aws;
    let asset_dir = config.bucket.asset_dir;
    let bucket_name = config.bucket.name;
    let expire = config.bucket.expire;

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

    tracing::debug!("/aws/s3/put-uri: {:?}", req);

    let total_count = req.total_count.unwrap_or(1);
    let id = Uuid::new_v4();
    let key = format!("{}/{}", asset_dir, id);

    // 1. Initiate multipart upload
    let upload_resp = client
        .create_multipart_upload()
        .bucket(bucket_name)
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
            .bucket(bucket_name)
            .key(&key)
            .upload_id(&upload_id)
            .part_number((part_number) as i32);

        let presigned = req
            .presigned(
                PresigningConfig::expires_in(Duration::from_secs(expire)).map_err(|e| {
                    tracing::error!("Failed to set expiration: {}", e);
                    Error::AssetError(e.to_string())
                })?,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to create presigned part URL: {}", e);
                Error::AssetError(e.to_string())
            })?;

        presigned_uris.push(presigned.uri().to_string());
    }

    let public_url = format!("https://{}/{}", bucket_name, key);

    Ok(Json(AssetPresignedUris {
        presigned_uris,
        uris: vec![public_url],
        upload_id: Some(upload_id),
        key: Some(key),
    }))
}
