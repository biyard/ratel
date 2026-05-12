use crate::common::*;
use crate::features::cross_posting::models::{ConnectionStatus, ErrorCategory, JobState};
use crate::features::cross_posting::types::SocialPlatform;

/// Response shape for connection-listing / mutation endpoints. Excludes
/// every credential-bearing field (FR-1 #6) — `credential_ciphertext`
/// MUST never appear on a response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Author-only post-detail syndication panel data (FR-7 #41–#45).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyndicationPanelResponse {
    pub post_id: FeedPartition,
    pub jobs: Vec<SyndicationJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyndicationJobView {
    pub platform: SocialPlatform,
    pub state: JobState,
    pub external_post_url: Option<String>,
    pub last_error_category: Option<ErrorCategory>,
    /// Human-readable error reason; only populated for `Failed` jobs.
    /// Phase 1 surfaces the platform-side error message verbatim, which
    /// is safe to show to the post's author (the only viewer).
    pub last_error_message: Option<String>,
    pub attempts: u8,
    /// Next scheduled retry time (epoch seconds). 0 when not scheduled.
    pub next_attempt_at: i64,
    /// Engagement counts (likes / comments / reposts) — populated when an
    /// `EngagementSnapshot` row exists for this `(post, platform)`. None
    /// for `Pending` / `Failed` jobs (they were never published).
    pub engagement: Option<EngagementCountsView>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct EngagementCountsView {
    pub likes: i32,
    pub comments: i32,
    pub reposts: i32,
    pub fetched_at: i64,
}

#[cfg(feature = "server")]
impl From<crate::features::cross_posting::models::EngagementSnapshot> for EngagementCountsView {
    fn from(s: crate::features::cross_posting::models::EngagementSnapshot) -> Self {
        Self {
            likes: s.likes,
            comments: s.comments,
            reposts: s.reposts,
            fetched_at: s.fetched_at,
        }
    }
}

#[cfg(feature = "server")]
impl SyndicationJobView {
    /// Build from a `SyndicationJob` and an optional `EngagementSnapshot`
    /// already-loaded by the controller. The internal `external_post_id`
    /// (an `at://` URI for Bluesky) is intentionally not exposed; only
    /// the human-clickable `external_post_url` is.
    pub fn from_job(
        job: crate::features::cross_posting::models::SyndicationJob,
        engagement: Option<EngagementCountsView>,
    ) -> Self {
        Self {
            platform: job.platform,
            state: job.state,
            external_post_url: job.external_post_url,
            last_error_category: job.last_error_category,
            last_error_message: job.last_error_message,
            attempts: job.attempts,
            next_attempt_at: job.next_attempt_at,
            engagement,
        }
    }
}
