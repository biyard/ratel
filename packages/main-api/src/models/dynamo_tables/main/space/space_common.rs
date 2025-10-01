use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceCommon {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub sk: EntityType,

    // Space statistics
    pub participants: i64,

    #[dynamo(prefix = "VIS", name = "find_by_visibility", index = "gsi6", pk)]
    pub visibility: SpaceVisibility,

    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi1", pk)]
    pub post_pk: Partition,

    pub status: SpaceStatus,
    pub started_at: i64,
    pub ended_at: Option<i64>,
}

impl SpaceCommon {
    pub fn new(pk: Partition, post_pk: Partition) -> Self {
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::SpaceCommon,
            post_pk,
            participants: 0,
            visibility: SpaceVisibility::Public,
            status: SpaceStatus::Draft,

            started_at: now,
            ended_at: None,
        }
    }

    pub fn with_visibility(mut self, visibility: SpaceVisibility) -> Self {
        self.visibility = visibility;
        self
    }
}
