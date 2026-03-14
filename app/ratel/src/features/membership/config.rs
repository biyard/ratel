use crate::common::{CommonConfig, Environment};
pub use crate::common::PortoneConfig;

use super::*;

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
