use std::time::Duration;

use crate::config;
use aws_config::{Region, retry::RetryConfig, timeout::TimeoutConfig};
use aws_sdk_bedrockruntime::{
    Client, Config,
    config::Credentials,
    types::{ContentBlock, ConversationRole, Message},
};

use dto::{Error, Result};

#[derive(Clone)]
pub struct BedrockClient {
    client: Client,
    model_id: String,
}

impl BedrockClient {
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

        let model_id = conf.bedrock_model_id;

        Self {
            client,
            model_id: model_id.to_string(),
        }
    }

    pub async fn send_message(&self, prompt: String) -> Result<String> {
        let bedrock_response = self
            .client
            .converse()
            .model_id(&self.model_id)
            .messages(
                Message::builder()
                    .role(ConversationRole::User)
                    .content(ContentBlock::Text(prompt))
                    .build()
                    .map_err(|e| {
                        tracing::error!("Error building Bedrock message: {:?}", e);
                        Error::AwsBedrockError(format!("{:?}", e))
                    })?,
            )
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Error calling Bedrock Converse: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;
        tracing::debug!("TOKEN USAGE: {:?}", bedrock_response.usage);
        let contents = bedrock_response
            .output()
            .ok_or(Error::AwsBedrockError("Empty Bedrock response".to_string()))?
            .as_message()
            .map_err(|e| {
                tracing::error!("Error extracting message from Bedrock response: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?
            .content();

        tracing::debug!("Bedrock response content: {:?}", contents);

        let text = contents
            .first()
            .ok_or(Error::AwsBedrockError(
                "Empty Bedrock response content".to_string(),
            ))?
            .as_text()
            .map_err(|e| {
                tracing::error!("Error extracting text from Bedrock content: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?
            .to_string();

        Ok(text)
    }
}
