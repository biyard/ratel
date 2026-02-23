use crate::*;
use common::utils::time::get_now_timestamp_millis;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct SpaceInstalledApp {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: SpaceAppName,

    pub created_at: i64,
    pub updated_at: i64,
}

impl SpaceInstalledApp {
    pub fn new(space_pk: Partition, name: SpaceAppName) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceInstalledApp(name.to_string()),
            name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(space_pk: &Partition, name: SpaceAppName) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInstalledApp(name.to_string()),
        )
    }

    pub fn sk_prefix() -> String {
        EntityType::SpaceInstalledApp(String::new()).to_string()
    }
}
