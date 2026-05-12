use crate::common::*;
use crate::features::cross_posting::types::SocialPlatform;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Number of shards used for sweeper-Query parallelism on `dispatch_shard` /
/// `engagement_shard` GSIs. **Implementation-fixed; do not change without a
/// re-shard migration** (see Risks → "Re-sharding migration" in the design
/// doc for the dual-read 4 → 32 procedure).
pub const SHARD_COUNT: u32 = 4;

/// Lock TTL for the Stage 2 dispatcher's two-phase commit (seconds). Must
/// exceed the configured Lambda max execution time (currently 30 s) with
/// safety margin. Stealing the lock before TTL elapses risks duplicate
/// external posts; setting it too high slows recovery from a dead Lambda.
pub const LOCK_TTL_SEC: i64 = 60;

/// One row per (post × platform). Tracks the lifecycle of a single
/// syndication attempt: pending → dispatching (lock held) → published /
/// failed / skipped, with retry scheduling on the `dispatch_shard` sparse
/// GSI and engagement polling on the `engagement_shard` sparse GSI.
///
/// Design doc: docs/superpowers/specs/2026-04-28-cross-posting-design.md
/// (`SyndicationJob` section). FR-5 #29–#34, FR-6 #39, FR-7 #45.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SyndicationJob {
    #[dynamo(prefix = "SJ", pk)]
    pub pk: Partition, // Feed(post_id)

    pub sk: EntityType, // SyndicationJob(platform.to_string())

    /// Sparse GSI partition key for the **retry sweeper** (Stage 3, 1D).
    /// Computed via the single shared `services::shard::shard_for(post_id)`
    /// utility (deterministic non-cryptographic hash, never `DefaultHasher`).
    /// Set when the job enters the retry queue (state=Failed & retryable);
    /// cleared (`None`) when the job reaches a terminal state — sparse GSI
    /// drops the row entirely.
    #[dynamo(index = "gsi1", name = "find_due_jobs", pk)]
    pub dispatch_shard: Option<String>,

    /// Sparse GSI partition key for the **engagement refresh sweeper**
    /// (Stage 4, 1D). Same shard derivation as `dispatch_shard`; the
    /// separate field lets the two sweepers scale independently.
    #[dynamo(index = "gsi2", name = "find_due_engagement", pk)]
    pub engagement_shard: Option<String>,

    /// GSI1 sort key (Number type so DynamoDB range comparators (`<=`)
    /// work directly). Only meaningful when `dispatch_shard.is_some()`;
    /// when the shard is `None` (terminal state) this stays at `0` and
    /// the row is absent from the GSI anyway, so the value is never read.
    #[dynamo(index = "gsi1", sk)]
    pub next_attempt_at: i64,

    /// GSI2 sort key. Same `0`-when-shard-is-`None` convention as
    /// `next_attempt_at`.
    #[dynamo(index = "gsi2", sk)]
    pub engagement_next_at: i64,

    /// For fan-out / privacy re-check at dispatch time.
    pub author_user_id: Partition,

    pub platform: SocialPlatform,
    pub state: JobState,

    /// 0..=3. Incremented inside Stage 2 on each terminal-failure write.
    pub attempts: u8,
    pub last_error_category: Option<ErrorCategory>,
    pub last_error_message: Option<String>,

    /// Platform-side identifiers (Published only).
    pub external_post_id: Option<String>,
    pub external_post_url: Option<String>,

    // body_override: Option<String>      // RESERVED for v1.5 (per-network compose variants).
    //                                     // Stage 2 dispatcher reads this in front of
    //                                     // format_for_platform() — Phase 1 always None,
    //                                     // v1.5 fills from PostSyndicationDirective.
    //                                     // Field will be added without backfill (None on
    //                                     // legacy rows) so Phase 1 → 1.5 is non-breaking.

    /// Length of the syndicated body (in characters). Body content itself is
    /// **never** persisted on the job — privacy / log-redaction guard
    /// (FR-10 #53). The length is kept for observability only.
    pub body_snapshot_len: i32,

    /// Backlink URL (`{canonical_url}?utm_source={platform}`) baked at
    /// Stage 1 enqueue. Never mutated downstream — used by both the publish
    /// path and the `find_by_backlink` lock-recovery probe.
    pub backlink_url: String,

    /// Idempotency lock for in-flight Stage 2 dispatch. Set to a fresh UUID
    /// before the platform API call; cleared after success / failure write.
    /// A second Lambda invocation observing
    /// `dispatch_lock_id.is_some() && lock_acquired_at + LOCK_TTL_SEC > now`
    /// MUST exit without calling the platform — prevents duplicate posts on
    /// Lambda retry between API success and DB write.
    pub dispatch_lock_id: Option<String>,
    pub lock_acquired_at: Option<i64>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum JobState {
    #[default]
    Pending,
    Published,
    Failed,
    /// Privacy guard tripped at dispatch (post became Private / team-shared
    /// after enqueue). FR-6 #39–#40.
    Skipped,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCategory {
    #[default]
    Unknown,
    /// Non-retryable. Surface "Reconnect" CTA via inbox notification.
    AuthExpired,
    /// Retryable with backoff.
    RateLimited,
    /// Non-retryable. Platform rejected content (length, policy, etc.).
    ContentRejected,
    /// Retryable with backoff.
    NetworkError,
}
