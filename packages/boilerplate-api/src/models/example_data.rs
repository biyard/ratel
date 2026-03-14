use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct ExampleData {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "INFO", name = "find_by_data", index = "gsi1", pk)]
    pub data: String,
}

impl ExampleData {
    pub fn new(info: String) -> Self {
        let pk: ExampleDataPartition = uuid::Uuid::now_v7().to_string().into();
        let created_at = chrono::Utc::now().timestamp_millis();

        Self {
            pk: pk.into(),
            sk: EntityType::TS(created_at),
            created_at,
            updated_at: created_at,
            data: info,
        }
    }
}
