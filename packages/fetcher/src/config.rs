#![allow(unused)]
use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub migrate: bool,
    pub rpc_endpoint: &'static str,

    pub telegram_token: Option<&'static str>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database: DatabaseConfig::default(),
            migrate: option_env!("MIGRATE")
                .map(|s| s.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
            rpc_endpoint: option_env!("RPC_ENDPOINT").expect("RPC_ENDPOINT is required"),
            telegram_token: option_env!("TELEGRAM_TOKEN"),
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
