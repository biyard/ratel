use std::sync::OnceLock;

use crate::common::config::ai_writer_config::{self, AiWriterKind};

#[async_trait::async_trait]
pub trait WriterAi: Send + Sync + 'static {
    async fn generate(
        &self,
        req: WriterAiRequest,
    ) -> std::result::Result<String, WriterAiError>;
}

#[derive(Debug, Clone)]
pub struct WriterAiRequest {
    pub user_prompt: String,
    pub max_tokens: i32,
    pub temperature: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum WriterAiError {
    #[error("ai backend network failure: {0}")]
    Network(String),
    #[error("ai backend returned empty response")]
    Empty,
    #[error("ai backend other failure: {0}")]
    Other(String),
}

static WRITER_AI: OnceLock<Box<dyn WriterAi>> = OnceLock::new();

pub fn writer_ai() -> &'static dyn WriterAi {
    WRITER_AI
        .get_or_init(|| -> Box<dyn WriterAi> {
            let cfg = ai_writer_config::get();
            match cfg.kind {
                AiWriterKind::Aws => Box::new(super::backends::BedrockWriter::from_config(cfg)),
                AiWriterKind::Ollama => Box::new(super::backends::OllamaWriter::from_config(cfg)),
                #[cfg(feature = "bypass")]
                AiWriterKind::Fixture => Box::new(super::backends::FixtureWriter::default()),
            }
        })
        .as_ref()
}
