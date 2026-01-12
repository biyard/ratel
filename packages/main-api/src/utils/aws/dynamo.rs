use aws_config::{Region, SdkConfig};
use aws_sdk_dynamodb::{Client, Config, config::Credentials};
use bdk::prelude::*;
use by_types::DatabaseConfig;
use std::env;

use crate::config;

#[derive(Debug, Clone)]
pub struct DynamoClient {
    pub client: Client,
}

impl DynamoClient {
    pub fn new(config: Option<SdkConfig>, enable_config: bool) -> Self {
        let (endpoint, aws_region, access_key_id, secret_access_key) = if enable_config {
            let conf = config::get();
            // Only for local development
            let endpoint = match conf.dynamodb {
                DatabaseConfig::DynamoDb { endpoint, .. }
                    if endpoint.unwrap_or_default().to_lowercase() == "none"
                        || endpoint.unwrap_or_default() == "" =>
                {
                    None
                }
                DatabaseConfig::DynamoDb { endpoint, .. } => endpoint.map(|s| s.to_string()),
                _ => panic!(
                    "DynamoDB config not found. In Local env, you must set DynamoDB config with Endpoint"
                ),
            };

            (
                endpoint,
                conf.aws.region.to_string(),
                conf.aws.access_key_id.to_string(),
                conf.aws.secret_access_key.to_string(),
            )
        } else {
            let endpoint = env::var("DYNAMO_ENDPOINT").ok();
            let endpoint = match endpoint.as_deref() {
                Some(value) if value.is_empty() || value.eq_ignore_ascii_case("none") => None,
                Some(value) => Some(value.to_string()),
                None => None,
            };

            let aws_region = env::var("REGION")
                .or_else(|_| env::var("AWS_REGION"))
                .unwrap_or_else(|_| "ap-northeast-2".to_string());

            let access_key_id = env::var("AWS_ACCESS_KEY_ID").unwrap_or_default();
            let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default();

            (endpoint, aws_region, access_key_id, secret_access_key)
        };

        let aws_config = if let Some(config) = config {
            let mut builder = config.into_builder();
            if let Some(endpoint) = endpoint {
                builder = builder.endpoint_url(endpoint);
            }
            let config = builder.build();
            Config::from(&config)
        } else {
            let mut builder = Config::builder()
                .region(Region::new(aws_region))
                .behavior_version_latest();

            if !access_key_id.is_empty() && !secret_access_key.is_empty() {
                builder = builder.credentials_provider(
                    Credentials::builder()
                        .access_key_id(access_key_id)
                        .secret_access_key(secret_access_key)
                        .provider_name("ratel")
                        .build(),
                );
            }

            if let Some(endpoint) = endpoint {
                builder = builder.endpoint_url(endpoint);
            }

            builder.build()
        };

        let client = Client::from_conf(aws_config);
        Self { client }
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> Self {
        let aws_config: aws_sdk_dynamodb::Config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url("http://localhost:4566")
            .build();
        let client = Client::from_conf(aws_config);
        Self { client }
    }
}
