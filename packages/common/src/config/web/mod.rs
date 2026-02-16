use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct WebConfig {
    pub env: Environment,
    pub log_level: LogLevel,
}
