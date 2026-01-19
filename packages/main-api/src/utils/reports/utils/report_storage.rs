use crate::*;

use aws_config::BehaviorVersion;
use aws_config::{Region, defaults};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReportS3Config {
    pub bucket_name: String,
    pub asset_dir: String,
    pub env: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

#[derive(Debug, Clone)]
pub struct PresignedReportUpload {
    pub presigned_url: String,
    pub metadata_url: String,
    pub key: String,
}

pub async fn upload_report_pdf(pdf_bytes: Vec<u8>) -> Result<(String, String)> {
    let ratel_config = crate::config::get();
    let aws_config = &ratel_config.aws;

    let asset_dir = ratel_config.s3.asset_dir;
    let bucket_name = ratel_config.s3.name;
    let bucket_region = ratel_config.s3.region;

    let env = ratel_config.env;

    let cfg = defaults(BehaviorVersion::latest())
        .region(Region::new(bucket_region))
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "ratel",
        ))
        .load()
        .await;

    let client = S3Client::new(&cfg);

    let id = Uuid::new_v4();
    let key = format!("{}/{}/reports/{}.pdf", asset_dir, env.to_lowercase(), id);

    client
        .put_object()
        .bucket(bucket_name)
        .key(&key)
        .content_type("application/pdf")
        .body(ByteStream::from(pdf_bytes))
        .send()
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let uri = ratel_config.s3.get_url(&key);
    Ok((key, uri))
}

pub async fn upload_report_pdf_with_config(
    pdf_bytes: Vec<u8>,
    cfg: ReportS3Config,
) -> Result<(String, String)> {
    let aws_cfg = defaults(BehaviorVersion::latest())
        .region(Region::new(cfg.region.clone()))
        .credentials_provider(Credentials::new(
            cfg.access_key_id.clone(),
            cfg.secret_access_key.clone(),
            None,
            None,
            "ratel",
        ))
        .load()
        .await;

    let client = S3Client::new(&aws_cfg);

    let id = Uuid::new_v4();
    let key = format!(
        "{}/{}/reports/{}.pdf",
        cfg.asset_dir,
        cfg.env.to_lowercase(),
        id
    );

    client
        .put_object()
        .bucket(&cfg.bucket_name)
        .key(&key)
        .content_type("application/pdf")
        .body(ByteStream::from(pdf_bytes))
        .send()
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let uri = format!("https://{}/{}", cfg.bucket_name, key);
    Ok((key, uri))
}

pub async fn presign_report_upload() -> Result<PresignedReportUpload> {
    use aws_sdk_s3::presigning::PresigningConfig;

    let ratel_config = crate::config::get();
    let aws_config = &ratel_config.aws;

    let asset_dir = ratel_config.s3.asset_dir;
    let bucket_name = ratel_config.s3.name;
    let bucket_region = ratel_config.s3.region;
    let env = ratel_config.env;

    let cfg = defaults(BehaviorVersion::latest())
        .region(Region::new(bucket_region))
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "ratel",
        ))
        .load()
        .await;

    let client = S3Client::new(&cfg);
    let id = Uuid::new_v4();
    let key = format!("{}/{}/reports/{}.pdf", asset_dir, env.to_lowercase(), id);

    let presigned = client
        .put_object()
        .bucket(bucket_name)
        .key(&key)
        .content_type("application/pdf")
        .presigned(
            PresigningConfig::expires_in(std::time::Duration::from_secs(
                ratel_config.s3.expire,
            ))
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?,
        )
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let metadata_url = ratel_config.s3.get_url(&key);

    Ok(PresignedReportUpload {
        presigned_url: presigned.uri().to_string(),
        metadata_url,
        key,
    })
}
