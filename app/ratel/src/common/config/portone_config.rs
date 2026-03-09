use crate::common::{info, warn, Environment};

#[derive(Debug, Clone, Copy)]
pub struct PortoneConfig {
    #[cfg(feature = "server")]
    pub api_secret: &'static str,
    pub kpn_channel_key: &'static str,
    pub inicis_channel_key: &'static str,
    pub store_id: &'static str,
}

impl PortoneConfig {
    pub fn channel_key(&self) -> &'static str {
        if !self.kpn_channel_key.is_empty() {
            self.kpn_channel_key
        } else {
            self.inicis_channel_key
        }
    }

    #[cfg(feature = "server")]
    pub fn notice_urls(&self, env: Environment) -> Vec<String> {
        let hook = match env {
            Environment::Local => {
                let output = std::process::Command::new("curl")
                    .args(["-s", "--max-time", "5", "https://ifconfig.me"])
                    .output();
                let ip_address = match output {
                    Ok(out) if out.status.success() => {
                        String::from_utf8_lossy(&out.stdout).trim().to_string()
                    }
                    _ => {
                        warn!("Failed to fetch IP from ifconfig.me, defaulting to localhost");
                        "localhost".to_string()
                    }
                };

                let port = option_env!("PORT").unwrap_or("8000");
                let url = format!("http://{}:{}/hooks/portone", ip_address, port);
                info!("Using local IP address for Portone notice URL: {}", url);
                Box::leak(url.into_boxed_str())
            }
            Environment::Dev => "https://dev.ratel.foundation/hooks/portone",
            Environment::Staging => "https://stg.ratel.foundation/hooks/portone",
            Environment::Production => "https://ratel.foundation/hooks/portone",
        };

        vec![hook.to_string()]
    }
}

impl Default for PortoneConfig {
    fn default() -> Self {
        PortoneConfig {
            #[cfg(feature = "server")]
            api_secret: option_env!("PORTONE_API_SECRET").unwrap_or_else(|| {
                warn!(
                    "PORTONE_API_SECRET not set, using default value. Some features may not work properly."
                );
                "your_default_api_secret"
            }),
            kpn_channel_key: option_env!("PORTONE_KPN_CHANNEL_KEY").unwrap_or_else(|| {
                warn!(
                    "PORTONE_KPN_CHANNEL_KEY not set, using default value. Some features may not work properly."
                );
                "your_default_kpn_channel_key"
            }),
            inicis_channel_key: option_env!("PORTONE_INICIS_CHANNEL_KEY").unwrap_or_else(|| {
                warn!(
                    "PORTONE_INICIS_CHANNEL_KEY not set, using default value. Some features may not work properly."
                );
                "your_default_inicis_channel_key"
            }),
            store_id: option_env!("PORTONE_STORE_ID").unwrap_or_else(|| {
                warn!(
                    "PORTONE_STORE_ID not set, using default value. Some features may not work properly."
                );
                "your_default_store_id"
            }),
        }
    }
}
