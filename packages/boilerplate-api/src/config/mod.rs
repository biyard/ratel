mod aws_config;
mod db_config;
mod env;

pub use aws_config::*;
pub use db_config::*;
pub use env::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Config {
    pub env: Env,
    pub aws: AwsConfig,
    pub db: DbConfig,
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

pub fn ddb() -> &'static aws_sdk_dynamodb::Client {
    get().db.ddb
}
