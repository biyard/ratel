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
    pub chime_bucket_name: &'static str,
    pub slack_channel_sponsor: &'static str,
    pub slack_channel_abusing: &'static str,
    pub slack_channel_monitor: &'static str,
    pub kaia: KaiaConfig,
    pub watermark_sqs_url: &'static str,
    pub from_email: &'static str,
    pub telegram_token: Option<&'static str>,
    pub noncelab_token: &'static str,
    pub did: DidConfig,
    pub bedrock: BedrockConfig,
    pub private_bucket_name: &'static str,
    pub dual_write: DualWriteConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct DidConfig {
    pub bbs_bls_x: &'static str,
    pub bbs_bls_y: &'static str,
    pub bbs_bls_d: &'static str,
    pub bbs_bls_crv: &'static str,
    pub p256_x: &'static str,
    pub p256_y: &'static str,
    pub p256_d: &'static str,
    pub p256_crv: &'static str,
}

#[derive(Debug)]
pub struct KaiaConfig {
    pub endpoint: &'static str,
    pub owner_key: &'static str,
    pub owner_address: &'static str,
    pub feepayer_key: &'static str,
    pub feepayer_address: &'static str,
}

#[derive(Debug)]
pub struct BucketConfig {
    pub name: &'static str,
    pub asset_dir: &'static str,
    pub expire: u64,
}
#[derive(Debug)]
pub struct BedrockConfig {
    pub nova_micro_model_id: &'static str,
    pub nova_lite_model_id: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct DualWriteConfig {
    pub enabled: bool,
    pub table_name: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            watermark_sqs_url: option_env!("WATERMARK_QUEUE_URL").expect("You must set WATERMARK_QUEUE_URL"),
            kaia: KaiaConfig {
                endpoint: option_env!("KAIA_ENDPOINT").expect("You must set KAIA_ENDPOINT"),
                owner_key: option_env!("KAIA_OWNER_KEY").expect("You must set KAIA_OWNER_KEY"),
                owner_address: option_env!("KAIA_OWNER_ADDR").expect("You must set KAIA_OWNER_ADDRESS"),
                feepayer_key: option_env!("KAIA_FEEPAYER_KEY").expect("You must set KAIA_FEEPAYER_KEY"),
                feepayer_address: option_env!("KAIA_FEEPAYER_ADDR").expect("You must set KAIA_FEEPAYER_ADDR"),
            },
            from_email: option_env!("FROM_EMAIL").unwrap_or("no-reply@ratel.foundation"),
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
                asset_dir: option_env!("ASSET_DIR").expect("You must set ASSET_DIR"),
                expire: option_env!("BUCKET_EXPIRE").unwrap_or_else(|| {
                    tracing::warn!("We recommend to set BUCKET_EXPIRE. BUCKET_EXPIRE is not set. Default is 3600.");
                    "3600"
                }) .parse()
                    .unwrap(),
            },
            chime_bucket_name: option_env!("CHIME_BUCKET").expect("CHIME_BUCKET required"),
            migrate: option_env!("MIGRATE")
                .map(|s| s.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
            slack_channel_sponsor: option_env!("SLACK_CHANNEL_SPONSOR")
                .expect("SLACK_CHANNEL_SPONSOR is required"),
            slack_channel_abusing: option_env!("SLACK_CHANNEL_ABUSING")
                .expect("SLACK_CHANNEL_ABUSING is required"),
            slack_channel_monitor: option_env!("SLACK_CHANNEL_MONITOR")
                .expect("SLACK_CHANNEL_MONITOR is required"),
            telegram_token: option_env!("TELEGRAM_TOKEN").filter(|s| !s.is_empty()),
            noncelab_token: option_env!("NONCELAB_TOKEN").expect("You must set NONCELAB_TOKEN"),
            did: DidConfig {
                bbs_bls_x: option_env!("BBS_BLS_X").expect("You must set BBS_BLS_X"),
                bbs_bls_y: option_env!("BBS_BLS_Y").expect("You must set BBS_BLS_Y"),
                bbs_bls_d: option_env!("BBS_BLS_D").expect("You must set BBS_BLS_D"),
                bbs_bls_crv: option_env!("BBS_BLS_CRV").expect("You must set BBS_BLS_CRV"),
                p256_x: option_env!("P256_X").expect("You must set P256_X"),
                p256_y: option_env!("P256_Y").expect("You must set P256_Y"),
                p256_d: option_env!("P256_D").expect("You must set P256_D"),
                p256_crv: option_env!("P256_CRV").expect("You must set P256_CRV"),
            },
            private_bucket_name: option_env!("PRIVATE_BUCKET_NAME").expect("You must set PRIVATE_BUCKET_NAME"),
            bedrock: BedrockConfig {
                nova_micro_model_id: option_env!("NOVA_MICRO_MODEL_ID").expect("You must set NOVA_MICRO_MODEL_ID"),
                nova_lite_model_id: option_env!("NOVA_LITE_MODEL_ID").expect("You must set NOVA_LITE_MODEL_ID"),
            },
            dual_write: DualWriteConfig {
                enabled: option_env!("DUAL_WRITE_ENABLED")
                    .map(|s| s.parse::<bool>().unwrap_or(false))
                    .unwrap_or(false),
                table_name: option_env!("DUAL_WRITE_TABLE_NAME").unwrap_or("ratel-main"),
            },
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
