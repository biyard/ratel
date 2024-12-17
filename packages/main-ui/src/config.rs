use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub env: String,
    pub log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV").to_string(),
            log_level: match option_env!("LOG_LEVEL") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
        }
    }
}

pub fn get() -> Config {
    Config::default()
}
