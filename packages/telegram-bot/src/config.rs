#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub log_level: &'static str,
    pub telegram_token: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            telegram_token: option_env!("TELEGRAM_TOKEN").expect("You must set TELEGRAM_TOKEN"),
            log_level: option_env!("RUST_LOG").unwrap_or("info"),
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
