use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use base64::{Engine as _, engine::general_purpose};
use dto::by_axum::auth::{Authorization, DynamoUserSession};

use crate::{
    AppState,
    models::user::User,
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
    }
}

pub async fn get_test_user(cli: &aws_sdk_dynamodb::Client) -> User {
    use crate::types::UserType;

    let profile = "http://example.com/profile.png".to_string();
    let username = create_user_name();
    let email = format!("a+{}@example.com", username);
    let nickname = format!("nickname-{}", username);

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
pub async fn create_auth(user: User) -> Authorization {
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
