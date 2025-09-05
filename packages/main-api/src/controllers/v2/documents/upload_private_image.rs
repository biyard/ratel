use bdk::prelude::*;
use dto::{
    JsonSchema, Result, aide,
    by_axum::axum::{
        Json,
        extract::{Query, State},
    },
};
use serde::{Deserialize, Serialize};

use crate::utils::aws::S3Client;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct CreatePrivateImageQueryParams {
    #[schemars(description = "Number of S3 presigned URLs to create")]
    pub total_size: Option<i32>,
    #[schemars(description = "Category for the images, e.g., 'passport', 'medical'")]
    pub category: Option<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct CreatePrivateImageResponse {
    #[schemars(description = "List of S3 presigned URLs to upload images")]
    pub presigned_uris: Vec<PutUrlResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
pub struct PutUrlResponse {
    #[schemars(description = "S3 Presigned URL to upload the image")]
    pub presigned_uri: String,
    #[schemars(description = "S3 Object key for the uploaded image")]
    pub key: String,
}

#[derive(Clone)]
pub struct UploadPrivateImageState {
    pub s3_client: S3Client,
}

pub async fn upload_private_image_handler(
    State(state): State<UploadPrivateImageState>,
    Query(params): Query<CreatePrivateImageQueryParams>,
) -> Result<Json<CreatePrivateImageResponse>> {
    let prefix = params.category.unwrap_or("passport".into());
    let res = state
        .s3_client
        .get_put_object_uri(params.total_size, Some(&prefix), Some(3600))
        .await?;
    let presigned_uris = res
        .into_iter()
        .map(|item| PutUrlResponse {
            presigned_uri: item.presigned_uri,
            key: item.key,
        })
        .collect();
    Ok(Json(CreatePrivateImageResponse { presigned_uris }))
}
