use aws_config::{Region, SdkConfig};
pub use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::{Config, config::Credentials};
#[derive(Debug, Clone)]
pub struct DynamoBuilder;

impl DynamoBuilder {
    pub fn new(config: &SdkConfig, local_endpoint: Option<String>) -> DynamoClient {
        let mut config = config.clone();

        if let Some(endpoint) = local_endpoint {
            config = config.into_builder().endpoint_url(endpoint).build();
        }
        let aws_config = Config::from(&config);

        let client = DynamoClient::from_conf(aws_config);
        client
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> DynamoClient {
        let aws_config: aws_sdk_dynamodb::Config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url("http://localhost:4566")
            .build();
        let client = DynamoClient::from_conf(aws_config);
        client
    }
}
