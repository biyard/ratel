//! Analysis result DTOs.
//!
//! Two distinct result paths feed the detail page:
//!
//! 1. **Auto** — populated by the DDB-stream Lambda the moment a new
//!    `AnalyzeReport` is inserted with `status=InProgress`. Aggregates
//!    poll/quiz/follow behaviour for the report's matched respondents
//!    (intersection of every filter chip's user set). Stored on a
//!    separate row (`SpaceAnalyzeReportResult`) keyed 1:1 by report id.
//!
//! 2. **On-demand** — populated by the user pressing 확인 on the
//!    discussion analysis form. The POST creates a new
//!    `SpaceAnalyzeDiscussionResult` row (with a UUIDv7 in the sk so
//!    history is preserved + naturally time-sorted) carrying only the
//!    user-supplied params; a separate stream Lambda picks up the
//!    INSERT, runs the LDA / TF-IDF / text-network pipeline, then
//!    overwrites the same row with the computed result fields.

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use schemars::JsonSchema;

/// Per-option tally for a poll/quiz question. The order in
/// `option_labels` is preserved from the underlying question — index
/// `i` in `count` corresponds to `option_labels[i]`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct OptionTally {
    pub label: String,
    pub count: u32,
}

/// Aggregate for one question of a poll the report's matched users
/// answered. `respondent_count` is how many matched users responded to
/// THIS specific question (not the whole poll), so percentages should
/// always be computed against this denominator on the client.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct PollQuestionAggregate {
    pub poll_id: String,
    pub poll_title: String,
    pub question_idx: usize,
    pub question_title: String,
    pub options: Vec<OptionTally>,
    pub respondent_count: u32,
    /// Free-text answers (only set when the underlying question is
    /// short/long-answer). Empty for choice-style questions.
    #[serde(default)]
    pub text_answers: Vec<String>,
}

/// Same shape as `PollQuestionAggregate` plus the correct option index
/// set so the panel can paint correct/incorrect bars.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct QuizQuestionAggregate {
    pub quiz_id: String,
    pub quiz_title: String,
    pub question_idx: usize,
    pub question_title: String,
    pub options: Vec<OptionTally>,
    /// Indices of the correct options for this question. May be empty
    /// for short-answer-style quiz questions.
    pub correct_indices: Vec<u32>,
    /// Number of matched users who picked at least one correct option
    /// on this question. Out of `respondent_count`.
    pub correct_count: u32,
    pub respondent_count: u32,
    #[serde(default)]
    pub text_answers: Vec<String>,
}

/// Top-N follow targets among the report's matched users, ordered by
/// count descending. `count` is "how many of the matched users follow
/// this target".
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct FollowTargetAggregate {
    pub user_pk: String,
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
    pub count: u32,
}

// ── Discussion-side results (LDA / TF-IDF / text-network) ──────────

/// One LDA topic row for the result panel. Topic indices are 1-based
/// in the label (`토픽_1`) for direct rendering parity with the mock.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct TopicRow {
    pub topic: String,
    /// Top-N keywords for this topic, joined into a single comma-sep
    /// string so the panel can render it without further client-side
    /// glue. Stored as a list to keep individual keywords inspectable.
    pub keywords: Vec<String>,
}

/// One TF-IDF row. `score` is the raw TF-IDF weight summed across the
/// matched-user discussion corpus; `relative` is `score / max_score`
/// (already normalised) so the bar widths are direct percentages.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct TermScore {
    pub term: String,
    pub score: f64,
    pub relative: f64,
}

/// Co-occurrence graph node. `weight` is term frequency in the corpus.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct NetworkNode {
    pub term: String,
    pub weight: u32,
}

/// Co-occurrence graph edge between two nodes.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct NetworkEdge {
    pub source: String,
    pub target: String,
    pub weight: u32,
}

/// User-supplied parameters for the discussion analysis run. Mirrors
/// the four inputs in the detail page's "분석 설정" form.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
pub struct DiscussionAnalysisParams {
    /// Number of LDA topics. Bound 1..=20 on the form.
    pub num_topics: usize,
    /// How many TF-IDF terms to keep. Bound 1..=20.
    pub top_n_tfidf: usize,
    /// How many network nodes to keep (top-N by weight). Bound 1..=30.
    pub top_n_network: usize,
    /// Comma-separated keywords the user wants excluded from results.
    /// Applied as an extra stop-word layer over the corpus.
    pub excluded_keywords: Vec<String>,
}

impl DiscussionAnalysisParams {
    pub fn validate(&self) -> bool {
        (1..=20).contains(&self.num_topics)
            && (1..=20).contains(&self.top_n_tfidf)
            && (1..=30).contains(&self.top_n_network)
    }
}
