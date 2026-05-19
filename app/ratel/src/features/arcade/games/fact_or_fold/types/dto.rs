//! DTOs and shared enums for the Fact or Fold game.
//!
//! PR1 surface: only the admin subject + settings DTOs are populated.
//! Round/lobby/settlement DTOs land alongside PR3+.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

// ── Shared enums ───────────────────────────────────────────────────

/// Verdict assigned by the operator at subject-creation time. Hidden
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

/// Subject lifecycle. `Draft` is creator-editable, `Scheduled` is
/// queued for `scheduled_at`, `Live` means a round is in progress
/// (subject becomes mostly immutable — see §FR-43), `Settled` is
/// post-round and only Reveal sources may grow.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectStatus {
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

// ── Subject DTOs ─────────────────────────────────────────────────

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateSubjectRequest {
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
pub struct PublishSubjectRequest {
    pub scheduled_at: Option<i64>,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubjectRequest {
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
pub struct SubjectResponse {
    pub id: FactFoldSubjectEntityType,
    pub status: SubjectStatus,
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

// ── Round + Lobby (PR3) ───────────────────────────────────────────

/// Round lifecycle. PR3 only orchestrates Waiting → NewsReveal;
/// downstream stages land in PR4 with bets and PR5 with chat.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundStatus {
    /// Lobby is filling up. Joins still accepted.
    #[default]
    Waiting,
    /// Stage 1 — players read the subject. Joins closed.
    NewsReveal,
    /// Stage 2 — first bet. (PR4)
    Bet,
    /// Stage 3 — write rationale. (PR4)
    Rationale,
    /// Stage 4 — show rationales. (PR4)
    Reveal,
    /// Stage 5 — chat + flip window. (PR5)
    Debate,
    /// Stage 6 — payout in flight. (PR6)
    Settlement,
    /// Final state — payouts done, history only.
    Settled,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RoundResponse {
    pub id: FactFoldRoundEntityType,
    pub subject_id: FactFoldSubjectEntityType,
    pub status: RoundStatus,
    /// User ids currently in the round. Order = join order.
    pub participant_pks: Vec<UserPartition>,
    /// Set when the round transitions out of Waiting.
    pub started_at: Option<i64>,
    /// Set when the round reaches Settled.
    pub settled_at: Option<i64>,
    /// Millis-since-epoch when the *current* stage began. None while
    /// the round is Waiting. Drives the client-side countdown.
    pub stage_started_at: Option<i64>,
    /// Millis-since-epoch when the current stage will auto-advance.
    /// Server-verified by [`crate::features::arcade::games::fact_or_fold::services::
    /// stage_machine`] on every round read/write (§FR-9).
    pub stage_deadline_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Snapshot exposed at `GET /api/fact-or-fold/lobby`. Most fields
/// are read-mostly UI hints; the join button uses `can_join` +
/// `pending_user_in_round`.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LobbyResponse {
    /// `Some` when a Waiting round exists with a usable subject.
    pub current_round: Option<RoundResponse>,
    /// True iff the lobby has a current round AND the caller is not
    /// already in it AND there is room for one more.
    pub can_join: bool,
    /// True iff the caller is already in the current round.
    pub already_joined: bool,
    /// Round capacity from settings — UI hint for the "x / capacity"
    /// label. Mirrors `FactFoldSettings::round_capacity`.
    pub round_capacity: i32,
    /// Min RP required to join (FR-23 balance guard) — UI hint for
    /// the "you need N RP" message.
    pub min_bet_rp: i64,
    /// Buy-in chips locked from the wallet at lobby join time.
    /// Mirrors `ArcadeSettings::default_buy_in_chips` so the matching
    /// screen can show the exact escrow amount instead of a hardcoded
    /// number.
    #[serde(default)]
    pub buy_in_chips: i64,
    /// True iff at least one Scheduled subject is due (or already
    /// Live). When `current_round` is None and this is False, the
    /// lobby is closed: the admin needs to publish more subjects.
    pub subject_available: bool,
}

// ── Bet + Rationale + Participant (PR4) ───────────────────────────

/// Which side a player bet on. Mirrors `Verdict` shape; kept
/// separate so future "side options" extensions (e.g. abstain) can
/// land here without mutating the verdict enum used by subjects.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetSide {
    #[default]
    #[serde(rename = "REAL")]
    Real,
    #[serde(rename = "FAKE")]
    Fake,
}

impl From<Verdict> for BetSide {
    fn from(v: Verdict) -> Self {
        match v {
            Verdict::Real => BetSide::Real,
            Verdict::Fake => BetSide::Fake,
        }
    }
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlaceBetRequest {
    pub side: BetSide,
    /// RatelPoints staked. Server validates against
    /// `FactFoldSettings::min_bet_rp..=max_bet_rp`.
    pub amount_rp: i64,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BetResponse {
    pub user_pk: UserPartition,
    pub side: BetSide,
    pub amount_rp: i64,
    pub locked_at: i64,
    /// Set after the §FR-29 last-10s flip — the side the player
    /// switched to.
    pub flipped_to: Option<BetSide>,
    /// User id whose rationale was cited as the flip trigger.
    pub flip_cite_user_pk: Option<UserPartition>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// §FR-16/17 — last-10s bet-change slot. Requires `cite_user_pk`
/// pointing at *another* round participant who has submitted a
/// rationale. Flip is one-shot per player; a player who already
/// flipped this round is rejected.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FlipBetRequest {
    /// Side the player is flipping to. Must differ from the
    /// player's *current* side (the original bet side, or the
    /// already-flipped side if somehow they call this again).
    pub side: BetSide,
    /// Round participant whose rationale drove the flip. §FR-17:
    /// a flip without citation is invalid. §FR-18: settlement
    /// re-verifies the cited user actually submitted a rationale.
    pub cite_user_pk: UserPartition,
}

/// `FlipBetResponse` mirrors `BetResponse` shape — the flip has
/// already been applied so `flipped_to + flip_cite_user_pk` are set.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FlipBetResponse {
    pub user_pk: UserPartition,
    pub original_side: BetSide,
    pub flipped_to: BetSide,
    pub cite_user_pk: UserPartition,
    pub amount_rp: i64,
}

/// §FR-16 — the flip slot is open only during the final
/// `FLIP_SLOT_LAST_MS` of the Debate stage.
pub const FLIP_SLOT_LAST_MS: i64 = 10_000;

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubmitRationaleRequest {
    /// 50–200 chars per spec. Texts shorter than 50 still post but
    /// are flagged `essence_eligible = false` (not promoted).
    pub text: String,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RationaleResponse {
    pub user_pk: UserPartition,
    pub text: String,
    pub submitted_at: i64,
    /// True iff `text.chars().count() >= RATIONALE_ESSENCE_MIN_CHARS`.
    pub essence_eligible: bool,
    /// Set once the player has registered this rationale to their
    /// Essence (PR6 wires the actual upsert).
    pub essence_registered: bool,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub user_pk: UserPartition,
    pub joined_at: i64,
    /// True only on the row returned to the insider themselves;
    /// always false on rows surfaced to other players (insider
    /// protection per design doc §Constraints).
    pub is_insider: bool,
    /// Latest heartbeat — used by the reconnect-grace window.
    pub last_seen_at: i64,
    /// True once the round auto-forfeited the player after the
    /// reconnect grace expired.
    pub forfeited: bool,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InsiderStatementResponse {
    /// `Some(text)` only when the caller is the insider for this
    /// round; `None` for everyone else.
    pub statement: Option<String>,
}

// ── Chat (PR4f) ───────────────────────────────────────────────────

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PostChatRequest {
    /// 80-char cap enforced server-side (roadmap §FR-25).
    pub text: String,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PostChatResponse {
    pub msg_id: String,
    pub author_pk: UserPartition,
    pub text: String,
    pub sent_at: i64,
}

/// Chat-message char cap (roadmap §FR-25).
pub const CHAT_TEXT_MAX_CHARS: usize = 80;

/// Per-page chat message fetch limit for the polling endpoint.
/// 4-player × 70s × 80 chars sessions don't approach this, so a
/// single-page response is the v1 contract.
pub const CHAT_PAGE_LIMIT: usize = 200;

/// Wire-format chat row. Used both by the polling endpoint and by
/// the SSE `chat_message` event payload — they share the same
/// shape on purpose so a v2 transport swap doesn't change the
/// client-side parser.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMessagePayload {
    pub msg_id: String,
    pub author_pk: UserPartition,
    pub text: String,
    pub sent_at: i64,
}

/// Polling response for `GET .../rounds/{id}/chat`.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListChatResponse {
    pub items: Vec<ChatMessagePayload>,
    /// Id of the last message in `items`, or `None` for an empty
    /// page. Clients echo this back via `?since=` on the next poll.
    pub last_id: Option<String>,
}

/// Response from `DELETE .../rounds/{id}/chat` — count of transcript
/// rows actually removed. Idempotent: a re-call after the rows are
/// already gone returns `deleted: 0`.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeleteRoundChatResponse {
    pub deleted: i64,
}

// ── Round-read DTOs (player-side) ─────────────────────────────────
//
// These five GET endpoints feed the game-room views. They mirror the
// admin-side reads where they exist but redact fields that would leak
// the verdict / other-player decisions before the appropriate stage.

/// Public subject shape served to round participants. The verdict
/// + reveal sources are gated by `round.status` — they only fill in
/// once the round reaches `Settled`. The insider statement is **not**
/// in this payload; it has its own per-caller endpoint
/// (`/insider-statement`) so the response can be tailored per user.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RoundSubjectResponse {
    pub id: FactFoldSubjectEntityType,
    pub headline_text: String,
    pub body_excerpt: String,
    pub source_label: String,
    pub category_tags: Vec<String>,
    pub difficulty: i32,
    /// `Some` only once the round is `Settled`. Until then the
    /// verdict is the operator's secret.
    pub verdict: Option<Verdict>,
    /// Empty until `Settled`.
    pub reveal_summary: String,
    /// Empty until `Settled`.
    pub reveal_sources: Vec<RevealSource>,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListBetsResponse {
    /// While the round is at `Bet` or `Rationale`, this list only
    /// contains the caller's own bet (so the UI can confirm what
    /// they staked without leaking opponents' decisions). At
    /// `Reveal` and later it contains every participant's bet.
    pub items: Vec<BetResponse>,
}

/// One rationale row in the round-scoped list endpoint. `text` is
/// redacted to an empty string for non-caller rows until the round
/// reaches `Reveal`; the row still exists so the UI can render the
/// "submitted" pulse during the `Rationale` stage without leaking
/// the actual text.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListRationalesResponse {
    pub items: Vec<RationaleResponse>,
}

/// Per-participant row enriched with display metadata so the UI can
/// render avatars + names without a separate user lookup.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RoundParticipantSummary {
    pub user_pk: UserPartition,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub joined_at: i64,
    pub last_seen_at: i64,
    pub forfeited: bool,
    /// True only on the caller's own row (insider protection — other
    /// players' is_insider flag is never surfaced).
    pub is_insider: bool,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListParticipantsResponse {
    pub items: Vec<RoundParticipantSummary>,
}

// ── Bet + Rationale constants ─────────────────────────────────────

/// Minimum chars for a rationale to count as "Essence-eligible"
/// (per spec §FR-37).
pub const RATIONALE_ESSENCE_MIN_CHARS: usize = 50;
/// Hard upper bound to keep rows small and submissions skim-able.
pub const RATIONALE_TEXT_MAX_CHARS: usize = 200;

// ── Stats + leaderboard (PR7) ─────────────────────────────────────

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserStatsResponse {
    pub user_pk: UserPartition,
    pub total_rounds: i64,
    pub correct_count: i64,
    pub accuracy_bps: i32,
    pub lifetime_delta_chips: i64,
    pub last_played_at: i64,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LeaderboardEntryResponse {
    pub user_pk: UserPartition,
    /// Resolved from the player's User row at read time so the table
    /// can render without an extra round-trip per entry. Empty string
    /// when the user row is missing (defensive — should never happen).
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub profile_url: String,
    pub accuracy_bps: i32,
    pub total_rounds: i64,
    pub correct_count: i64,
    pub lifetime_delta_chips: i64,
    pub last_played_at: i64,
}

// ── Queue health ──────────────────────────────────────────────────

/// Queue depth + FR-45 alert flag for the admin dashboard. Computed
/// over `Scheduled` subjects whose `scheduled_at` is in the future.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueueAlarmResponse {
    /// Days from "now" to the latest scheduled subject. `0.0` when the
    /// queue is empty.
    pub queue_days_remaining: f64,
    /// Snapshot of `FactFoldSettings::queue_low_alert_days` at read time
    /// — the threshold the UI should compare against locally if it
    /// re-renders before the next poll.
    pub alert_threshold_days: i32,
    /// True when `queue_days_remaining <= alert_threshold_days`. The UI
    /// uses this directly to drive a banner.
    pub alert: bool,
    /// Number of `Scheduled` subjects with `scheduled_at >= now`.
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

/// Subject body excerpt length window — roadmap §FR-40.
pub const HEADLINE_BODY_MIN: usize = 200;
pub const HEADLINE_BODY_MAX: usize = 500;
/// Max subject display text length — generous; UI typically renders
/// short copy.
pub const HEADLINE_TEXT_MAX: usize = 200;
/// Difficulty stars 1..=5.
pub const HEADLINE_DIFFICULTY_MIN: i32 = 1;
pub const HEADLINE_DIFFICULTY_MAX: i32 = 5;
/// Reveal sources upper bound (roadmap mentions 2–3 — allow up to 5 for
/// safety once a round has settled and more sources accumulate).
pub const REVEAL_SOURCES_MAX: usize = 5;

// ── Model → DTO conversions ───────────────────────────────────────
//
// Server-only because the source types are DynamoEntity models. Kept
// here next to the DTOs so the wire shape and the conversion stay in
// lockstep when fields evolve.

#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldBet, FactFoldRound,
};

#[cfg(feature = "server")]
impl From<&FactFoldRound> for RoundResponse {
    fn from(row: &FactFoldRound) -> Self {
        let id = row.id().unwrap_or_default();
        RoundResponse {
            id: FactFoldRoundEntityType(id),
            subject_id: FactFoldSubjectEntityType(row.subject_id.clone()),
            status: row.status,
            participant_pks: row
                .participant_pks
                .iter()
                .cloned()
                .map(UserPartition::from)
                .collect(),
            started_at: row.started_at,
            settled_at: row.settled_at,
            stage_started_at: row.stage_started_at,
            stage_deadline_at: row.stage_deadline_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[cfg(feature = "server")]
impl From<&FactFoldBet> for BetResponse {
    fn from(row: &FactFoldBet) -> Self {
        BetResponse {
            user_pk: UserPartition::from(row.user_pk.clone()),
            side: row.side,
            amount_rp: row.amount_rp,
            locked_at: row.locked_at,
            flipped_to: row.flipped_to,
            flip_cite_user_pk: row.flip_cite_user_pk.clone().map(UserPartition::from),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
