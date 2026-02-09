use common::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
pub struct SpaceNavItemModel {
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
