use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, InferenceConfiguration, Message,
};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use tokio::sync::OnceCell;

use crate::common::ai::writer::{WriterAi, WriterAiError, WriterAiRequest};
use crate::common::config::ai_writer_config::AiWriterConfig;

pub struct BedrockWriter {
    model: String,
    endpoint: Option<String>,
    client: OnceCell<BedrockClient>,
}

impl BedrockWriter {
    pub fn from_config(cfg: &AiWriterConfig) -> Self {
        Self {
            model: cfg.model.clone(),
            endpoint: cfg.endpoint.clone(),
            client: OnceCell::new(),
        }
    }

    async fn client(&self) -> &BedrockClient {
        self.client
            .get_or_init(|| async {
                let mut loader =
                    aws_config::defaults(BehaviorVersion::latest());
                if let Some(ref ep) = self.endpoint {
                    loader = loader.endpoint_url(ep.clone());
                }
                let cfg = loader.load().await;
                BedrockClient::new(&cfg)
            })
            .await
    }
}

#[async_trait::async_trait]
impl WriterAi for BedrockWriter {
    async fn generate(
        &self,
        req: WriterAiRequest,
    ) -> std::result::Result<String, WriterAiError> {
        let client = self.client().await;

        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(req.user_prompt))
            .build()
            .map_err(|e| WriterAiError::Other(format!("message build: {e:?}")))?;

        let response = client
            .converse()
            .model_id(&self.model)
            .inference_config(
                InferenceConfiguration::builder()
                    .max_tokens(req.max_tokens)
                    .temperature(req.temperature)
                    .build(),
            )
            .messages(message)
            .send()
            .await
            .map_err(|e| WriterAiError::Network(format!("bedrock converse: {e:?}")))?;

        let mut text = String::new();
        if let Some(output) = response.output() {
            if let Ok(msg) = output.as_message() {
                for block in msg.content() {
                    if let Ok(t) = block.as_text() {
                        text.push_str(t);
                    }
                }
            }
        }

        if text.is_empty() {
            Err(WriterAiError::Empty)
        } else {
            Ok(text)
        }
    }
}
