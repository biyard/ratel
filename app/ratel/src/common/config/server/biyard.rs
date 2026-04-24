use crate::common::services::BiyardService;
use dioxus::fullstack::Lazy;

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct BiyardConfig {
    pub api_url: String,
    pub api_secret: &'static str,
    pub project_id: &'static str,
}

#[cfg(feature = "server")]
impl Default for BiyardConfig {
    fn default() -> Self {
        Self {
            api_url: std::env::var("BIYARD_API_URL").unwrap_or_else(|_| {
                tracing::warn!(
                    "BIYARD_API_URL not set at runtime, using default value. Some features may not work properly."
                );
                "https://api.biyard.co".to_string()
            }),
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
        config.api_url,
    ))
});
