#[derive(Debug, Clone, Copy)]
pub struct BiyardConfig {
    pub api_secret: &'static str,
    pub project_id: &'static str,
    pub base_url: &'static str,
}

impl Default for BiyardConfig {
    fn default() -> Self {
        BiyardConfig {
            api_secret: option_env!("BIYARD_API_SECRET").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_API_SECRET not set, using default value. Some features may not work properly."
                );

                "your_default_api_secret"
            }),
            project_id: option_env!("BIYARD_PROJECT_ID").unwrap_or_else(|| {
                tracing::warn!(
                    "PORTONE_KPN_CHANNEL_KEY not set, using default value. Some features may not work properly."
                );

                "your_default_kpn_channel_key"
            }),
            base_url: option_env!("BIYARD_BASE_URL").unwrap_or_else(|| {
                tracing::warn!(
                    "BIYARD_BASE_URL not set, using default value. Some features may not work properly."
                );

                "your_default_base_url"
            }),
        }
    }
}
