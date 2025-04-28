use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    // pub env: &'static str,
    pub openapi_key: &'static str,
    // pub aws: AwsConfig,
    pub database: DatabaseConfig,
    // pub signing_domain: &'static str,
    // pub auth: AuthConfig,
    pub migrate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // env: option_env!("ENV").expect("You must set ENV"),
            openapi_key: option_env!("OPENAPI_KEY").expect("OPENAPI_KEY is required"),
            // signing_domain: option_env!("AUTH_DOMAIN").expect("AUTH_DOMAIN is required"),
            // aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
            // auth: AuthConfig::default(),
            migrate: option_env!("MIGRATE")
                .map(|s| s.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
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
