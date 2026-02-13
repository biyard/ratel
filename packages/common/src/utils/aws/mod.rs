use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;

pub mod dynamo;

pub mod s3;

pub mod error;

pub fn get_aws_config(
    access_key_id: String,
    secret_access_key: String,
    region: String,
) -> aws_config::SdkConfig {
    let timeout_config = aws_config::timeout::TimeoutConfig::builder()
        .operation_attempt_timeout(std::time::Duration::from_secs(5))
        .build();

    let retry_config = aws_config::retry::RetryConfig::standard().with_max_attempts(3);
    let aws_config = aws_config::SdkConfig::builder()
        .credentials_provider(
            aws_credential_types::provider::SharedCredentialsProvider::new(
                Credentials::builder()
                    .access_key_id(access_key_id)
                    .secret_access_key(secret_access_key)
                    .provider_name("ratel")
                    .build(),
            ),
        )
        .region(aws_config::Region::new(region))
        .timeout_config(timeout_config)
        .retry_config(retry_config)
        .behavior_version(BehaviorVersion::latest())
        .build();

    aws_config
}
