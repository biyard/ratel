use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamOwner {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
}
