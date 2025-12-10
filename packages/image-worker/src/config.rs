use crate::s3_config::S3Config;
use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub aws: AwsConfig,
    pub bucket: S3Config,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
            bucket: S3Config::default(),
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
