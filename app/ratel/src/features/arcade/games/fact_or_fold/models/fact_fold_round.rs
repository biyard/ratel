use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::RoundStatus;

#[allow(unused_imports)]
use rmcp::schemars;

/// One round = one headline being judged by ≤ `round_capacity`
/// players. Created at lobby-start, transitions through stages, ends
/// at Settled. Per-participant data (bets, rationales, chat) lives
/// at separate sk's under the same pk; PR3 only carries the round
/// header itself + the participant_pks list (cheap headcount + dedup).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldRound {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldRound(round_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Headline being judged. Anchor pk on the headline side is
    /// `Partition::FactFoldHeadlines`; this is just the inner id.
    pub headline_id: String,

    pub status: RoundStatus,

    /// User pks in join order. Capacity is enforced by the matching
    /// service against `FactFoldSettings::round_capacity`.
    #[serde(default)]
    pub participant_pks: Vec<Partition>,

    /// Set when the round leaves Waiting.
    pub started_at: Option<i64>,
    /// Set when the round reaches Settled.
    pub settled_at: Option<i64>,

    /// Millis-since-epoch when the *current* stage began. Together
    /// with `stage_deadline_at` this is the server-verified clock the
    /// stage machine reads to decide auto-advancement (§FR-9).
    /// `None` while `status == Waiting`; `Some` once NewsReveal kicks
    /// off and re-stamped on every stage advance.
    #[serde(default)]
    pub stage_started_at: Option<i64>,
    /// Millis-since-epoch when the current stage is scheduled to
    /// auto-advance. Computed as `stage_started_at + duration` per
    /// `FactFoldSettings`.
    #[serde(default)]
    pub stage_deadline_at: Option<i64>,
}

#[cfg(feature = "server")]
impl FactFoldRound {
    pub fn keys(round_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldRound(round_id.to_string()),
        )
    }

    pub fn new_waiting(round_id: String, headline_id: String, first_user_pk: Partition) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&round_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            headline_id,
            status: RoundStatus::Waiting,
            participant_pks: vec![first_user_pk],
            started_at: None,
            settled_at: None,
            stage_started_at: None,
            stage_deadline_at: None,
        }
    }

    pub fn id(&self) -> Option<String> {
        match &self.sk {
            EntityType::FactFoldRound(id) => Some(id.clone()),
            _ => None,
        }
    }
}
