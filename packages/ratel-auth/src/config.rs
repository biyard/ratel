mod telegram_token;

use crate::*;
use dioxus::logger::tracing::Level;

pub use telegram_token::*;

#[derive(Debug, Default)]
pub struct Config {
    pub common: CommonConfig,

    pub telegram_token: TelegramToken,
}

#[cfg(feature = "server")]
impl Config {
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        self.common.dynamodb()
    }

    pub fn sns(&self) -> &crate::utils::aws::SnsClient {
        &SNS_CLIENT
    }

    pub fn ses(&self) -> &crate::utils::aws::SesClient {
        &SES_CLIENT
    }
}

#[cfg(feature = "server")]
static SNS_CLIENT: dioxus::fullstack::Lazy<crate::utils::aws::SnsClient> =
    dioxus::fullstack::Lazy::new(|| async move {
        let aws_conf = common::config::aws_config::AwsConfig::default();
        let sns_region = std::env::var("SNS_REGION")
            .unwrap_or_else(|_| aws_conf.region.to_string());
        let config = aws_sdk_sns::Config::builder()
            .region(aws_sdk_sns::config::Region::new(sns_region))
            .behavior_version_latest()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                aws_conf.access_key_id,
                aws_conf.secret_access_key,
                None,
                None,
                "loaded-from-config",
            ))
            .build();
        dioxus::Ok(crate::utils::aws::SnsClient::new(config))
    });

#[cfg(feature = "server")]
static SES_CLIENT: dioxus::fullstack::Lazy<crate::utils::aws::SesClient> =
    dioxus::fullstack::Lazy::new(|| async move {
        let aws_conf = common::config::aws_config::AwsConfig::default();
        let config = aws_sdk_sesv2::Config::builder()
            .region(aws_sdk_sesv2::config::Region::new(aws_conf.region))
            .behavior_version_latest()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                aws_conf.access_key_id,
                aws_conf.secret_access_key,
                None,
                None,
                "loaded-from-config",
            ))
            .build();
        let from_email = std::env::var("FROM_EMAIL")
            .unwrap_or_else(|_| "no-reply@ratel.foundation".to_string());
        dioxus::Ok(crate::utils::aws::SesClient::new(config, true, from_email))
    });

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
