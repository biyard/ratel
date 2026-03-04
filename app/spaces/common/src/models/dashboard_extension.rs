use common::{utils::time::get_now_timestamp_millis, DynamoEntity, EntityType, Partition};
use serde::{Deserialize, Serialize};

use crate::types::dashboard::{DashboardComponentData, DashboardExtension};

#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity)]
pub struct DashboardExtensionEntity {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub data: String,
}

impl DashboardExtensionEntity {
    pub fn from_data(
        space_pk: Partition,
        ext_id: &str,
        data: &DashboardComponentData,
    ) -> common::Result<Self> {
        let now = get_now_timestamp_millis();
        let extension = DashboardExtension {
            id: ext_id.to_string(),
            data: data.clone(),
        };
        let json = serde_json::to_string(&extension)
            .map_err(|e| common::Error::Unknown(format!("Failed to serialize extension: {e}")))?;

        Ok(Self {
            pk: space_pk,
            sk: EntityType::SpaceDashboardExtension(ext_id.to_string()),
            created_at: now,
            updated_at: now,
            data: json,
        })
    }

    pub fn keys_for(space_pk: &Partition, ext_id: &str) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceDashboardExtension(ext_id.to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl DashboardExtensionEntity {
    pub async fn upsert_extension(
        space_pk: &Partition,
        ext_id: &str,
        data: &DashboardComponentData,
    ) -> common::Result<()> {
        let config = common::config::CommonConfig::default();
        let cli = config.dynamodb();
        let entity = Self::from_data(space_pk.clone(), ext_id, data)?;
        entity.upsert(cli).await?;
        Ok(())
    }

    pub async fn delete_extension(space_pk: &Partition, ext_id: &str) -> common::Result<()> {
        let config = common::config::CommonConfig::default();
        let cli = config.dynamodb();
        let (pk, sk) = Self::keys_for(space_pk, ext_id);
        let _ = Self::delete(cli, &pk, Some(sk)).await;
        Ok(())
    }
}
