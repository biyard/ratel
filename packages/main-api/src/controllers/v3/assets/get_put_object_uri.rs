use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{AppState, Error, types::file_type::FileType};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPutObjectUriQueryParams {
    #[schemars(description = "total object count")]
    pub total_count: Option<usize>,
    #[schemars(description = "file type")]
    pub file_type: FileType,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct AssetPresignedUris {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
    pub upload_id: Option<String>,
    pub key: Option<String>,
}

pub async fn get_put_object_uri(
    State(AppState { .. }): State<AppState>,
    Query(req): Query<GetPutObjectUriQueryParams>,
) -> Result<Json<AssetPresignedUris>, crate::Error> {
    use aws_sdk_s3::presigning::PresigningConfig;
    use uuid::Uuid;

    use aws_config::BehaviorVersion;
    use aws_config::{Region, defaults};
    use aws_sdk_s3::config::Credentials;

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
    let mut presigned_uris = vec![];
    let mut uris = vec![];
    let total_count = req.total_count.unwrap_or(1);
    for _ in 0..total_count {
        let id = Uuid::new_v4();
        let key = format!("{}/{}", asset_dir, id);

        let presigned_request = client
            .put_object()
            .bucket(bucket_name)
            .key(key.clone())
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
                tracing::error!("Failed to put object {}", e.to_string());
                Error::AssetError(e.to_string())
            })?;
        presigned_uris.push(presigned_request.uri().to_string());
        uris.push(format!("https://{}/{}", bucket_name, key));
    }

    Ok(Json(AssetPresignedUris {
        presigned_uris,
        uris,
        upload_id: None,
        key: None,
    }))
}
