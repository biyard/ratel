use super::ServerConfig;
use crate::common::utils::aws::S3Client;
use dioxus::fullstack::Lazy;

pub static S3_CLIENT: Lazy<S3Client> = Lazy::new(|| async move {
    let config = ServerConfig::default();
    let S3Config {
        bucket_name,
        asset_dir,
        expire,
    } = S3Config::default();

    let aws_sdk_config = config.aws.get_sdk_config();

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

        S3Config {
            bucket_name,
            asset_dir,
            expire,
        }
    }
}
