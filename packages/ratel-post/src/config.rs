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
        self.common.aws.dynamodb()
    }

    pub fn sns(&self) -> &common::utils::aws::SnsClient {
        self.common.aws.sns()
    }

    pub fn ses(&self) -> &common::utils::aws::SesClient {
        self.common.aws.ses()
    }
}

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
