use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// One rationale per (round, user). Persisted permanently per
/// §FR-11 even if the player declines to register it to Essence.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldRationale {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldRationale(user_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,
    pub text: String,
    pub submitted_at: i64,
    /// True iff `text.chars().count() >= RATIONALE_ESSENCE_MIN_CHARS`.
    /// Computed at submit time so the eligibility never changes
    /// after the fact (even if config moves).
    pub essence_eligible: bool,
    /// Set true once the player opts in via the post-settlement
    /// review screen (PR6). Until then, the rationale row exists
    /// but does not propagate to the player's Essence index.
    pub essence_registered: bool,
}

#[cfg(feature = "server")]
impl FactFoldRationale {
    pub fn keys(round_id: &str, user_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldRationale(user_id.to_string()),
        )
    }

    pub fn new(round_id: &str, user_pk: Partition, text: String, essence_eligible: bool) -> Self {
        let user_id = UserPartition::from(user_pk.clone()).0;
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(round_id, &user_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            text,
            submitted_at: now,
            essence_eligible,
            essence_registered: false,
        }
    }
}
