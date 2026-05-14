//! DTOs and shared enums for the Fact or Fold game.
//!
//! PR1 surface: only the admin headline + settings DTOs are populated.
//! Round/lobby/settlement DTOs land alongside PR3+.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

// ── Shared enums ───────────────────────────────────────────────────

/// Verdict assigned by the operator at headline-creation time. Hidden
/// from participants until settlement.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    #[default]
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "FAKE")]
    Fake,
}

/// Headline lifecycle. `Draft` is creator-editable, `Scheduled` is
/// queued for `scheduled_at`, `Live` means a round is in progress
/// (headline becomes mostly immutable — see §FR-43), `Settled` is
/// post-round and only Reveal sources may grow.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadlineStatus {
    #[default]
    Draft,
    Scheduled,
    Live,
    Settled,
    Deleted,
}

/// A single reveal source surfaced to all players after settlement.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevealSource {
    /// Display label, e.g. "한은 보도자료".
    pub label: String,
    /// Absolute URL of the verification source.
    pub url: String,
}

// ── Headline DTOs ─────────────────────────────────────────────────

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateHeadlineRequest {
    pub headline_text: String,
    pub body_excerpt: String,
    pub verdict: Verdict,
    /// Difficulty 1..=5.
    pub difficulty: i32,
    /// Free-form category tags (e.g. "경제", "정치").
    #[serde(default)]
    pub category_tags: Vec<String>,
    pub source_label: String,
    /// Private truth-statement delivered to the insider at round start.
    /// v1 collects exactly one — see roadmap §FR-26 (no "possibly-false"
    /// statement, no mafia mode).
    pub insider_statement: String,
    /// Plain-text summary shown to all players at settlement.
    pub reveal_summary: String,
    /// 2–3 verification source links shown at settlement.
    #[serde(default)]
    pub reveal_sources: Vec<RevealSource>,
    /// Optional millis timestamp. None = save as draft. Some = scheduled
    /// publication.
    pub scheduled_at: Option<i64>,
}

/// Two modes (roadmap §FR-42):
///   - `scheduled_at: Some(ts)` → set to Scheduled at that time
///   - `scheduled_at: None`     → publish now (Live immediately)
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PublishHeadlineRequest {
    pub scheduled_at: Option<i64>,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateHeadlineRequest {
    pub headline_text: Option<String>,
    pub body_excerpt: Option<String>,
    pub verdict: Option<Verdict>,
    pub difficulty: Option<i32>,
    pub category_tags: Option<Vec<String>>,
    pub source_label: Option<String>,
    pub insider_statement: Option<String>,
    pub reveal_summary: Option<String>,
    pub reveal_sources: Option<Vec<RevealSource>>,
    pub scheduled_at: Option<i64>,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HeadlineResponse {
    pub id: FactFoldHeadlineEntityType,
    pub status: HeadlineStatus,
    pub headline_text: String,
    pub body_excerpt: String,
    pub verdict: Verdict,
    pub difficulty: i32,
    pub category_tags: Vec<String>,
    pub source_label: String,
    pub insider_statement: String,
    pub reveal_summary: String,
    pub reveal_sources: Vec<RevealSource>,
    pub scheduled_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

// ── Queue health ──────────────────────────────────────────────────

/// Queue depth + FR-45 alert flag for the admin dashboard. Computed
/// over `Scheduled` headlines whose `scheduled_at` is in the future.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueueAlarmResponse {
    /// Days from "now" to the latest scheduled headline. `0.0` when the
    /// queue is empty.
    pub queue_days_remaining: f64,
    /// Snapshot of `FactFoldSettings::queue_low_alert_days` at read time
    /// — the threshold the UI should compare against locally if it
    /// re-renders before the next poll.
    pub alert_threshold_days: i32,
    /// True when `queue_days_remaining <= alert_threshold_days`. The UI
    /// uses this directly to drive a banner.
    pub alert: bool,
    /// Number of `Scheduled` headlines with `scheduled_at >= now`.
    pub scheduled_future_count: i32,
}

// ── Settings DTOs ─────────────────────────────────────────────────

/// Admin-tunable parameters mirrored from roadmap §"Tunable parameters".
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactOrFoldSettingsResponse {
    pub round_capacity: i32,
    pub stage_news_reveal_sec: i32,
    pub stage_bet_sec: i32,
    pub stage_rationale_sec: i32,
    pub stage_reveal_sec: i32,
    pub stage_debate_sec: i32,
    pub min_bet_rp: i64,
    pub max_bet_rp: i64,
    /// Stored as basis points (10000 = 1.0×) to keep DDB numeric and
    /// avoid float drift. UI converts to/from the human "1.6×" label.
    pub correct_side_multiplier_bps: i32,
    /// Insider correct bonus (× stake). bps.
    pub insider_correct_bonus_bps: i32,
    /// Flip-citation influence bonus rate. bps.
    pub influence_bonus_bps: i32,
    pub new_user_signup_rp: i64,
    pub reconnect_grace_sec: i32,
    pub queue_low_alert_days: i32,
}

impl Default for FactOrFoldSettingsResponse {
    fn default() -> Self {
        Self {
            round_capacity: 4,
            stage_news_reveal_sec: 30,
            stage_bet_sec: 10,
            stage_rationale_sec: 30,
            stage_reveal_sec: 20,
            stage_debate_sec: 70,
            min_bet_rp: 100,
            max_bet_rp: 1000,
            correct_side_multiplier_bps: 16_000, // 1.6×
            insider_correct_bonus_bps: 5_000,    // 0.5×
            influence_bonus_bps: 3_000,          // 0.3×
            new_user_signup_rp: 5_000,
            reconnect_grace_sec: 90,
            queue_low_alert_days: 5,
        }
    }
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateFactOrFoldSettingsRequest {
    pub round_capacity: Option<i32>,
    pub stage_news_reveal_sec: Option<i32>,
    pub stage_bet_sec: Option<i32>,
    pub stage_rationale_sec: Option<i32>,
    pub stage_reveal_sec: Option<i32>,
    pub stage_debate_sec: Option<i32>,
    pub min_bet_rp: Option<i64>,
    pub max_bet_rp: Option<i64>,
    pub correct_side_multiplier_bps: Option<i32>,
    pub insider_correct_bonus_bps: Option<i32>,
    pub influence_bonus_bps: Option<i32>,
    pub new_user_signup_rp: Option<i64>,
    pub reconnect_grace_sec: Option<i32>,
    pub queue_low_alert_days: Option<i32>,
}

// ── Constants ─────────────────────────────────────────────────────

/// Headline body excerpt length window — roadmap §FR-40.
pub const HEADLINE_BODY_MIN: usize = 200;
pub const HEADLINE_BODY_MAX: usize = 500;
/// Max headline display text length — generous; UI typically renders
/// short copy.
pub const HEADLINE_TEXT_MAX: usize = 200;
/// Difficulty stars 1..=5.
pub const HEADLINE_DIFFICULTY_MIN: i32 = 1;
pub const HEADLINE_DIFFICULTY_MAX: i32 = 5;
/// Reveal sources upper bound (roadmap mentions 2–3 — allow up to 5 for
/// safety once a round has settled and more sources accumulate).
pub const REVEAL_SOURCES_MAX: usize = 5;
