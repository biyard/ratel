use super::ServerConfig;
use crate::common::utils::aws::S3Client;
use aws_config::Region;
use dioxus::fullstack::Lazy;

pub static S3_CLIENT: Lazy<S3Client> = Lazy::new(|| async move {
    let config = ServerConfig::default();
    let S3Config {
        bucket_name,
        asset_dir,
        expire,
        region,
    } = S3Config::default();

    let aws_sdk_config = config.aws.get_sdk_config();
    let aws_sdk_config = aws_sdk_config.into_builder().region(Region::new(region)).build();

    dioxus::Ok(S3Client::new(
        &aws_sdk_config,
        bucket_name,
        asset_dir,
        expire,
    ))
});

pub struct S3Config {
    pub bucket_name: String,
    pub asset_dir: Option<String>,
    pub expire: u64,
    pub region: String,
}

impl Default for S3Config {
    fn default() -> Self {
        let bucket_name = match option_env!("BUCKET_NAME") {
            Some(ep) => ep.to_string(),
            None => "metadata.ratel.foundation".to_string(),
        };
        let asset_dir = match option_env!("ASSET_DIR") {
            Some(ep) => Some(ep.to_string()),
            None => None,
        };
        let expire = match option_env!("BUCKET_EXPIRE") {
            Some(value) => value.parse::<u64>().ok().unwrap_or_default(),
            None => 3600,
        };
        let region = option_env!("S3_REGION").unwrap_or("ap-northeast-2").to_string();

        S3Config {
            bucket_name,
            asset_dir,
            expire,
            region,
        }
    }
}
