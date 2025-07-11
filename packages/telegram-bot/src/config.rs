use dto::by_types::config::DatabaseConfig;
#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub log_level: &'static str,
    pub telegram_token: &'static str,
    pub telegram_mini_app_uri: &'static str,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            telegram_token: option_env!("TELEGRAM_TOKEN").expect("You must set TELEGRAM_TOKEN"),
            log_level: option_env!("RUST_LOG").unwrap_or("info"),
            telegram_mini_app_uri: option_env!("TELEGRAM_MINI_APP_URI")
                .unwrap_or("https://t.me/crypto_ratel_bot/spaces?startapp"),
            database: DatabaseConfig::default(),
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
        &CONFIG.as_ref().unwrap()
    }
}
