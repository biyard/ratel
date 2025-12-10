use aws_sdk_s3::types::CompletedPart;
use bdk::prelude::*;
use by_axum::axum::{Json, extract::State};
use serde::Deserialize;

use crate::{AppState, Error};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct CompleteMultipartUploadRequest {
    pub upload_id: String,
    pub key: String,
    pub parts: Vec<UploadedPart>,
}

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UploadedPart {
    pub part_number: i32,
    pub etag: String,
}

pub async fn complete_multipart_upload(
    State(AppState { .. }): State<AppState>,
    Json(req): Json<CompleteMultipartUploadRequest>,
) -> Result<Json<String>, crate::Error> {
    use aws_config::{BehaviorVersion, Region, defaults};
    use aws_sdk_s3::types::CompletedMultipartUpload;
    use aws_sdk_s3::{Client, config::Credentials};

    let config = crate::config::get();
    let aws_config = &config.aws;
    let bucket_name = config.bucket.name;

    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(config.bucket.region))
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
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
        .bucket(bucket_name)
        .key(&req.key)
        .upload_id(&req.upload_id)
        .multipart_upload(upload)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to complete multipart upload: {}", e);
            Error::AssetError(e.to_string())
        })?;

    let bucket_config = crate::config::get().bucket;
    let final_url = bucket_config.get_url(&req.key);
    Ok(Json(final_url))
}
