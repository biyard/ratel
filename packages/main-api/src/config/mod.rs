pub mod did_config;
pub mod google_cloud;
mod portone_config;
mod s3_config;
mod x402_config;

use did_config::DidConfig;
pub use google_cloud::*;
pub use portone_config::*;
pub use s3_config::S3Config;
pub use x402_config::*;

mod biyard_config;
use biyard_config::BiyardConfig;

use crate::*;
use by_types::config::*;

#[derive(Debug, Clone, Copy)]
pub struct BinanceConfig {
    pub redirect_domain: &'static str,
    pub api_key: &'static str,
    pub base_url: &'static str,
    pub secret_key: &'static str,
    pub webhook: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    // FIXME: disable for test
    pub binance: BinanceConfig,
    pub aws: AwsConfig,
    pub s3: S3Config,
    pub dynamodb: DatabaseConfig,
    pub chime_bucket_name: &'static str,
    pub kaia: KaiaConfig,
    pub from_email: &'static str,
    pub telegram_token: Option<&'static str>,
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

    pub portone: PortoneConfig,
    pub x402: X402Config,
    pub account_id: &'static str,

    pub biyard: BiyardConfig,
    pub google_cloud: GoogleCloudConfig,
}

impl Config {
    pub fn is_local(&self) -> bool {
        self.env == "local"
    }

    /// Returns the IP address fetched from ifconfig.me at build time.
    /// This is determined during compilation, not at runtime.
    pub fn ip_address(&self) -> &'static str {
        env!("BUILD_IP_ADDRESS")
    }

    pub fn day_unit(&self) -> i64 {
        if self.is_local() {
            return 60 * 1_000; // 1 minute in milliseconds
        }

        24 * 60 * 60 * 1_000
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KaiaConfig {
    pub endpoint: &'static str,
    pub owner_key: &'static str,
    pub owner_address: &'static str,
    pub feepayer_key: &'static str,
    pub feepayer_address: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct BedrockConfig {
    pub nova_micro_model_id: &'static str,
    pub nova_lite_model_id: &'static str,
    pub agent_id: &'static str,
    pub agent_alias_id: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct FirebaseConfig {
    pub project_id: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kaia: KaiaConfig {
                endpoint: option_env!("KAIA_ENDPOINT")
                    .unwrap_or("https://public-en-kairos.node.kaia.io"),
                owner_key: option_env!("KAIA_OWNER_KEY").expect("You must set KAIA_OWNER_KEY"),
                owner_address: option_env!("KAIA_OWNER_ADDR")
                    .expect("You must set KAIA_OWNER_ADDRESS"),
                feepayer_key: option_env!("KAIA_FEEPAYER_KEY")
                    .expect("You must set KAIA_FEEPAYER_KEY"),
                feepayer_address: option_env!("KAIA_FEEPAYER_ADDR")
                    .expect("You must set KAIA_FEEPAYER_ADDR"),
            },
            from_email: option_env!("FROM_EMAIL").unwrap_or("no-reply@ratel.foundation"),
            env: option_env!("ENV").expect("You must set ENV"),
            binance: BinanceConfig {
                redirect_domain: option_env!("REDIRECT_DOMAIN")
                    .unwrap_or("https://dev.ratel.foundation"),
                api_key: option_env!("BINANCE_API_KEY").expect("BINANCE_API_KEY is required"),
                base_url: "https://bpay.binanceapi.com/binancepay/openapi",
                secret_key: option_env!("BINANCE_SECRET_KEY")
                    .expect("BINANCE_SECRET_KEY is required"),
                webhook: option_env!("BINANCE_WEBHOOK")
                    .unwrap_or("https://api.dev.ratel.foundation/v2/binances/webhooks"),
            },
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            aws: AwsConfig::default(),
            dynamodb: DatabaseConfig::DynamoDb {
                aws: AwsConfig::default(),
                endpoint: option_env!("DYNAMO_ENDPOINT"),
                table_prefix: option_env!("DYNAMO_TABLE_PREFIX")
                    .expect("You must set TABLE_PREFIX"),
            },
            s3: S3Config::default(),
            chime_bucket_name: option_env!("CHIME_BUCKET").expect("CHIME_BUCKET required"),
            telegram_token: option_env!("TELEGRAM_TOKEN").filter(|s| !s.is_empty()),
            did: DidConfig::default(),
            private_bucket_name: option_env!("PRIVATE_BUCKET_NAME")
                .expect("You must set PRIVATE_BUCKET_NAME"),
            #[cfg(not(feature = "no-secret"))]
            firebase: FirebaseConfig {
                project_id: option_env!("FIREBASE_PROJECT_ID")
                    .expect("You must set FIREBASE_PROJECT_ID"),
            },

            #[cfg(not(feature = "no-secret"))]
            bedrock: BedrockConfig {
                nova_micro_model_id: option_env!("NOVA_MICRO_MODEL_ID")
                    .expect("You must set NOVA_MICRO_MODEL_ID"),
                nova_lite_model_id: option_env!("NOVA_LITE_MODEL_ID")
                    .expect("You must set NOVA_LITE_MODEL_ID"),
                agent_id: option_env!("BEDROCK_AGENT_ID").expect("You must set BEDROCK_AGENT_ID"),
                agent_alias_id: option_env!("BEDROCK_AGENT_ALIAS_ID")
                    .expect("You must set BEDROCK_AGENT_ALIAS_ID"),
            },

            #[cfg(not(feature = "no-secret"))]
            watermark_sqs_url: option_env!("WATERMARK_QUEUE_URL")
                .expect("You must set WATERMARK_QUEUE_URL"),
            portone: PortoneConfig::default(),
            x402: X402Config::default(),
            account_id: option_env!("ACCOUNT_ID").unwrap_or(""),
            biyard: BiyardConfig::default(),

            google_cloud: GoogleCloudConfig::default(),
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
