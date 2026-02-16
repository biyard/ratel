mod firebase_config;

pub use firebase_config::*;

use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct WebConfig {
    pub env: Environment,
    pub log_level: LogLevel,

    pub firebase: FirebaseConfig,
}
