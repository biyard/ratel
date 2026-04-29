//! User-triggered text-analysis result for one discussion under one
//! report. History is preserved — every `확인` press creates a new row
//! (UUIDv7 in the sk keeps them naturally time-ordered). The detail
//! page reads the LATEST row per (report_id, discussion_id) for normal
//! rendering; the full history list is available for an "이전 분석"
//! drawer if/when the UI needs it.
//!
//! Storage shape:
//! - `pk`: Space partition (same as parent report)
//! - `sk`: `SpaceAnalyzeDiscussionResult#{report_id}#{discussion_id}#{request_uuid}`
//!
//! Lifecycle:
//! 1. POST `/analyze_discussion` creates the row with `status=InProgress`
//!    and the user-supplied `params`. Result fields stay default.
//! 2. DDB stream INSERT → Lambda runs lindera + Gibbs LDA + TF-IDF +
//!    text-network → updates the SAME row with results and
//!    `status=Finish`. Filter on INSERT keeps Lambda's own update
//!    from re-triggering itself.
//! 3. Frontend refetches by sk-prefix-query (latest first) to render.

use crate::features::spaces::pages::apps::apps::analyzes::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct SpaceAnalyzeDiscussionResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub status: AnalyzeReportStatus,

    /// Parent report id. Plain string (no prefix) so this row is
    /// queryable independently of how the sk is parsed.
    #[serde(default)]
    pub report_id: String,
    #[serde(default)]
    pub discussion_id: String,
    /// UUIDv7 — also embedded in sk, surfaced here for direct access.
    #[serde(default)]
    pub request_id: String,

    /// User-supplied analysis parameters.
    #[serde(default)]
    pub params: DiscussionAnalysisParams,

    /// How many comments fed the analysis (matched-user comments on
    /// the target discussion). 0 while `status == InProgress`.
    #[serde(default)]
    pub analyzed_comment_count: i64,

    // ── Results — empty until status flips to Finish ──────────────
    #[serde(default)]
    pub topics: Vec<TopicRow>,
    #[serde(default)]
    pub tfidf_terms: Vec<TermScore>,
    #[serde(default)]
    pub network_nodes: Vec<NetworkNode>,
    #[serde(default)]
    pub network_edges: Vec<NetworkEdge>,
}

#[cfg(feature = "server")]
impl SpaceAnalyzeDiscussionResult {
    /// Builds a fresh "queued" row from a discussion analyse request.
    /// The Lambda fills the result fields and flips `status`.
    pub fn new(
        space_pk: Partition,
        report_id: String,
        discussion_id: String,
        params: DiscussionAnalysisParams,
    ) -> Self {
        use crate::common::utils::time::get_now_timestamp_millis;

        let now = get_now_timestamp_millis();
        let request_id = uuid::Uuid::now_v7().to_string();
        // EntityType variant accepts at most 2 unnamed fields, so the
        // (discussion_id, request_uuid) pair is packed into the second
        // slot as `"{discussion_id}#{request_uuid}"`. The natural sk
        // string still ends up `…#{report_id}#{discussion_id}#{uuid}`,
        // which keeps `begins_with` queries by (report, discussion)
        // intact and lets UUIDv7's lexicographic order surface latest
        // first when sorting sk-desc.
        let sk = EntityType::SpaceAnalyzeDiscussionResult(
            report_id.clone(),
            format!("{}#{}", discussion_id, request_id),
        );

        Self {
            pk: space_pk,
            sk,
            created_at: now,
            updated_at: now,
            status: AnalyzeReportStatus::InProgress,
            report_id,
            discussion_id,
            request_id,
            params,
            analyzed_comment_count: 0,
            topics: Vec::new(),
            tfidf_terms: Vec::new(),
            network_nodes: Vec::new(),
            network_edges: Vec::new(),
        }
    }
}
