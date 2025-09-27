use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct BinanceConfig {
    pub redirect_domain: &'static str,
    pub api_key: &'static str,
    pub base_url: &'static str,
    pub secret_key: &'static str,
    pub webhook: &'static str,
}

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    // FIXME: disable for test
    pub binance: BinanceConfig,
    pub aws: AwsConfig,
    pub bucket: BucketConfig,
    pub database: DatabaseConfig,
    pub dynamodb: DatabaseConfig,
    pub signing_domain: &'static str,
    pub auth: AuthConfig,
    pub migrate: bool,
    pub chime_bucket_name: &'static str,
    pub slack_channel_sponsor: &'static str,
    pub slack_channel_abusing: &'static str,
    pub slack_channel_monitor: &'static str,
    pub kaia: KaiaConfig,
    pub from_email: &'static str,
    pub telegram_token: Option<&'static str>,
    pub noncelab_token: &'static str,
    pub did: DidConfig,
    pub private_bucket_name: &'static str,

    // Not supported features in test code
    #[cfg(not(feature = "no-secret"))]
    pub firebase: FirebaseConfig,
    #[cfg(not(feature = "no-secret"))]
    pub bedrock: BedrockConfig,
    #[cfg(not(feature = "no-secret"))]
    // FIXME: integrate with localstack SQS
    pub watermark_sqs_url: &'static str,
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
pub struct FirebaseConfig {
    pub project_id: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kaia: KaiaConfig {
                endpoint: option_env!("KAIA_ENDPOINT").unwrap_or("https://public-en-kairos.node.kaia.io"),
                owner_key: option_env!("KAIA_OWNER_KEY").expect("You must set KAIA_OWNER_KEY"),
                owner_address: option_env!("KAIA_OWNER_ADDR").expect("You must set KAIA_OWNER_ADDRESS"),
                feepayer_key: option_env!("KAIA_FEEPAYER_KEY").expect("You must set KAIA_FEEPAYER_KEY"),
                feepayer_address: option_env!("KAIA_FEEPAYER_ADDR").expect("You must set KAIA_FEEPAYER_ADDR"),
            },
            from_email: option_env!("FROM_EMAIL").unwrap_or("no-reply@ratel.foundation"),
            env: option_env!("ENV").expect("You must set ENV"),
            binance: BinanceConfig { redirect_domain: option_env!("REDIRECT_DOMAIN").unwrap_or("https://dev.ratel.foundation"), api_key: option_env!("BINANCE_API_KEY").expect("BINANCE_API_KEY is required"), base_url: "https://bpay.binanceapi.com/binancepay/openapi", secret_key: option_env!("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY is required"), webhook: option_env!("BINANCE_WEBHOOK").unwrap_or("https://api.dev.ratel.foundation/v2/binances/webhooks"), },
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            signing_domain: option_env!("AUTH_DOMAIN").expect("AUTH_DOMAIN is required"),
            aws: AwsConfig::default(),
            database: DatabaseConfig::Postgres {
                url: option_env!("DATABASE_URL").unwrap_or("postgresql://postgres:postgres@localhost:5432/ratel"),
                pool_size: option_env!("DATABASE_POOL_SIZE")
                    .unwrap_or("10".into())
                    .parse()
                    .expect("DATABASE_POOL_SIZE must be a number")
            },
            dynamodb: DatabaseConfig::DynamoDb {
                aws: AwsConfig::default(),
                endpoint: option_env!("DYNAMO_ENDPOINT"),
                table_prefix: option_env!("DYNAMO_TABLE_PREFIX").expect("You must set TABLE_PREFIX"),
            },
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
            #[cfg(not(feature = "no-secret"))]
            firebase: FirebaseConfig {
                project_id: option_env!("FIREBASE_PROJECT_ID").expect("You must set FIREBASE_PROJECT_ID"),
            },

            #[cfg(not(feature = "no-secret"))]
            bedrock: BedrockConfig {
                nova_micro_model_id: option_env!("NOVA_MICRO_MODEL_ID").expect("You must set NOVA_MICRO_MODEL_ID"),
                nova_lite_model_id: option_env!("NOVA_LITE_MODEL_ID").expect("You must set NOVA_LITE_MODEL_ID"),
            },

            #[cfg(not(feature = "no-secret"))]
            watermark_sqs_url: option_env!("WATERMARK_QUEUE_URL").expect("You must set WATERMARK_QUEUE_URL"),
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
