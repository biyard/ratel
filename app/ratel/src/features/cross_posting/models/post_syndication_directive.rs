use crate::common::*;
use crate::features::cross_posting::types::SocialPlatform;
use std::collections::HashMap;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Sidecar entity that carries per-post cross-posting intent **without**
/// polluting the canonical `Post` entity with per-platform fields.
///
/// Written inside the existing `update_post_handler` Publish branch, in the
/// same `transact_write_items!` batch as the `Post` updater (atomic).
/// Read by Stage 1 Lambda when `Post.status` transitions Draft → Published.
///
/// Design doc: docs/superpowers/specs/2026-04-28-cross-posting-design.md
/// (`PostSyndicationDirective` section).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct PostSyndicationDirective {
    #[dynamo(prefix = "PSD", pk)]
    pub pk: Partition, // Feed(post_id)

    pub sk: EntityType, // SyndicationDirective (singleton per post)

    pub enabled_platforms: Vec<SocialPlatform>,

    /// Phase 1: always empty (UI does not expose per-network compose
    /// variants). v1.5 fills this; Stage 1 factory pipes the value through
    /// to `SyndicationJob.body_override`.
    pub platform_overrides: HashMap<SocialPlatform, String>,

    /// User who published the post; Stage 1 reads this to resolve the
    /// matching SocialConnections.
    pub author_user_id: Partition,

    pub created_at: i64,
}
