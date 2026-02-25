use common::{
    utils::time::get_now_timestamp_millis, DynamoEntity, EntityType, Partition, SpacePartition,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity)]
pub struct DashboardExtensionEntity {
    pub pk: Partition,  // PK: `SPACE#{space_id}`
    pub sk: EntityType, // SK: `SPACE_DASHBOARD_EXTENSION#{extension_id}`

    pub created_at: i64,
    pub updated_at: i64,

    pub data: String,
}

impl DashboardExtensionEntity {
    pub fn new(space_id: SpacePartition, data: String) -> Self {
        let now = get_now_timestamp_millis();
        let (pk, sk) = Self::keys(space_id);

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            data,
        }
    }

    pub fn keys(space_id: SpacePartition) -> (Partition, EntityType) {
        let extension_id = uuid::Uuid::now_v7().to_string();
        (
            space_id.into(),
            EntityType::SpaceDashboardExtension(extension_id),
        )
    }
}
