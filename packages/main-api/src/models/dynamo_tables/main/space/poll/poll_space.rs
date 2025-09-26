use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PollSpace {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "INFO", name = "find_by_info", index = "gsi1", pk)]
    pub info: String,
}

impl PollSpace {
    pub fn new(info: String) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::PollSpace(uid),
            sk: EntityType::Space,
            created_at,
            updated_at: created_at,
            info,
        }
    }
}
