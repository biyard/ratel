mod telegram_token;

use crate::features::auth::*;
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

    pub fn s3(&self) -> &crate::common::utils::aws::S3Client {
        self.common.s3()
    }

    pub fn sns(&self) -> &crate::common::utils::aws::SnsClient {
        self.common.sns()
    }

    pub fn ses(&self) -> &crate::common::utils::aws::SesClient {
        self.common.ses()
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
