use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::{RevealSource, SubjectStatus, Verdict};

#[allow(unused_imports)]
use rmcp::schemars;

/// Single subject = single round's content (a news item players will
/// judge as Real or Fake). Stored under the shared anchor pk
/// `Partition::FactFoldSubjects` with a sk carrying the subject id
/// (`EntityType::FactFoldSubject(subject_id)`) so a single `query` on
/// the anchor pk lists every subject.
///
/// Lifecycle: Draft → Scheduled (optional) → Live (round in progress)
/// → Settled. Once Live, only `reveal_sources` may grow (roadmap §FR-43).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldSubject {
    pub pk: Partition,  // Partition::FactFoldSubjects
    pub sk: EntityType, // EntityType::FactFoldSubject(subject_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Operator who authored this subject.
    pub creator_pk: Partition,

    /// Status drives the GSI3 partition: `pick_next_subject` queries
    /// `Live` (already-activated, FIFO-pickable) and `Scheduled`
    /// (queued for a future timestamp) partitions independently, so
    /// the per-status FIFO is computed at index time rather than in
    /// memory.
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "FFS", name = "find_by_status", index = "gsi3", pk)
    )]
    pub status: SubjectStatus,

    pub verdict: Verdict,

    pub headline_text: String,
    pub body_excerpt: String,

    /// Difficulty stars 1..=5.
    pub difficulty: i32,

    #[serde(default)]
    pub category_tags: Vec<String>,

    pub source_label: String,

    /// Single private statement delivered to the insider. v1 has no
    /// "possibly-false" mafia statement — see roadmap §FR-26.
    pub insider_statement: String,

    pub reveal_summary: String,

    #[serde(default)]
    pub reveal_sources: Vec<RevealSource>,

    /// Millis since epoch; None for plain drafts. When Some, the
    /// scheduler activates the subject as the live round at this time.
    pub scheduled_at: Option<i64>,

    /// Materialized sort key for FIFO picking — equal to
    /// `scheduled_at.unwrap_or(created_at)`. Lives on the row as a
    /// real i64 (not derived per query) so the GSI3 sort key sees a
    /// stable, indexable column. Update sites that change
    /// `scheduled_at` must also update this field.
    #[serde(default)]
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "PICK", index = "gsi3", sk)
    )]
    pub pick_at: i64,

    /// Millis since epoch when the subject's active window ends.
    /// `pick_next_subject` filters by `now ∈ [scheduled_at, expires_at]`
    /// — a subject past `expires_at` is auto-retired and never picked
    /// again. `0` means "not yet set" (legacy rows or drafts without a
    /// window); pick logic treats `0` as an indefinite/no-expiry value
    /// so existing data keeps the prior "until manually settled" semantics.
    #[serde(default)]
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl FactFoldSubject {
    pub fn anchor_pk() -> Partition {
        Partition::FactFoldSubjects
    }

    pub fn keys(subject_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFoldSubjects,
            EntityType::FactFoldSubject(subject_id.to_string()),
        )
    }

    /// Build a fresh draft from a creation request. Caller is responsible
    /// for validation and uniqueness check; this only assembles the row.
    pub fn new_draft(
        subject_id: String,
        creator_pk: Partition,
        headline_text: String,
        body_excerpt: String,
        verdict: Verdict,
        difficulty: i32,
        category_tags: Vec<String>,
        source_label: String,
        insider_statement: String,
        reveal_summary: String,
        reveal_sources: Vec<RevealSource>,
        scheduled_at: Option<i64>,
        expires_at: i64,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let status = if scheduled_at.is_some() {
            SubjectStatus::Scheduled
        } else {
            SubjectStatus::Draft
        };
        let pick_at = scheduled_at.unwrap_or(now);
        Self {
            pk: Partition::FactFoldSubjects,
            sk: EntityType::FactFoldSubject(subject_id),
            created_at: now,
            updated_at: now,
            creator_pk,
            status,
            verdict,
            headline_text,
            body_excerpt,
            difficulty,
            category_tags,
            source_label,
            insider_statement,
            reveal_summary,
            reveal_sources,
            scheduled_at,
            pick_at,
            expires_at,
        }
    }

    /// Inner id encoded in the sort key.
    pub fn id(&self) -> Option<String> {
        match &self.sk {
            EntityType::FactFoldSubject(id) => Some(id.clone()),
            _ => None,
        }
    }

    /// True once a round has been started against this subject; mutation
    /// rules tighten (roadmap §FR-43).
    pub fn is_locked(&self) -> bool {
        matches!(self.status, SubjectStatus::Live | SubjectStatus::Settled)
    }
}
