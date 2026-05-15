use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::BetSide;

#[allow(unused_imports)]
use rmcp::schemars;

/// One row per (round, user). Created on first POST to
/// `/rounds/{id}/bets`; mutated by the §FR-29 last-10s flip via
/// `POST /bets/flip` (PR5). Settlement reads these rows + the
/// rationales to compute payouts.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldBet {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldBet(user_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,
    pub side: BetSide,
    pub amount_rp: i64,
    pub locked_at: i64,
    /// Set by the PR5 flip endpoint. `Some(side)` means the player
    /// switched in the last-10s window; settlement uses the
    /// post-flip side.
    pub flipped_to: Option<BetSide>,
    /// User pk whose rationale was cited as the flip trigger.
    /// Settlement awards the cited player a 30% influence bonus
    /// when their citation drove a successful flip.
    pub flip_cite_user_pk: Option<Partition>,
}

#[cfg(feature = "server")]
impl FactFoldBet {
    pub fn keys(round_id: &str, user_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldBet(user_id.to_string()),
        )
    }

    pub fn new(round_id: &str, user_pk: Partition, side: BetSide, amount_rp: i64) -> Self {
        let user_id = user_pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap_or(&user_pk.to_string())
            .to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(round_id, &user_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            side,
            amount_rp,
            locked_at: now,
            flipped_to: None,
            flip_cite_user_pk: None,
        }
    }
}
