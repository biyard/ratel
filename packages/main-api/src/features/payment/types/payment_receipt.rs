use crate::{features::payment::{TeamPurchase, UserPurchase}, *};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct PaymentReceipt {
    pub id: String,
    pub paid_at: i64,
    pub tx_type: TransactionType,
    pub currency: Currency,
    pub tx_id: String,
    pub amount: i64,
}

impl From<UserPurchase> for PaymentReceipt {
    fn from(purchase: UserPurchase) -> Self {
        Self {
            id: purchase.payment_id,
            paid_at: purchase.created_at,
            tx_type: purchase.tx_type,
            currency: purchase.currency,
            tx_id: purchase.tx_id,
            amount: purchase.amount,
        }
    }
}

impl From<TeamPurchase> for PaymentReceipt {
    fn from(purchase: TeamPurchase) -> Self {
        Self {
            id: purchase.payment_id,
            paid_at: purchase.created_at,
            tx_type: purchase.tx_type,
            currency: purchase.currency,
            tx_id: purchase.tx_id,
            amount: purchase.amount,
        }
    }
}
