use crate::types::*;
use crate::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceNavItem {
    pub pk: Partition,
    pub sk: EntityType,

    pub page: String,

    #[dynamo(prefix = "SPACE_NAV", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub order: i64,

    pub created_at: i64,
    pub updated_at: i64,
}

impl SpaceNavItem {
    pub fn new(space_pk: Partition, page: String, order: i64) -> Self {
        let now = crate::utils::time::get_now_timestamp_millis();

        Self {
            pk: space_pk.clone(),
            sk: EntityType::SpaceNavItem(page.clone()),
            page,
            space_pk,
            order,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(space_pk: &Partition, page: &str) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceNavItem(page.to_string()),
        )
    }
}
