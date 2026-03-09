mod firebase_config;
mod wallet_connect_config;

pub use firebase_config::*;
pub use wallet_connect_config::*;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct WebConfig {
    pub env: Environment,
    pub log_level: LogLevel,

    pub firebase: FirebaseConfig,
    pub wallet_connect: WalletConnectConfig,
}
