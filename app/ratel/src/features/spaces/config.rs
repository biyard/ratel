use crate::common::CommonConfig;

#[cfg(feature = "server")]
use crate::common::by_types::config::{AwsConfig, DatabaseConfig};

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
}

impl Default for Config {
    fn default() -> Self {
        let common = CommonConfig::default();
        Config { common }
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
