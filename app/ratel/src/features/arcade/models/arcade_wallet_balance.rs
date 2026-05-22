use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Per-user chip balance singleton. Lives at
/// `Partition::ArcadeWallet(user_id) + EntityType::ArcadeWalletBalance`.
/// Append-only transaction history sits at the same pk under
/// `EntityType::ArcadeWalletTxn(txn_id)` so a single pk query returns
/// the whole ledger (balance + every txn).
///
/// Chip vs RP: see [crate::features::arcade] for the casino metaphor.
/// `chip_balance` is the player's spendable balance — buy-in deducts,
/// settle credits, convert moves RP into chips.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct ArcadeWalletBalance {
    pub pk: Partition,  // Partition::ArcadeWallet(user_id)
    pub sk: EntityType, // EntityType::ArcadeWalletBalance

    pub created_at: i64,
    pub updated_at: i64,

    /// Player's currently spendable chips. v1 RP↔chip is 1:1 but
    /// `chip_balance` is always tracked in chip units.
    #[serde(default)]
    pub chip_balance: i64,
}

#[cfg(feature = "server")]
impl ArcadeWalletBalance {
    pub fn keys(user_id: &str) -> (Partition, EntityType) {
        (
            Partition::ArcadeWallet(user_id.to_string()),
            EntityType::ArcadeWalletBalance,
        )
    }

    /// Load the balance row, returning an empty (zero-balance) row
    /// when the user has never interacted with the arcade. Callers
    /// can `upsert` it back to materialise the row.
    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
        user_id: &str,
    ) -> crate::common::Result<Self> {
        let (pk, sk) = Self::keys(user_id);
        let row = ArcadeWalletBalance::get(cli, &pk, Some(sk.clone())).await?;
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Ok(row.unwrap_or_else(|| Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            chip_balance: 0,
        }))
    }
}
