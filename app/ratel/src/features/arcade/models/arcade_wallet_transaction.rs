use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Wallet transaction kind. Together with `amount` (signed) this is
/// the full record of what happened. `ref_*` fields are optional
/// breadcrumbs back to the entity that caused the txn.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArcadeTxnKind {
    /// RP → chip conversion. `amount` is positive (chips credited).
    #[default]
    Convert,
    /// Chip → RP redemption. `amount` is negative (chips debited).
    /// v1 endpoint is disabled but the kind exists for forward compat.
    Redeem,
    /// Round buy-in. `amount` is negative (chips placed on the table).
    BuyIn,
    /// Round settle. `amount` is positive when the player wins chips
    /// back, zero when they lose everything.
    Settle,
    /// Operator-driven grant or correction.
    Adjustment,
}

/// Append-only ledger row for the arcade wallet. Each chip movement
/// (convert / buy-in / settle / ...) writes one row. The pk is the
/// same as the balance row, so a single pk query reads the full
/// history in one round-trip; `sk = EntityType::ArcadeWalletTxn(ulid)`
/// makes the rows time-sortable.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct ArcadeWalletTransaction {
    pub pk: Partition,  // Partition::ArcadeWallet(user_id)
    pub sk: EntityType, // EntityType::ArcadeWalletTxn(txn_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub kind: ArcadeTxnKind,

    /// Signed chip delta. Positive = credited, negative = debited.
    /// Sum of all txn amounts for a pk must equal the balance row's
    /// `chip_balance` invariant.
    pub amount: i64,

    /// Player chip balance *after* this txn was applied. Useful for
    /// idempotency / audit trails when reconstructing history.
    pub balance_after: i64,

    /// For BuyIn / Settle: the round this txn relates to. Stored as
    /// the raw round id (no prefix) so it round-trips with the
    /// FactFold partition.
    #[serde(default)]
    pub ref_round_id: Option<String>,
}

#[cfg(feature = "server")]
impl ArcadeWalletTransaction {
    pub fn keys(user_id: &str, txn_id: &str) -> (Partition, EntityType) {
        (
            Partition::ArcadeWallet(user_id.to_string()),
            EntityType::ArcadeWalletTxn(txn_id.to_string()),
        )
    }
}
