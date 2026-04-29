use crate::common::*;
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::SocialPlatform;

/// Response shape for connection-listing / mutation endpoints. Excludes
/// every credential-bearing field (FR-1 #6) — `credential_ciphertext`
/// MUST never appear on a response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct ConnectionResponse {
    pub platform: SocialPlatform,
    pub status: ConnectionStatus,
    pub external_handle: String,
    pub external_user_id: String,
    pub auto_post_enabled: bool,
    pub posts_syndicated_count: i64,
    pub last_synced_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl From<crate::features::cross_posting::models::SocialConnection> for ConnectionResponse {
    fn from(c: crate::features::cross_posting::models::SocialConnection) -> Self {
        Self {
            platform: c.platform,
            status: c.status,
            external_handle: c.external_handle,
            external_user_id: c.external_user_id,
            auto_post_enabled: c.auto_post_enabled,
            posts_syndicated_count: c.posts_syndicated_count,
            last_synced_at: c.last_synced_at,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}
