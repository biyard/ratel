use crate::types::*;
use crate::utils::generate_random_code;
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct AttributeCode {
    pub pk: Partition,
    #[dynamo(index = "gsi1", prefix = "AC", name = "find", pk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi1", prefix = "AC", name = "find", sk)]
    pub created_at: i64,

    pub birth_date: Option<String>, // YYYYMMDD
    pub gender: Option<Gender>,
    pub university: Option<String>,
}

impl AttributeCode {
    pub fn new() -> Self {
        let code = generate_random_code();
        let created_at = get_now_timestamp_millis();

        Self {
            pk: Partition::AttributeCode(code),
            sk: EntityType::AttributeCode,
            created_at,
            ..Default::default()
        }
    }
}
