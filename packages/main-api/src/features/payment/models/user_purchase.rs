use crate::features::payment::*;
use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserPurchase {
    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", pk)]
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", sk)]
    pub created_at: i64,

    pub tx_type: TransactionType,
    pub amount: i64,
}

impl UserPurchase {
    pub fn new(pk: Partition, tx_type: TransactionType, amount: i64) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_millis();

        Self {
            pk,
            sk: EntityType::UserPurchase(uuid),
            tx_type,
            amount,
            created_at,
        }
    }

    pub fn get_payment_id(&self) -> String {
        match &self.sk {
            EntityType::UserPurchase(id) => id.clone(),
            _ => panic!("Invalid sk for UserPurchase"),
        }
    }
}
