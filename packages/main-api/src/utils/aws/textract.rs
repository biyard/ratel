use std::time::Duration;

use crate::config;
use aws_config::{Region, retry::RetryConfig, timeout::TimeoutConfig};

use aws_sdk_textract::{Client, Config, config::Credentials, types::Document};

use dto::{Error, Result};
#[derive(Clone)]
pub struct TextractClient {
    client: Client,
}

impl TextractClient {
    pub fn new() -> Self {
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

        let client = Client::from_conf(aws_config);

        Self { client }
    }

    pub async fn detect_labels(&self, doc: Document) -> Result<Vec<String>> {
        let textract_output = self
            .client
            .detect_document_text()
            .document(doc)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Error calling Textract: {:?}", e);
                Error::AwsTextractError(e.to_string())
            })?;

        Ok(textract_output
            .blocks()
            .iter()
            .filter_map(|block| {
                if block.block_type() == Some(&aws_sdk_textract::types::BlockType::Line) {
                    block.text().map(String::from)
                } else {
                    None
                }
            })
            .collect())
    }
}
