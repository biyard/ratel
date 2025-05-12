use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct SlackChannel {
    pub bill: &'static str,
}

#[derive(Debug)]
pub struct Config {
    // pub env: &'static str,
    pub openapi_key: &'static str,
    // pub aws: AwsConfig,
    pub database: DatabaseConfig,
    // pub signing_domain: &'static str,
    // pub auth: AuthConfig,
    pub migrate: bool,
    pub slack: SlackChannel,
    #[allow(dead_code)]
    pub us_congress_key: &'static str,
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
            slack: SlackChannel {
                bill: option_env!("SLACK_CHANNEL_BILL").expect("SLACK_CHANNEL_BILL is required"),
            },
            us_congress_key: option_env!("US_CONGRESS_KEY").expect("US_CONGRESS_KEY is required"),
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
