#[derive(Debug, Clone, Copy)]
pub struct BiyardConfig {
    pub api_secret: &'static str,
    pub project_id: &'static str,
    pub base_url: &'static str,
}

impl Default for BiyardConfig {
    fn default() -> Self {
        BiyardConfig {
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
            base_url: option_env!("BIYARD_API_URL").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_BASE_URL not set, using default value. Some features may not work properly."
                );

                "https://dev.biyard.co"
            }),
        }
    }
}
