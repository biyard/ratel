use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub log_level: Level,
    pub main_api_endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            log_level: match option_env!("LOG_LEVEL") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
            main_api_endpoint: match option_env!("MAIN_API_ENDPOINT") {
                Some(endpoint) => endpoint.to_string(),
                None => format!(
                    "https://api.{}",
                    option_env!("DOMAIN").unwrap_or("dev.democrasee.me")
                ),
            },
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
