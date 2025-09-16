use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostSpace {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "INFO", name = "find_by_info", index = "gsi1", pk)]
    pub info: String,
}

impl PostSpace {
    pub fn new(pk: Partition, info: String) -> Self {
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::PostSpace,
            created_at,
            updated_at: created_at,
            info,
        }
    }
}
