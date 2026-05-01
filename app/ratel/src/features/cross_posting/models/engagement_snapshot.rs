use crate::common::*;
use crate::features::cross_posting::types::SocialPlatform;

/// Per-(post, platform) engagement counts mirror, refreshed by Stage 4 on an
/// adaptive cadence (1 h while < 24 h old, 6 h up to 7 d, 24 h up to 30 d,
/// then stop). Author-only on read.
///
/// The next-refresh timestamp does **not** live here — it lives on
/// `SyndicationJob.engagement_next_at` because the scheduler walks jobs
/// (sparse GSI on `engagement_shard`), not snapshots.
///
/// Design doc: docs/superpowers/specs/2026-04-28-cross-posting-design.md
/// (`EngagementSnapshot` section). FR-7 #45.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct EngagementSnapshot {
    #[dynamo(prefix = "ES", pk)]
    pub pk: Partition, // Feed(post_id)

    pub sk: EntityType, // EngagementSnapshot(platform.to_string())

    pub platform: SocialPlatform,

    pub likes: i32,
    pub comments: i32,
    pub reposts: i32,

    pub fetched_at: i64,
}
