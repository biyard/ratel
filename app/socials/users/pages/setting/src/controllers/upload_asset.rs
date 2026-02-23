use crate::*;
#[cfg(feature = "server")]
use ::aws_sdk_s3::config::Credentials;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AssetPresignedUris {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
    pub upload_id: Option<String>,
    pub key: Option<String>,
}

#[get("/api/assets?total_count&file_type", user: ratel_auth::User)]
pub async fn get_put_object_uri(
    total_count: Option<usize>,
    file_type: Option<String>,
) -> Result<AssetPresignedUris> {
    #[cfg(not(feature = "server"))]
    {
        let _ = total_count;
        return Err(Error::NotSupported(
            "Asset upload is only available on server.".to_string(),
        ));
    }

    #[cfg(feature = "server")]
    {
        use ::aws_config::{BehaviorVersion, Region, defaults};
        let config = crate::config::get();
        let s3 = config.s3;
        let aws = config.common.aws;

        let loader = defaults(BehaviorVersion::latest())
            .region(Region::new(s3.region))
            .credentials_provider(Credentials::new(
                aws.access_key_id,
                aws.secret_access_key,
                None,
                None,
                "ratel",
            ));
        let sdk_config = loader.load().await;

        let client = common::utils::aws::S3Client::new(&sdk_config, s3.name.to_string());
        let count = total_count.unwrap_or(1).max(1) as i32;
        let uploads = client
            .presign_upload(Some(count), Some(s3.asset_dir), Some(s3.expire))
            .await?;

        let presigned_uris = uploads
            .iter()
            .map(|item| item.presigned_uri.clone())
            .collect::<Vec<_>>();
        let uris = uploads
            .iter()
            .map(|item| s3.get_url(&item.key))
            .collect::<Vec<_>>();

        return Ok(AssetPresignedUris {
            presigned_uris,
            uris,
            upload_id: None,
            key: None,
        });
    }
}
