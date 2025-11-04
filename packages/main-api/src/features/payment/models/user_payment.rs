use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserPayment {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub billing_key: Option<String>,
    pub customer_id: String,
    pub name: String,
    pub birth_date: String,
}

impl UserPayment {
    pub fn new(pk: Partition, customer_id: String, name: String, birth_date: String) -> Self {
        if !matches!(pk, Partition::User(_)) {
            panic!("UserPayment pk must be of Partition::User type");
        }
        let now = time::get_now_timestamp_millis();

        Self {
            pk: CompositePartition(pk, Partition::Payment),
            sk: EntityType::Created(now.to_string()),
            customer_id,
            name,
            birth_date,

            ..Default::default()
        }
    }
}
