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
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let status = if scheduled_at.is_some() {
            SubjectStatus::Scheduled
        } else {
            SubjectStatus::Draft
        };
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
