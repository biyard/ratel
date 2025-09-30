#![allow(warnings)]
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use base64::{Engine as _, engine::general_purpose};
use bdk::prelude::*;
use dto::{
    axum::AxumRouter,
    by_axum::auth::{Authorization, DynamoUserSession},
};
use reqwest::header::HeaderValue;

use crate::{
    AppState,
    models::{email::EmailVerification, user::User},
    post,
    types::{EntityType, Partition},
    utils::aws::{DynamoClient, SesClient},
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

pub async fn ensure_logged_in_and_get_cookie(
    app: AxumRouter,
    ddb: aws_sdk_dynamodb::Client,
    now: u64,
) -> (HeaderValue, String, String) {
    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{:x}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: { "email": email.clone() },
        response_type: crate::controllers::v3::auth::verification::send_code::SendCodeResponse,
    };
    assert_eq!(status, 200, "send-verification-code failed");
    assert!(body.expired_at > now as i64, "expired_at must be in future");

    let EmailVerification { value: code, .. } = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .expect("EmailVerification::get failed")
    .expect("verification row not found");

    let (status, _headers, _user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email.clone(),
            "password": "0x1111",
            "code": code,
            "display_name": "testuser",
            "username": username.clone(),
            "profile_url": "https://example.com/profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User,
    };
    assert_eq!(status, 200, "signup failed");

    let (status, headers, _user) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email.clone(),
            "password": "0x1111",
        },
        response_type: crate::models::user::User,
    };
    assert_eq!(status, 200, "login failed");
    let cookie = headers
        .get("set-cookie")
        .cloned()
        .expect("missing set-cookie header after login");

    (cookie, email, username)
}

pub async fn create_test_user(cli: &aws_sdk_dynamodb::Client) -> User {
    use crate::types::UserType;

    let profile = "http://example.com/profile.png".to_string();
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

#[deprecated(note = "use create_test_user instead")]
pub async fn get_test_user(cli: &aws_sdk_dynamodb::Client) -> User {
    create_test_user(cli).await
}

#[deprecated(note = "use get_auth instead")]
pub async fn create_auth(user: User) -> Authorization {
    get_auth(&user)
}
