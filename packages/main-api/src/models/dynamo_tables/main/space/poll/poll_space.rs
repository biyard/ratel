use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PollSpace {
    pub pk: Partition,
    pub sk: EntityType,

    pub user_response_count: i64, // Participants count
}

impl PollSpace {
    pub fn new(pk: Partition) -> crate::Result<Self> {
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "PollSpace must be under Space partition".to_string(),
            ));
        }

        Ok(Self {
            pk,
            sk: EntityType::Space,
            user_response_count: 0,
        })
    }
}
