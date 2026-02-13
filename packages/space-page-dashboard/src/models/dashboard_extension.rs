use common::{EntityType, Partition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
// #[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity)]
pub struct DashboardExtensionEntity {
    pub pk: Partition, // PK: `SPACE#{space_id}`
    pub sk: EntityType, // SK: `SPACE_DASHBOARD_EXTENSION#{extension_id}`

    pub created_at: i64,
    pub updated_at: i64,

    pub extension_id: String,
    pub data: String,
}

impl DashboardExtensionEntity {
    pub fn new(space_id: &str, data: String) -> Self {
        let extension_id = uuid::Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp_millis();

        Self {
            pk: Partition::Space(space_id.to_string()),
            sk: EntityType::SpaceDashboardExtension(extension_id.clone()),
            created_at: now,
            updated_at: now,
            extension_id,
            data,
        }
    }

    pub fn with_id(space_id: &str, extension_id: String, data: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();

        Self {
            pk: Partition::Space(space_id.to_string()),
            sk: EntityType::SpaceDashboardExtension(extension_id.clone()),
            created_at: now,
            updated_at: now,
            extension_id,
            data,
        }
    }
}
