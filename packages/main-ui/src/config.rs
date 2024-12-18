use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub log_level: Level,
    pub topic_api_endpoint: String,
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
            topic_api_endpoint: match option_env!("TOPIC_API_ENDPOINT") {
                Some(endpoint) => endpoint.to_string(),
                None => format!(
                    "https://topic-api.{}",
                    option_env!("DOMAIN").unwrap_or("dev.democrasee.me")
                ),
            },
        }
    }
}

static mut CONFIG: Option<Config> = None;

pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
