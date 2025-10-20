#![allow(warnings)]
use crate::*;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use axum::AxumRouter;
use base64::{Engine as _, engine::general_purpose};
use bdk::prelude::*;
use by_axum::auth::{Authorization, DynamoUserSession};
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

    AppState {
        dynamo: DynamoClient::mock(aws_config.clone()),
        ses: SesClient::mock(aws_config),
        pool: sqlx::Pool::connect_lazy("postgres://postgres:password@localhost/postgres").unwrap(),
    }
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

pub fn get_auth(user: &User) -> Authorization {
    let session = DynamoUserSession {
        pk: user.pk.to_string(),
        typ: user.user_type as i64,
    };
    Authorization::DynamoSession(session)
}

pub fn mock_principal_token() -> String {
    use ring::rand::SystemRandom;
    use ring::signature::{ED25519, Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey};

    let rng = SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key pair");
    let keypair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).expect("Failed to parse pkcs8");
    let public_key = keypair.public_key().as_ref();

    let timestamp = chrono::Utc::now().timestamp();

    let message = format!("{}-{}", "dev.ratel.foundation", timestamp);
    let message_bytes = message.as_bytes();

    let signature: Signature = keypair.sign(message_bytes);

    UnparsedPublicKey::new(&ED25519, public_key)
        .verify(message_bytes, signature.as_ref())
        .expect("Failed to verify signature");

    let public_key_b64 = general_purpose::STANDARD.encode(public_key);
    let signature_b64 = general_purpose::STANDARD.encode(signature.as_ref());

    let token = format!("{}:eddsa:{}:{}", timestamp, public_key_b64, signature_b64);

    token
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
