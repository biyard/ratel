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

