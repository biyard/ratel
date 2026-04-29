//! Report DTOs and helpers for the analyzes arena LIST view.
//!
//! Status enum doubles as the DynamoDB-stored value on `SpaceAnalyzeReport`
//! and the view-side status badge driver. The lowercase variants returned
//! by `as_str()` feed `data-status` on the HTML mockup so the existing
//! CSS picks up the badge styling unchanged.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Lifecycle of a saved analyze report. `InProgress` is the initial state
/// after submit — DynamoDB stream pipelines (LDA / TF-IDF / poll-quiz
/// aggregation) flip it to `Finish` once analysis completes, or `Failed`
/// on irrecoverable error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum AnalyzeReportStatus {
    #[default]
    InProgress,
    Finish,
    Failed,
}

impl AnalyzeReportStatus {
    /// Lowercase string used in the `data-status` attribute. Kept aligned
    /// with the HTML mockup CSS — `running` for in-progress, `done` for
    /// finish — so existing styling renders without changes.
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalyzeReportStatus::InProgress => "running",
            AnalyzeReportStatus::Finish => "done",
            AnalyzeReportStatus::Failed => "failed",
        }
    }
}

/// Source of a filter chip — i.e. which action type produced it.
///
/// Determines the chip palette via `data-source` (cyan for poll, purple
/// for quiz, blue for discussion, orange for follow). The lowercase
/// string from `Display` is what the CSS selector matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum AnalyzeFilterSource {
    #[default]
    Poll,
    Quiz,
    Discussion,
    Follow,
}

impl AnalyzeFilterSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "poll",
            AnalyzeFilterSource::Quiz => "quiz",
            AnalyzeFilterSource::Discussion => "discussion",
            AnalyzeFilterSource::Follow => "follow",
        }
    }
}

impl fmt::Display for AnalyzeFilterSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One filter chip on a saved report card.
///
/// The mockup distinguishes a coloured `source` capsule (POLL / QUIZ /
/// DISCUSSION / FOLLOW) from the longer `label` body that names the
/// specific question or option. `item_id` / `question_id` / `option_id`
/// are stored verbatim so the detail page can re-run the same matching
/// logic against live response data without re-deriving anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AnalyzeReportFilter {
    pub source: AnalyzeFilterSource,
    pub source_label: String,
    pub label: String,
    pub item_id: String,
    pub question_id: String,
    pub option_id: String,
    pub option_text: String,
    pub question_title: String,
    pub correct: bool,
}

/// One saved analysis as displayed in the list carousel and returned by
/// `GET /api/spaces/{space_id}/apps/analyzes/reports`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AnalyzeReport {
    pub id: String,
    pub name: String,
    pub status: AnalyzeReportStatus,
    /// Unix timestamp in milliseconds. View-side date / time formatting
    /// happens at render time so the wire format stays a single field.
    pub created_at: i64,
    pub filters: Vec<AnalyzeReportFilter>,
}

/// One hydrated row of source data, flat across every source. The
/// consumer dispatches on `source` and reads only the relevant
/// fields; unused fields stay empty strings. Kept flat (vs. tagged
/// enum) so the wire format is one stable JSON shape that the
/// records page can pass through without per-source decoding.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeRecordRow {
    pub source: AnalyzeFilterSource,
    pub filter_idx: u32,

    pub user_pk: String,
    pub user_username: String,
    pub user_display_name: String,
    pub user_profile_url: String,

    /// Poll/Quiz: question title. Empty for discussion/follow.
    #[serde(default)]
    pub question_text: String,
    /// Poll/Quiz: selected option text. Empty for discussion/follow.
    #[serde(default)]
    pub answer_text: String,

    /// Discussion: parent post title. Empty otherwise.
    #[serde(default)]
    pub post_title: String,
    /// Discussion: comment body. Empty otherwise.
    #[serde(default)]
    pub comment_text: String,

    /// Follow: target user pk string ("USER#..." / "TEAM#..."). Empty otherwise.
    #[serde(default)]
    pub target_pk: String,
    /// Follow: target username. Empty otherwise.
    #[serde(default)]
    pub target_username: String,
    /// Follow: target display name. Empty otherwise.
    #[serde(default)]
    pub target_display_name: String,
}

/// Frozen pointer to one underlying record that matched a filter chip
/// at report-save time. The detail "사용된 데이터 확인하기" page hydrates
/// these refs against the live source entities (poll questions, quiz
/// attempts, comments, follow edges) — so if the original poll question
/// is later edited the displayed text follows, but the *which row*
/// pointer never moves.
///
/// `filter_idx` indexes back into the parent report's `filters` vec so
/// the records page can group rows under their originating chip.
/// Source-specific fields are flattened into one struct (vs. an enum)
/// so DDB serialization stays a plain JSON object — DynamoEntity's derive
/// chokes on enum variants with named fields. Unused fields per source
/// stay empty strings; the consumer dispatches on `source`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MatchedRecordRef {
    pub source: AnalyzeFilterSource,
    pub filter_idx: u32,
    /// User who produced this record. For follow chips, this is the
    /// follower (their action of following the target user).
    pub user_pk: String,
    /// Source-side primary key of the row, stringified. Poll/quiz: the
    /// `SpacePoll` / `SpaceQuiz` entity-type id. Discussion: the post
    /// partition. Follow: the target user partition.
    pub item_id: String,
    /// Source-side sub-id when the record sits inside a multi-row
    /// container — e.g. comment sk for discussion. Empty for the rest.
    #[serde(default)]
    pub sub_id: String,
    /// Question index within the poll/quiz answers vec. `0` for
    /// non-poll/quiz sources.
    #[serde(default)]
    pub question_idx: u32,
    /// Selected option index for poll/quiz. `0` for non-poll/quiz.
    #[serde(default)]
    pub option_idx: u32,
}

