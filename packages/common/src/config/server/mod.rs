pub mod aws_config;
mod dynamodb;

use super::*;

#[derive(Debug, Clone, Copy, Default)]
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
