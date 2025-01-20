#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct VerifyCryptoStanceRequest {
    pub code: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum VerificationActionRequest {
    CryptoStance(VerifyCryptoStanceRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum VerificationActionResponse {
    ChangeCryptoStance {
        request_id: String,
    },

    #[default]
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationCryptoStance {
    pub id: String,
    pub r#type: String,
    pub code: String,
    pub created_at: u64,
    pub expired_at: u64,
    pub done_at: Option<u64>,
}

impl VerificationCryptoStance {
    pub fn new(id: String, code: String, expire_time: u64) -> Self {
        let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        Self {
            id,
            r#type: "verification_crypto_stance".to_string(),
            code,
            created_at: now,
            expired_at: now + expire_time,
            done_at: None,
        }
    }
}