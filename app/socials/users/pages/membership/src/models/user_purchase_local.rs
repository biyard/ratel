use crate::dto::TransactionType;
use crate::serde;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct UserPurchaseLocal {
    #[cfg_attr(feature = "server", dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", pk))]
    #[cfg_attr(feature = "server", dynamo(prefix = "PAYMENT", name = "find_by_status", index = "gsi2", pk))]
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[cfg_attr(feature = "server", dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", sk))]
    pub created_at: i64,

    #[cfg_attr(feature = "server", dynamo(name = "find_by_status", index = "gsi2", sk))]
    #[cfg_attr(feature = "server", dynamo(name = "find_by_payment_id", index = "gsi3", sk))]
    pub status: String,
    pub tx_type: TransactionType,
    pub amount: i64,
    pub currency: String,
    #[cfg_attr(feature = "server", dynamo(prefix = "PAYMENT", name = "find_by_payment_id", index = "gsi3", pk))]
    pub payment_id: String,
    pub tx_id: String,
}
