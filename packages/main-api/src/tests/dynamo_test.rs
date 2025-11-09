#![allow(warnings)]
use crate::{utils::aws::S3Client, *};
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use axum::AxumRouter;
use base64::{Engine as _, engine::general_purpose};
use bdk::prelude::*;
use tower_sessions::Session;

use crate::{
    AppState,
    models::user::User,
    types::UserType,
    utils::{
        aws::{DynamoClient, SesClient},
        password::hash_password,
    },
};

pub fn get_test_aws_config() -> aws_config::SdkConfig {
    let aws_config = aws_config::SdkConfig::builder()
        .credentials_provider(
            aws_credential_types::provider::SharedCredentialsProvider::new(
                Credentials::builder()
                    .access_key_id("test")
                    .secret_access_key("test")
                    .provider_name("ratel")
                    .build(),
            ),
        )
        .region(aws_config::Region::new("us-east-1"))
        .behavior_version(BehaviorVersion::latest())
        .build();

    aws_config
}
pub fn create_app_state() -> AppState {
    let aws_config = get_test_aws_config();

    AppState::new(
        DynamoClient::mock(aws_config.clone()),
        SesClient::mock(aws_config.clone()),
        S3Client::mock(aws_config),
    )
}

pub async fn create_test_user(cli: &aws_sdk_dynamodb::Client) -> User {
    use crate::types::UserType;

    let profile = "https://ratel.foundation/images/default-profile.png".to_string();
    let username = create_user_name();
    let nickname = create_nick_name();
    let email = format!("a+{}@example.com", nickname);

    let user = User::new(
        nickname,
        email,
        profile,
        true,
        true,
        UserType::Individual,
        username,
        Some("password".to_string()),
    );
    user.create(cli).await.unwrap();

    return user;
}

pub fn create_user_name() -> String {
    let uuid = uuid::Uuid::new_v4();
    format!("user{}", uuid)
}

pub fn create_nick_name() -> String {
    let short_uuid = &uuid::Uuid::new_v4().simple().to_string()[..6];
    format!("nickname{}", short_uuid)
}

pub async fn create_user_session(
    app: AxumRouter,
    cli: &aws_sdk_dynamodb::Client,
) -> (User, axum::http::HeaderMap) {
    let uid = uuid::Uuid::new_v4().to_string();
    let email = format!("{}@example.com", uid);
    let password = hash_password(&uid);
    let user = User::new(
        format!("displayName{}", uid),
        email.clone(),
        "https://metadata.ratel.foundation/ratel/default-profile.png".to_string(),
        true,
        true,
        UserType::Individual,
        uid.clone(),
        Some(password),
    );

    user.create(&cli).await.expect("Failed to create user");
    // For mocking user.
    let (_, header, _) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email,
            "password": uid,
        }
    };
    let session_cookie = header
        .get("set-cookie")
        .expect("No set-cookie header found")
        .to_str()
        .expect("Failed to convert set-cookie header to str")
        .to_string();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("cookie", session_cookie.parse().unwrap());
    (user, headers)
}
