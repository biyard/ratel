use aws_config::{Region, SdkConfig};
use aws_sdk_dynamodb::{Client, Config, config::Credentials};

#[derive(Debug, Clone)]
pub struct DynamoClient {
    pub client: Client,
}

impl DynamoClient {
    pub fn new(config: &SdkConfig, local_endpoint: Option<String>) -> Self {
        let mut config = config.clone();

        if let Some(endpoint) = local_endpoint {
            config = config.into_builder().endpoint_url(endpoint).build();
        }
        let aws_config = Config::from(&config);

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
