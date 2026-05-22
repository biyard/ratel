use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Materialized leaderboard row (PR7). Maintained as a settlement
/// side-effect: every time `FactFoldUserStats` is bumped, the
/// settlement handler also rewrites the matching leaderboard
/// entry — deleting the prior accuracy-keyed row and writing a
/// fresh one at the new sk.
///
/// sk encoding: `{accuracy_bps:010}#{user_id}` — accuracy in
/// basis points (0..=10_000) zero-padded to 10 chars so an
/// sk-DESC query at `Partition::FactFoldLeaderboard` returns the
/// highest-accuracy users first regardless of who tied at the
/// boundary.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldLeaderboardEntry {
    pub pk: Partition,  // Partition::FactFoldLeaderboard
    pub sk: EntityType, // EntityType::FactFoldLeaderboardEntry(inner)

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,
    /// Accuracy = correct_count / total_rounds, in basis points
    /// (10_000 = 100% accurate).
    pub accuracy_bps: i32,
    pub total_rounds: i64,
    pub correct_count: i64,
    pub lifetime_delta_chips: i64,
    pub last_played_at: i64,
}

#[cfg(feature = "server")]
impl FactFoldLeaderboardEntry {
    pub fn anchor_pk() -> Partition {
        Partition::FactFoldLeaderboard
    }

    pub fn encode_sk_inner(accuracy_bps: i32, user_id: &str) -> String {
        let clamped = accuracy_bps.clamp(0, 10_000) as u32;
        format!("{:010}#{}", clamped, user_id)
    }

    pub fn keys(accuracy_bps: i32, user_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFoldLeaderboard,
            EntityType::FactFoldLeaderboardEntry(Self::encode_sk_inner(
                accuracy_bps,
                user_id,
            )),
        )
    }
}
