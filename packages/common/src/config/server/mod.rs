#![allow(static_mut_refs)]
pub mod aws_config;
pub mod aws_ses;
pub mod aws_sns;
pub mod dynamodb;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ServerConfig {
    pub env: Environment,
    pub log_level: LogLevel,

    pub aws: aws_config::AwsConfig,
}

impl ServerConfig {
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        &dynamodb::DB
    }
}

static mut CONFIG: Option<ServerConfig> = None;

impl Default for ServerConfig {
    fn default() -> Self {
        unsafe {
            if CONFIG.is_none() {
                let obj = Self {
                    env: Default::default(),
                    log_level: Default::default(),
                    aws: Default::default(),
                };

                CONFIG = Some(obj);
            }
            CONFIG.clone().unwrap()
        }
    }
}
