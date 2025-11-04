use crate::features::payment::*;
use crate::types::*;
use crate::*;
const CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserPurchase {
    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", pk)]
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", sk)]
    pub created_at: i64,

    pub tx_type: TransactionType,
    pub amount: i64,
    pub payment_id: String,
    pub tx_id: Option<String>,
}

impl UserPurchase {
    pub fn new(pk: Partition, tx_type: TransactionType, amount: i64) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_millis();
        let payment_id = random_string::generate(30, CHARSET);

        Self {
            pk: CompositePartition(pk, Partition::Purchase),
            sk: EntityType::UserPurchase(uuid),
            tx_type,
            amount,
            created_at,
            payment_id,
            tx_id: None,
        }
    }
}
