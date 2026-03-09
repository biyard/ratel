mod telegram_token;

use crate::features::posts::*;
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

    pub fn sns(&self) -> &crate::common::utils::aws::SnsClient {
        self.common.sns()
    }

    pub fn ses(&self) -> &crate::common::utils::aws::SesClient {
        self.common.ses()
    }

    pub fn qdrant(&self) -> &crate::common::utils::aws::QdrantClient {
        self.common.qdrant()
    }

    pub fn bedrock_embeddings(&self) -> &crate::common::utils::aws::BedrockEmbeddingsClient {
        self.common.bedrock_embeddings()
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
