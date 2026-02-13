use dioxus::logger::tracing::Level;

#[cfg(feature = "server")]
use common::by_types::config::{AwsConfig, DatabaseConfig};

#[derive(Debug)]
pub struct Config {
    #[cfg(feature = "server")]
    pub aws: AwsConfig,
    #[cfg(feature = "server")]
    pub dynamodb: DatabaseConfig,
    pub upstream_url: &'static str,
    pub log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            #[cfg(feature = "server")]
            aws: AwsConfig::default(),
            #[cfg(feature = "server")]
            dynamodb: DatabaseConfig::default(),
            upstream_url: option_env!("UPSTREAM_URL").unwrap_or("https://dev.ratel.foundation"),
            log_level: option_env!("LOG_LEVEL")
                .map(|s| s.parse::<Level>().unwrap_or(Level::INFO))
                .unwrap_or(Level::INFO),
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
