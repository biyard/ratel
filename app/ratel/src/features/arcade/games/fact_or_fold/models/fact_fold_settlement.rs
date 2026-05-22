use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// One row per (round, user). Written by the settlement handler
/// (PR6) with the §FR-28~30 breakdown. `idempotency_key` is the
/// stable composite `{round_id}#{user_id}` — a re-run of the handler
/// hits the same row and a `create_if_not_exists` is a no-op, so
/// chips never get credited twice.
///
/// All amounts are in *chips* (the in-game token), not RP. The
/// chip ↔ RP conversion happens at the wallet boundary
/// (`ArcadeWallet::convert_*`), not here.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldSettlement {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldSettlement(user_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,

    /// `{round_id}#{user_id}` — used by the create-if-not-exists
    /// path so settlement re-runs are no-ops.
    pub idempotency_key: String,

    // ── Breakdown ─────────────────────────────────────────────────
    /// Stake refund for a winning player (= the player's own stake).
    /// 0 for losers.
    pub base_refund: i64,
    /// `(own_stake) × (correct_side_multiplier_bps − 10_000) / 10_000`.
    /// 0 for losers.
    pub correct_bonus: i64,
    /// Share of the loser pool, proportional to the winner's stake.
    /// 0 for losers.
    pub pool_share: i64,
    /// Citation bonus credited to the player whose rationale was
    /// cited by another winning flipper (§FR-29).
    pub influence_bonus: i64,
    /// Insider's correctness bonus (§FR-30). Non-zero only for the
    /// round insider when they bet on the truth side.
    pub insider_bonus: i64,

    /// Final chips credited back to the wallet for this user.
    /// `chips_out = base_refund + correct_bonus + pool_share +
    /// influence_bonus + insider_bonus`. The wallet's
    /// `settle(round, chips_out)` uses this number directly.
    pub chips_out: i64,
}

#[cfg(feature = "server")]
impl FactFoldSettlement {
    pub fn keys(round_id: &str, user_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldSettlement(user_id.to_string()),
        )
    }

    pub fn idempotency_key_for(round_id: &str, user_id: &str) -> String {
        format!("{round_id}#{user_id}")
    }
}
