use crate::macros::DynamoEntity;
use crate::*;
use common::utils::time::get_now_timestamp_millis;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity, PartialEq)]
pub struct SpaceApp {
    pub pk: Partition,
    pub sk: EntityType,

    pub app_type: SpaceAppType,

    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl SpaceApp {
    pub fn new(space_pk: Partition, app_type: SpaceAppType) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceApp(app_type.to_string()),
            app_type,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(space_pk: &Partition, app_type: SpaceAppType) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::SpaceApp(app_type.to_string()))
    }

    pub fn sk_prefix() -> String {
        EntityType::SpaceApp(String::new()).to_string()
    }

    pub fn dashboard_write_items(&self) -> Vec<aws_sdk_dynamodb::types::TransactWriteItem> {
        use space_common::models::dashboard_extension::DashboardExtensionEntity;
        use space_common::types::dashboard::*;

        match self.app_type {
            SpaceAppType::IncentivePool => {
                let mut items = vec![];

                let stat_card = DashboardComponentData::StatCard(StatCardData {
                    icon: DashboardIcon::IncentivePool,
                    value: "0".to_string(),
                    trend: 0.0,
                    trend_label: "USDT".to_string(),
                    total_winners: "0".to_string(),
                    rank_rate: "show".to_string(),
                    incentive_pool: String::new(),
                });
                if let Ok(e) =
                    DashboardExtensionEntity::from_data(self.pk.clone(), EXT_ID_STAT_CARD, &stat_card)
                {
                    items.push(e.upsert_transact_write_item());
                }

                let ranking = DashboardComponentData::RankingTable(RankingTableData {
                    entries: vec![],
                    page_size: 10,
                });
                if let Ok(e) =
                    DashboardExtensionEntity::from_data(self.pk.clone(), EXT_ID_RANKING_TABLE, &ranking)
                {
                    items.push(e.upsert_transact_write_item());
                }

                items
            }
            _ => vec![],
        }
    }

    pub fn dashboard_delete_items(&self) -> Vec<aws_sdk_dynamodb::types::TransactWriteItem> {
        use space_common::models::dashboard_extension::DashboardExtensionEntity;
        use space_common::types::dashboard::*;

        match self.app_type {
            SpaceAppType::IncentivePool => {
                let (pk1, sk1) = DashboardExtensionEntity::keys_for(&self.pk, EXT_ID_STAT_CARD);
                let (pk2, sk2) = DashboardExtensionEntity::keys_for(&self.pk, EXT_ID_RANKING_TABLE);
                vec![
                    DashboardExtensionEntity::delete_transact_write_item(&pk1, sk1),
                    DashboardExtensionEntity::delete_transact_write_item(&pk2, sk2),
                ]
            }
            _ => vec![],
        }
    }
}

impl SpaceApp {
    pub fn can_view(role: SpaceUserRole) -> Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }

    pub fn can_edit(role: SpaceUserRole) -> Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }

    pub fn can_delete(role: SpaceUserRole) -> Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }
}
