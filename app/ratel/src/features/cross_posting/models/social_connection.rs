use crate::common::*;
use crate::features::cross_posting::types::SocialPlatform;

/// Per-user, per-platform external account credential. KMS-encrypted.
///
/// Design doc: docs/superpowers/specs/2026-04-28-cross-posting-design.md
/// (`SocialConnection` section). FR-1 #1–#7.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SocialConnection {
    #[dynamo(prefix = "SC", index = "gsi1", name = "find_by_platform", pk)]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // SocialConnection(platform.to_string())

    /// Sparse GSI sort key — `"{platform}#{status}"`. Lets the dispatcher
    /// query "all Connected linkedin users" in one shot.
    #[dynamo(index = "gsi1", sk)]
    pub platform_status: String,

    pub platform: SocialPlatform,
    pub status: ConnectionStatus,

    pub external_handle: String,
    pub external_user_id: String,

    /// KMS-encrypted credential blob. Bluesky: app-password session JWTs.
    /// LinkedIn / Threads: OAuth access + refresh tokens.
    /// Decrypted only inside the dispatcher Lambda.
    pub credential_ciphertext: Vec<u8>,
    pub credential_kms_key_id: String,

    /// `Some` for OAuth tokens (LinkedIn ~60d, Threads long-lived w/ refresh),
    /// `None` for Bluesky app passwords (which do not expire).
    pub token_expires_at: Option<i64>,

    /// Per-platform auto-post toggle (FR-3 #17). Default `true` on connect.
    pub auto_post_enabled: bool,

    /// Cumulative count of syndicated posts (FR-3 #17). Atomic ADD on each
    /// successful Stage 2 dispatch.
    pub posts_syndicated_count: i64,

    pub last_synced_at: Option<i64>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::Display)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ConnectionStatus {
    #[default]
    Connected,
    /// Token expired or refresh failed (FR-5 #35). User must reconnect via
    /// the in-app notification CTA.
    AuthExpired,
    /// User-initiated disconnect (FR-1 #7). Soft delete: row is kept with
    /// `credential_ciphertext` zeroed so historical "posted via …"
    /// rendering on past `SyndicationJob` rows still resolves the handle.
    Revoked,
}
