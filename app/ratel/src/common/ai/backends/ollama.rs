use serde::{Deserialize, Serialize};

use crate::common::ai::writer::{WriterAi, WriterAiError, WriterAiRequest};
use crate::common::config::ai_writer_config::AiWriterConfig;

const DEFAULT_ENDPOINT: &str = "http://localhost:11434";

pub struct OllamaWriter {
    model: String,
    endpoint: String,
    http: reqwest::Client,
}

impl OllamaWriter {
    pub fn from_config(cfg: &AiWriterConfig) -> Self {
        let endpoint = cfg
            .endpoint
            .clone()
            .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string());
        Self {
            model: cfg.model.clone(),
            endpoint,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }
}

#[derive(Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: i32,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Option<OllamaResponseMessage>,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[async_trait::async_trait]
impl WriterAi for OllamaWriter {
    async fn generate(
        &self,
        req: WriterAiRequest,
    ) -> std::result::Result<String, WriterAiError> {
        let body = OllamaChatRequest {
            model: &self.model,
            messages: vec![OllamaMessage {
                role: "user",
                content: &req.user_prompt,
            }],
            stream: false,
            options: OllamaOptions {
                temperature: req.temperature,
                num_predict: req.max_tokens,
            },
        };

        let url = format!("{}/api/chat", self.endpoint.trim_end_matches('/'));
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| WriterAiError::Network(format!("ollama post: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(WriterAiError::Network(format!(
                "ollama status {status}: {body}"
            )));
        }

        let parsed: OllamaChatResponse = resp
            .json()
            .await
            .map_err(|e| WriterAiError::Other(format!("ollama parse: {e}")))?;

        let text = parsed
            .message
            .map(|m| m.content)
            .unwrap_or_default();

        if text.is_empty() {
            Err(WriterAiError::Empty)
        } else {
            Ok(text)
        }
    }
}
