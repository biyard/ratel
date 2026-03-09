use crate::features::spaces::apps::main::macros::DynamoEntity;
use crate::features::spaces::apps::main::*;
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

}

impl SpaceApp {
    pub fn can_view(_role: SpaceUserRole) -> Result<()> {
        Ok(())
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
