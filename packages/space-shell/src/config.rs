use dioxus::logger::tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub log_level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config {
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
        &CONFIG.as_ref().unwrap()
    }
}
