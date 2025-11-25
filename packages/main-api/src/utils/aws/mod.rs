mod bedrock_runtime;
pub use bedrock_runtime::{BedrockClient, BedrockModel};

mod rekognition;
pub use rekognition::RekognitionClient;

mod poll_scheduler;
pub use poll_scheduler::PollScheduler;

mod textract;
pub use textract::TextractClient;

mod s3;
pub use s3::{S3Client, S3ContentType, S3Object};

pub mod dynamo;
pub use dynamo::DynamoClient;

pub mod ses;
pub use ses::SesClient;

use crate::config;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
pub fn get_aws_config() -> aws_config::SdkConfig {
    let conf = config::get();
    let timeout_config = aws_config::timeout::TimeoutConfig::builder()
        .operation_attempt_timeout(std::time::Duration::from_secs(5))
        .build();

    let retry_config = aws_config::retry::RetryConfig::standard().with_max_attempts(3);
    let aws_config = aws_config::SdkConfig::builder()
        .credentials_provider(
            aws_credential_types::provider::SharedCredentialsProvider::new(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("ratel")
                    .build(),
            ),
        )
        .region(aws_config::Region::new(conf.aws.region))
        .timeout_config(timeout_config)
        .retry_config(retry_config)
        .behavior_version(BehaviorVersion::latest())
        .build();

    aws_config
}
