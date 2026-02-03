use crate::features::membership::PurchaseEntity;
use crate::features::payment::*;
use crate::types::*;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserPurchase {
    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", pk)]
    #[dynamo(prefix = "PAYMENT", name = "find_by_status", index = "gsi2", pk)]
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(name = "find_by_status", index = "gsi2", sk)]
    #[dynamo(name = "find_by_payment_id", index = "gsi3", sk)]
    pub status: PurchaseStatus,
    pub tx_type: TransactionType,
    pub amount: i64,
    pub currency: Currency,
    #[dynamo(prefix = "PAYMENT", name = "find_by_payment_id", index = "gsi3", pk)]
    pub payment_id: String,
    pub tx_id: String,
}

impl UserPurchase {
    pub fn new(
        pk: UserPartition,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        payment_id: String,
        tx_id: String,
    ) -> Self {
        let uuid = sorted_uuid();
        let created_at = now();

        Self {
            pk: CompositePartition::user_purchase_pk(pk.into()),
            sk: EntityType::UserPurchase(uuid),
            tx_type,
            amount,
            created_at,
            payment_id,
            tx_id,
            currency,
            status: PurchaseStatus::Success,
        }
    }
}

impl PurchaseEntity for UserPurchase {
    fn pk(&self) -> &CompositePartition {
        &self.pk
    }

    fn create_transact_write_item(&self) -> TransactWriteItem {
        self.create_transact_write_item()
    }
}
