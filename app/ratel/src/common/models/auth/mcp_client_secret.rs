use crate::common::*;

/// Stores a per-user MCP client secret for authenticating MCP tool calls.
///
/// The `secret` field stores a SHA-256 hash of the raw token. The raw token
/// is returned to the user only once at generation time and is never persisted.
///
/// - pk: USER#<user_id>
/// - sk: McpClientSecret
/// - gsi1 pk: MCS#<hashed_secret> (for lookup by hashed secret)
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct McpClientSecret {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "MCS", name = "find_by_secret", index = "gsi1", pk)]
    pub secret: String,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl McpClientSecret {
    /// Create a new McpClientSecret. Returns `(entity, raw_token)` where
    /// `raw_token` is the plaintext secret to show the user once.
    /// The entity stores only the SHA-256 hash.
    pub fn new(user_pk: Partition) -> (Self, String) {
        use base64::Engine;
        use sha2::Digest;

        let mut buf = [0u8; 32];
        rand::fill(&mut buf);
        let raw_token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf);

        let hash = sha2::Sha256::digest(raw_token.as_bytes());
        let hashed = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);

        let now = chrono::Utc::now().timestamp_millis();

        let entity = Self {
            pk: user_pk,
            sk: EntityType::McpClientSecret,
            secret: hashed,
            created_at: now,
        };

        (entity, raw_token)
    }

    /// Hash a raw token for lookup.
    pub fn hash_secret(raw_token: &str) -> String {
        use base64::Engine;
        use sha2::Digest;

        let hash = sha2::Sha256::digest(raw_token.as_bytes());
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
    }
}
