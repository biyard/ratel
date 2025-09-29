use aws_config::Region;
use aws_sdk_dynamodb::{Client, Config, config::Credentials};
use axum::AxumRouter;
use bdk::prelude::*;
use dto::by_types::DatabaseConfig;
use std::time::SystemTime;

use crate::{api_main, config};

pub struct TestContextV3 {
    pub app: AxumRouter,
    pub now: u64,
    pub ddb: aws_sdk_dynamodb::Client,
}

pub async fn setup_v3() -> TestContextV3 {
    let app = api_main::api_main().await.unwrap();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
        - 1750000000u64;

    let app = by_axum::finishing(app);
    let (aws, endpoint, _table_prefix) = match &config::get().dynamodb {
        DatabaseConfig::DynamoDb {
            aws,
            endpoint,
            table_prefix,
        } => (aws, endpoint, table_prefix),
        _ => panic!("Expected DynamoDb configuration for tests"),
    };

    let mut builder = Config::builder()
        .credentials_provider(
            Credentials::builder()
                .access_key_id(aws.access_key_id)
                .secret_access_key(aws.secret_access_key)
                .provider_name("ratel")
                .build(),
        )
        .region(Region::new(aws.region))
        .behavior_version_latest();

    if let Some(endpoint) = endpoint {
        builder = builder.endpoint_url(endpoint.to_string());
    }
    let aws_config = builder.build();

    let ddb = Client::from_conf(aws_config);

    TestContextV3 { app, now, ddb }
}
