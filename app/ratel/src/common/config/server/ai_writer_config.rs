use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiWriterKind {
    Aws,
    Ollama,
    #[cfg(feature = "bypass")]
    Fixture,
}

#[derive(Debug, Clone)]
pub struct AiWriterConfig {
    pub kind: AiWriterKind,
    pub model: String,
    /// Optional endpoint override. Used by both Bedrock (e.g. VPC interface
    /// endpoint) and Ollama (`/api/chat` base URL). When `None`, each
    /// backend falls back to its own default.
    pub endpoint: Option<String>,
}

impl Default for AiWriterConfig {
    fn default() -> Self {
        let raw = env::var("RATEL_AI_WRITER_TYPE").ok();
        let kind = match raw.as_deref() {
            Some("ollama") => AiWriterKind::Ollama,
            #[cfg(feature = "bypass")]
            Some("fixture") => AiWriterKind::Fixture,
            _ => AiWriterKind::Aws,
        };

        let model = env::var("RATEL_AI_WRITER_MODEL").unwrap_or_else(|_| match kind {
            AiWriterKind::Aws => "anthropic.claude-sonnet-4-20250514".to_string(),
            AiWriterKind::Ollama => "qwen2.5:3b".to_string(),
            #[cfg(feature = "bypass")]
            AiWriterKind::Fixture => "fixture".to_string(),
        });

        let endpoint = env::var("RATEL_AI_WRITER_ENDPOINT").ok();

        Self {
            kind,
            model,
            endpoint,
        }
    }
}

static AI_WRITER_CONFIG: OnceLock<AiWriterConfig> = OnceLock::new();

/// Lazily-initialized AI writer configuration.
///
/// Read once from env vars on first access and cached for the life of the
/// process. `ServerConfig` exposes this through `.ai_writer()` for
/// consistency with the rest of the config surface; callers that don't
/// already hold a `ServerConfig` can call this directly.
pub fn get() -> &'static AiWriterConfig {
    AI_WRITER_CONFIG.get_or_init(AiWriterConfig::default)
}
