use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub openapi_key: &'static str,
    pub openapi_url: &'static str,
    pub assembly_system_url: &'static str,
    pub assembly_detail_url: &'static str,
    pub aws: AwsConfig,
    pub bucket: BucketConfig,
    pub database: DatabaseConfig,
    pub signing_domain: &'static str,
    pub auth: AuthConfig,
    pub migrate: bool,
    pub slack_channel_sponsor: &'static str,
    pub slack_channel_abusing: &'static str,
    pub slack_channel_monitor: &'static str,
}

#[derive(Debug)]
pub struct BucketConfig {
    pub name: &'static str,
    pub domain: &'static str,
    pub asset_dir: &'static str,
    pub expire: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            openapi_key: option_env!("OPENAPI_KEY").expect("OPENAPI_KEY is required"),
            openapi_url: "https://open.assembly.go.kr/portal/openapi/",
            assembly_system_url: "https://likms.assembly.go.kr/filegate/servlet/FileGate",
            assembly_detail_url: "https://likms.assembly.go.kr/bill/billDetail.do",
            signing_domain: option_env!("AUTH_DOMAIN").expect("AUTH_DOMAIN is required"),
            aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            bucket: BucketConfig {
                name: option_env!("BUCKET_NAME").expect("You must set BUCKET_NAME"),
                domain: option_env!("BUCKET_DOMAIN").expect("You must set BUCKET_DOMAIN"),
                asset_dir: option_env!("ASSET_DIR").expect("You must set ASSET_DIR"),
                expire: option_env!("BUCKET_EXPIRE").unwrap_or_else(|| {
                    tracing::warn!("We recommend to set BUCKET_EXPIRE. BUCKET_EXPIRE is not set. Default is 3600.");
                    "3600"
                }) .parse()
                    .unwrap(),
            },
            migrate: option_env!("MIGRATE")
                .map(|s| s.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
            slack_channel_sponsor: option_env!("SLACK_CHANNEL_SPONSOR")
                .expect("SLACK_CHANNEL_SPONSOR is required"),
            slack_channel_abusing: option_env!("SLACK_CHANNEL_ABUSING")
                .expect("SLACK_CHANNEL_ABUSING is required"),
            slack_channel_monitor: option_env!("SLACK_CHANNEL_MONITOR")
                .expect("SLACK_CHANNEL_MONITOR is required"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
