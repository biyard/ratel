use aws_sdk_dynamodb::{
    Client, Config,
    config::{Credentials, Region},
};

use crate::aws_config::AwsConfig;
use dioxus::fullstack::Lazy;

pub static DB: Lazy<Client> = Lazy::new(|| async move {
    let endpoint = match option_env!("DYNAMODB_ENDPOINT") {
        Some(ep) => {
            let ep = ep.to_string();
            tracing::info!("DYNAMODB_ENDPOINT: {}", ep);

            if ep.to_lowercase() == "none" || ep.is_empty() {
                None
            } else {
                Some(ep)
            }
        }
        None => None,
    };

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
