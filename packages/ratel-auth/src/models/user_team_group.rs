use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserTeamGroup {
    #[dynamo(index = "gsi2", sk)]
    pub pk: Partition,
    #[dynamo(
        prefix = "TEAM_GROUP_PK",
        name = "find_by_team_group_pk",
        index = "gsi1",
        pk
    )]
    pub sk: EntityType,
    #[dynamo(index = "gsi1", sk)]
    pub team_group_permissions: i64,

    #[dynamo(
        prefix = "USER_TEAM_GROUP",
        index = "gsi2",
        name = "find_by_team_pk",
        pk
    )]
    pub team_pk: Partition,
}
