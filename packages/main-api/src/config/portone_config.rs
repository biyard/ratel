#[derive(Debug)]
pub struct PortoneConfig {
    pub api_secret: &'static str,
}

impl Default for PortoneConfig {
    fn default() -> Self {
        PortoneConfig {
            api_secret: option_env!("PORTONE_API_SECRET").unwrap_or_else(|| {
                tracing::warn!(
                    "PORTONE_API_SECRET not set, using default value. Some features may not work properly."
                );

                "your_default_api_secret"  
            }),
        }
    }
}
