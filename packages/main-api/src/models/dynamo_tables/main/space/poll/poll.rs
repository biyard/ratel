use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct Poll {
    pub pk: Partition,
    pub sk: EntityType,

    pub status: PollStatus,
    pub started_at: i64,
    pub ended_at: i64,

    pub user_response_count: i64, // Participants count
    pub response_editable: bool,  // Whether users can edit their responses
}

impl Poll {
    pub fn new(
        pk: Partition,
        response_editable: bool,
        started_at: i64,
        ended_at: i64,
    ) -> crate::Result<Self> {
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "PollSpace must be under Space partition".to_string(),
            ));
        }

        Ok(Self {
            pk,
            sk: EntityType::SpacePoll,
            user_response_count: 0,
            response_editable,
            started_at,
            ended_at,
            status: PollStatus::Ready,
        })
    }
}
