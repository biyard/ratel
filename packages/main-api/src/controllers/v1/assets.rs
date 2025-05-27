use by_axum::axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
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
            .with_state(self.clone()))
    }

    pub async fn get_put_object_uri(
        State(ctrl): State<AssetController>,
        Query(req): Query<AssetPresignedUrisReadAction>,
    ) -> Result<Json<AssetPresignedUris>> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use uuid::Uuid;

        use aws_config::BehaviorVersion;
        use aws_config::{Region, defaults};
        use aws_sdk_s3::config::Credentials;

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
        let mut presigned_uris = vec![];
        let mut uris = vec![];
        let total_count = req.total_count.unwrap_or(1);
        for _ in 0..total_count {
            let id = Uuid::new_v4();
            let key = format!("{}/{}", ctrl.asset_dir, id);

            let presigned_request = client
                .put_object()
                .bucket(ctrl.bucket_name)
                .key(key.clone())
                .presigned(
                    PresigningConfig::expires_in(std::time::Duration::from_secs(ctrl.expire))
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
            uris.push(format!("https://{}/{}", ctrl.bucket_name, key));
        }

        Ok(Json(AssetPresignedUris {
            presigned_uris,
            uris,
            total_count,
            file_type: FileType::None,
        }))
    }
}
