use crate::common::models::User;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AssetPresignedUris {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
    pub upload_id: Option<String>,
    pub key: Option<String>,
}

#[get("/api/assets?total_count&file_type", _user: User)]
pub async fn get_put_object_uri(
    total_count: Option<usize>,
    file_type: Option<String>,
) -> Result<AssetPresignedUris> {
    let config = crate::common::config::CommonConfig::default();
    let client = config.s3();

    let count = total_count.unwrap_or(1).max(1) as i32;
    let prefix = file_type.as_deref();
    let uploads = client.presign_upload(Some(count), prefix, None).await?;

    let presigned_uris = uploads
        .iter()
        .map(|item| item.presigned_uri.clone())
        .collect::<Vec<_>>();
    let uris = uploads
        .iter()
        .map(|item| client.get_url(&item.key))
        .collect::<Vec<_>>();

    Ok(AssetPresignedUris {
        presigned_uris,
        uris,
        upload_id: None,
        key: None,
    })
}
