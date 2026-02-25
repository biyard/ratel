use aws_sdk_dynamodb::{
    Client, Config,
    config::{Credentials, Region},
};

use crate::aws_config::AwsConfig;
use dioxus::fullstack::Lazy;

pub type DynamoClient = Client;

pub static DB: Lazy<Client> = Lazy::new(|| async move {
    let dynamo_config = DynamoConfig::default();
    let endpoint = dynamo_config.endpoint;

    let aws_config = AwsConfig::default();
    let mut builder = Config::builder()
        .region(Region::new(aws_config.region))
        .behavior_version_latest()
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "loaded-from-config",
        ));

    if let Some(ep) = endpoint {
        builder = builder.endpoint_url(ep);
    }

    dioxus::Ok(Client::from_conf(builder.build()))
});

pub struct DynamoConfig {
    pub endpoint: Option<String>,
}

impl Default for DynamoConfig {
    fn default() -> Self {
        let endpoint = match option_env!("DYNAMODB_ENDPOINT") {
            Some(ep) => Some(ep.to_string()),
            None => None,
        };

        DynamoConfig { endpoint }
    }
}
