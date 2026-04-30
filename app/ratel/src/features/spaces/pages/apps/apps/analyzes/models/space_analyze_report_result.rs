//! Auto-computed analysis result for a saved AnalyzeReport.
//!
//! Created exclusively by the DDB-stream Lambda when an AnalyzeReport is
//! INSERTED with `status=InProgress`. Holds the aggregations the user
//! sees on the detail page's poll / quiz / follow panels — everything
//! the report's matched-user set did across those three sources.
//!
//! Discussion analysis lives on a separate row
//! (`SpaceAnalyzeDiscussionResult`) because it is user-triggered,
//! per-discussion, and history-preserving.
//!
//! Storage shape:
//! - `pk`: Space partition (same as parent report)
//! - `sk`: `SpaceAnalyzeReportResult#{report_id}` (1:1 with the report)
//!
//! No `status` field — the existence of this row IS the "auto analysis
//! done" signal. Lambda creates this row, then flips
//! `AnalyzeReport.status` to `Finish`.

use crate::features::spaces::pages::apps::apps::analyzes::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct SpaceAnalyzeReportResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    /// Plain report id (no prefix) so consumers can look up the parent
    /// `SpaceAnalyzeReport` row directly without re-parsing the sk.
    #[serde(default)]
    pub report_id: String,

    /// Cached intersection size — exactly the same number the preview
    /// API would have returned for this report's filters at submit
    /// time. Stored so the detail page's banner can render without an
    /// extra round trip.
    #[serde(default)]
    pub respondent_count: i64,

    /// One row per (poll, question) the matched users answered. Same
    /// poll appears multiple times if it has multiple analysed
    /// questions — the panel groups by `poll_id` for rendering.
    #[serde(default)]
    pub poll_aggregates: Vec<PollQuestionAggregate>,

    /// Same shape as `poll_aggregates` but with quiz correctness data.
    #[serde(default)]
    pub quiz_aggregates: Vec<QuizQuestionAggregate>,

    /// Top-N followed targets among the matched users.
    #[serde(default)]
    pub follow_aggregates: Vec<FollowTargetAggregate>,
}

#[cfg(feature = "server")]
impl SpaceAnalyzeReportResult {
    pub fn new(space_pk: Partition, report_id: String) -> Self {
        use crate::common::utils::time::get_now_timestamp_millis;

        let now = get_now_timestamp_millis();
        let sk = EntityType::SpaceAnalyzeReportResult(report_id.clone());

        Self {
            pk: space_pk,
            sk,
            created_at: now,
            updated_at: now,
            report_id,
            respondent_count: 0,
            poll_aggregates: Vec::new(),
            quiz_aggregates: Vec::new(),
            follow_aggregates: Vec::new(),
        }
    }

    pub fn keys(space_pk: Partition, report_id: String) -> (Partition, EntityType) {
        (space_pk, EntityType::SpaceAnalyzeReportResult(report_id))
    }
}
