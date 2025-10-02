use aws_config::{Region, SdkConfig};
use aws_sdk_dynamodb::{Client, Config, config::Credentials};
use dto::by_types::DatabaseConfig;

use crate::config;

#[derive(Debug, Clone)]
pub struct DynamoClient {
    pub client: Client,
}

impl DynamoClient {
    pub fn new(config: Option<SdkConfig>) -> Self {
        let conf = config::get();
        // Only for local development
        let endpoint = match conf.dynamodb {
            DatabaseConfig::DynamoDb { endpoint, .. }
                if endpoint.unwrap_or_default().to_lowercase() == "none"
                    || endpoint.unwrap_or_default() == "" =>
            {
                None
            }
            DatabaseConfig::DynamoDb { endpoint, .. } => endpoint,
            _ => panic!(
                "DynamoDB config not found. In Local env, you must set DynamoDB config with Endpoint"
            ),
        };

        let aws_config = if let Some(config) = config {
            let mut builder = config.into_builder();
            if let Some(endpoint) = endpoint {
                builder = builder.endpoint_url(endpoint.to_string());
            }
            let config = builder.build();
            Config::from(&config)
        } else {
            let mut builder = Config::builder()
                .credentials_provider(
                    Credentials::builder()
                        .access_key_id(conf.aws.access_key_id)
                        .secret_access_key(conf.aws.secret_access_key)
                        .provider_name("ratel")
                        .build(),
                )
                .region(Region::new(conf.aws.region))
                .behavior_version_latest();

            if let Some(endpoint) = endpoint {
                builder = builder.endpoint_url(endpoint.to_string());
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
