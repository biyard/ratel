use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::{HeadlineStatus, RevealSource, Verdict};

#[allow(unused_imports)]
use rmcp::schemars;

/// Single headline = single round's content. Stored under the shared
/// anchor pk `Partition::FactFoldHeadlines` with a sk carrying the
/// headline id (`EntityType::FactFoldHeadline(headline_id)`) so a
/// single `query` on the anchor pk lists every headline.
///
/// Lifecycle: Draft → Scheduled (optional) → Live (round in progress)
/// → Settled. Once Live, only `reveal_sources` may grow (roadmap §FR-43).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldHeadline {
    pub pk: Partition,  // Partition::FactFoldHeadlines
    pub sk: EntityType, // EntityType::FactFoldHeadline(headline_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Operator who authored this headline.
    pub creator_pk: Partition,

    pub status: HeadlineStatus,
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
    /// scheduler activates the headline as the live round at this time.
    pub scheduled_at: Option<i64>,
}

#[cfg(feature = "server")]
impl FactFoldHeadline {
    pub fn anchor_pk() -> Partition {
        Partition::FactFoldHeadlines
    }

    pub fn keys(headline_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFoldHeadlines,
            EntityType::FactFoldHeadline(headline_id.to_string()),
        )
    }

    /// Build a fresh draft from a creation request. Caller is responsible
    /// for validation and uniqueness check; this only assembles the row.
    pub fn new_draft(
        headline_id: String,
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
            HeadlineStatus::Scheduled
        } else {
            HeadlineStatus::Draft
        };
        Self {
            pk: Partition::FactFoldHeadlines,
            sk: EntityType::FactFoldHeadline(headline_id),
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
            EntityType::FactFoldHeadline(id) => Some(id.clone()),
            _ => None,
        }
    }

    /// True once a round has been started against this headline; mutation
    /// rules tighten (roadmap §FR-43).
    pub fn is_locked(&self) -> bool {
        matches!(self.status, HeadlineStatus::Live | HeadlineStatus::Settled)
    }
}
