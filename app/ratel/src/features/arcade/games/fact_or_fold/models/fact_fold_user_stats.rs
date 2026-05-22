use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Lifetime per-user FOF statistics. Updated as a side effect of
/// settlement (PR6). Lives at `Partition::User(user_pk) +
/// EntityType::FactFoldUserStats` so a `get_user_profile` query
/// reads it inline with other user-scoped rows.
///
/// PR7 wires the leaderboard view on top of these rows — accuracy
/// ranking is `correct_count / total_rounds`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldUserStats {
    pub pk: Partition,  // Partition::User(user_pk)
    pub sk: EntityType, // EntityType::FactFoldUserStats

    pub created_at: i64,
    pub updated_at: i64,

    /// Total rounds the user has played (won or lost). Increments
    /// once per settled round.
    #[serde(default)]
    pub total_rounds: i64,
    /// Subset of `total_rounds` where the user's final side matched
    /// the truth. (`bet.flipped_to.unwrap_or(bet.side) == verdict`.)
    #[serde(default)]
    pub correct_count: i64,
    /// Cumulative chip delta across all rounds — sum of
    /// `chips_out - buy_in` per settlement. Can be negative.
    #[serde(default)]
    pub lifetime_delta_chips: i64,
    /// Millis-since-epoch of the last settled round.
    #[serde(default)]
    pub last_played_at: i64,
}

#[cfg(feature = "server")]
impl FactFoldUserStats {
    pub fn keys(user_id: &str) -> (Partition, EntityType) {
        (
            Partition::User(user_id.to_string()),
            EntityType::FactFoldUserStats,
        )
    }

    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
        user_id: &str,
    ) -> crate::common::Result<Self> {
        let (pk, sk) = Self::keys(user_id);
        let row = FactFoldUserStats::get(cli, &pk, Some(sk.clone())).await?;
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Ok(row.unwrap_or_else(|| Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            total_rounds: 0,
            correct_count: 0,
            lifetime_delta_chips: 0,
            last_played_at: 0,
        }))
    }
}
