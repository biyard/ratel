use common::CommonConfig;
use common::Environment;

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
    pub portone: PortoneConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            common: CommonConfig::default(),
            portone: PortoneConfig::default(),
        }
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

impl Config {
    pub fn day_unit(&self) -> i64 {
        match self.common.env {
            Environment::Local => 60 * 1_000,
            _ => 24 * 60 * 60 * 1_000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PortoneConfig {
    pub api_secret: &'static str,
    pub kpn_channel_key: &'static str,
    pub store_id: &'static str,
    pub notice_url: &'static str,
}

impl PortoneConfig {
    pub fn notice_urls(&self) -> Vec<String> {
        vec![self.notice_url.to_string()]
    }
}

impl Default for PortoneConfig {
    fn default() -> Self {
        let kpn_channel_key = option_env!("PORTONE_KPN_CHANNEL_KEY")
            .filter(|value| !value.is_empty())
            .or_else(|| option_env!("PORTONE_INICIS_CHANNEL_KEY"))
            .unwrap_or("");

        PortoneConfig {
            api_secret: option_env!("PORTONE_API_SECRET").unwrap_or(""),
            kpn_channel_key,
            store_id: option_env!("PORTONE_STORE_ID").unwrap_or(""),
            notice_url: option_env!("PORTONE_NOTICE_URL").unwrap_or("http://localhost:3000/hooks/portone"),
        }
    }
}
