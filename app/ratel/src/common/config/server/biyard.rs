use aws_sdk_dynamodb::{
    config::{Credentials, Region},
    Client, Config,
};

use crate::common::{aws_config::AwsConfig, services::BiyardService};
use dioxus::fullstack::Lazy;

#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy)]
pub struct BiyardConfig {
    pub api_secret: &'static str,
    pub project_id: &'static str,
}

#[cfg(feature = "server")]
impl Default for BiyardConfig {
    fn default() -> Self {
        Self {
            api_secret: option_env!("BIYARD_API_KEY").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_API_KEY not set, using default value. Some features may not work properly."
                );
                "biyard_default_api_key"
            }),
            project_id: option_env!("BIYARD_PROJECT_ID").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_PROJECT_ID not set, using default value. Some features may not work properly."
                );
                "ratel_project_id"
            }),
        }
    }
}

pub static BIYARD_SERVICE: Lazy<BiyardService> = Lazy::new(|| async move {
    let config = BiyardConfig::default();
    dioxus::Ok(BiyardService::new(
        config.api_secret.to_string(),
        config.project_id.to_string(),
        "https://api.biyard.co".to_string(),
    ))
});
