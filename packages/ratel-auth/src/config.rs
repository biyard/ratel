mod telegram_token;

use crate::*;
use dioxus::logger::tracing::Level;

pub use telegram_token::*;

#[derive(Debug, Default)]
pub struct Config {
    pub common: CommonConfig,

    pub telegram_token: TelegramToken,
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
