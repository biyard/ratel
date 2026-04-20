use aws_sdk_bedrockruntime::Client;
use aws_sdk_bedrockruntime::primitives::Blob;

use crate::common::{Error, Result};
use crate::common::utils::InfraError;

const MODEL_ID: &str = "amazon.titan-embed-text-v2:0";
const MAX_INPUT_CHARS: usize = 20_000;

#[derive(Debug, Clone)]
pub struct BedrockEmbeddingsClient {
    client: Client,
}

impl BedrockEmbeddingsClient {
    pub fn new(config: &aws_config::SdkConfig) -> Self {
        let client = Client::new(config);
        Self { client }
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let input_text = if text.len() > MAX_INPUT_CHARS {
            let mut end = MAX_INPUT_CHARS;
            while end > 0 && !text.is_char_boundary(end) {
                end -= 1;
            }
            &text[..end]
        } else {
            text
        };

        let body = serde_json::json!({
            "inputText": input_text,
            "dimensions": 1024,
            "normalize": true
        });

        let response = self
            .client
            .invoke_model()
            .model_id(MODEL_ID)
            .content_type("application/json")
            .body(Blob::new(serde_json::to_vec(&body).map_err(|e| {
                tracing::error!("Failed to serialize embedding request: {e}");
                InfraError::BedrockSerializeFailed
            })?))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Bedrock invoke_model failed: {e}");
                InfraError::BedrockInvokeFailed
            })?;

        let response_body: serde_json::Value =
            serde_json::from_slice(response.body().as_ref()).map_err(|e| {
                tracing::error!("Failed to parse embedding response: {e}");
                Error::from(InfraError::BedrockParseFailed)
            })?;

        let embedding = response_body["embedding"]
            .as_array()
            .ok_or_else(|| {
                Error::from(InfraError::BedrockNoEmbedding)
            })?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        Ok(embedding)
    }
}
