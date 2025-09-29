use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PollSpace {
    pub pk: Partition,
    pub sk: EntityType,

    pub user_response_count: i64, // Participants count
}

impl PollSpace {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::PollSpace(uid),
            sk: EntityType::Space,
            user_response_count: 0,
        }
    }
}
