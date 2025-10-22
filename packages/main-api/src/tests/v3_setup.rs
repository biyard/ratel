use aws_config::Region;
use aws_sdk_dynamodb::{Client, Config, config::Credentials};
use axum::AxumRouter;
use bdk::prelude::*;
use by_types::DatabaseConfig;
use std::time::SystemTime;

use crate::{api_main, config, models::user::User, tests::create_user_session};

#[derive(Clone)]
pub struct TestContextV3 {
    pub app: AxumRouter,
    pub now: u64,
    pub ddb: aws_sdk_dynamodb::Client,
    pub test_user: (User, axum::http::HeaderMap),
    pub user2: (User, axum::http::HeaderMap),
    pub admin_user: (User, axum::http::HeaderMap),
}

impl TestContextV3 {
    pub async fn setup() -> Self {
        setup_v3().await
    }

    pub async fn create_another_user(&self) -> (User, axum::http::HeaderMap) {
        create_user_session(self.app.clone(), &self.ddb).await
    }
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

    if let Some(endpoint) = endpoint.clone() {
        if endpoint.to_lowercase() != "none" && endpoint != "" {
            builder = builder.endpoint_url(endpoint.to_string());
        }
    }
    let aws_config = builder.build();

    let ddb = Client::from_conf(aws_config);
    let (user, headers) = create_user_session(app.clone(), &ddb).await;
    let (user2, headers2) = create_user_session(app.clone(), &ddb).await;

    // Create admin user
    let (mut admin, admin_headers) = create_user_session(app.clone(), &ddb).await;
    admin.user_type = crate::types::UserType::Admin;
    User::updater(admin.pk.clone(), admin.sk.clone())
        .with_user_type(crate::types::UserType::Admin)
        .execute(&ddb)
        .await
        .unwrap();

    TestContextV3 {
        app,
        now,
        ddb,
        test_user: (user, headers),
        user2: (user2, headers2),
        admin_user: (admin, admin_headers),
    }
}
