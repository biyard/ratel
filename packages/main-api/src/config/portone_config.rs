use super::*;

#[derive(Debug, Clone, Copy)]
pub struct PortoneConfig {
    pub api_secret: &'static str,
    pub kpn_channel_key: &'static str,
    pub store_id: &'static str,
}

impl PortoneConfig {
    pub fn notice_urls(&self) -> Vec<String> {
        let conf = get();

        if conf.is_local() {
            let ip = conf.ip_address();
            warn!("Using local IP address for notice_urls: {}", ip);
            vec![format!("http://{ip}:3000/hooks/portone")]
        } else {
            vec![format!("https://{}/hooks/portone", conf.domain)]
        }
    }
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
            kpn_channel_key: option_env!("PORTONE_KPN_CHANNEL_KEY").unwrap_or_else(|| {
                tracing::warn!(
                    "PORTONE_KPN_CHANNEL_KEY not set, using default value. Some features may not work properly."
                );

                "your_default_kpn_channel_key"
            }),
            store_id: option_env!("PORTONE_STORE_ID").unwrap_or_else(|| {
                tracing::warn!(
                    "PORTONE_STORE_ID not set, using default value. Some features may not work properly."
                );

                "your_default_store_id"
            }),
        }
    }
}
