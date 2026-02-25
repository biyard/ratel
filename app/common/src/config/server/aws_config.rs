use std::env;

use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;

#[derive(Debug, Clone, Copy)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
    pub account_id: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        let region = option_env!("AWS_REGION").expect("You must set AWS_REGION");
        let region = env::var("REGION").unwrap_or_else(|_| region.to_string());

        AwsConfig {
            region: Box::leak(region.into_boxed_str()),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID")
                .expect("You must set AWS_ACCESS_KEY_ID"),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required"),
            account_id: option_env!("ACCOUNT_ID").unwrap_or(""),
        }
    }
}

impl AwsConfig {
    pub fn get_sdk_config(&self) -> aws_config::SdkConfig {
        let timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .operation_attempt_timeout(std::time::Duration::from_secs(5))
            .build();

        let retry_config = aws_config::retry::RetryConfig::standard().with_max_attempts(3);
        let aws_config = aws_config::SdkConfig::builder()
            .credentials_provider(
                aws_credential_types::provider::SharedCredentialsProvider::new(
                    Credentials::builder()
                        .access_key_id(self.access_key_id)
                        .secret_access_key(self.secret_access_key)
                        .provider_name("ratel")
                        .build(),
                ),
            )
            .region(aws_config::Region::new(self.region))
            .timeout_config(timeout_config)
            .retry_config(retry_config)
            .behavior_version(BehaviorVersion::latest())
            .build();

        aws_config
    }
}
