#[cfg(not(feature = "no-secret"))]
pub use r::SqsClient;

#[cfg(feature = "no-secret")]
pub use noop::SqsClient;

#[cfg(not(feature = "no-secret"))]
mod r {
    use std::{sync::Arc, time::Duration};

    use crate::config;
    use aws_config::{Region, retry::RetryConfig, timeout::TimeoutConfig};
    use aws_sdk_sqs::{Config, config::Credentials};

    use dto::{Error, Result};
    pub struct SqsClient {
        client: aws_sdk_sqs::Client,
        queue_url: String,
    }

    impl SqsClient {
        pub async fn new() -> Arc<Self> {
            let conf = config::get();
            let timeout_config = TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_secs(5))
                .build();

            let retry_config = RetryConfig::standard().with_max_attempts(3);
            let aws_config = Config::builder()
                .credentials_provider(
                    Credentials::builder()
                        .access_key_id(conf.aws.access_key_id)
                        .secret_access_key(conf.aws.secret_access_key)
                        .provider_name("ratel")
                        .build(),
                )
                .region(Region::new(conf.aws.region))
                .timeout_config(timeout_config)
                .retry_config(retry_config)
                .behavior_version_latest()
                .clone()
                .build();

            let client = aws_sdk_sqs::Client::from_conf(aws_config);
            Arc::new(Self {
                client,
                queue_url: conf.watermark_sqs_url.to_string(),
            })
        }

        pub async fn send_message(&self, message: &str) -> Result<()> {
            match self
                .client
                .send_message()
                .queue_url(&self.queue_url)
                .message_body(message)
                .send()
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::error!("Failed to send message to SQS: {:?}", e);
                    Err(Error::ServerError(
                        "Failed to send message to SQS".to_string(),
                    ))
                }
            }
        }
    }
}


#[cfg(feature = "no-secret")]
mod noop {
    use std::sync::Arc;

    pub struct SqsClient {
    }

    impl SqsClient {
        pub async fn new() -> Arc<Self> {
            Arc::new(Self {})
        }

        pub async fn send_message(&self, message: &str) -> dto::Result<()> {
            Ok(())
        }
    }
}
