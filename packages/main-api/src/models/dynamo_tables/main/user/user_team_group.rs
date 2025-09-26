use crate::{models::team::team_group::TeamGroup, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
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

    #[dynamo(index = "gsi2", name = "find_by_team_pk", pk)]
    pub team_pk: Partition,
}

impl UserTeamGroup {
    pub fn new(
        pk: Partition, // User Pk
        TeamGroup {
            pk: team_pk,       // Team Pk
            sk: team_group_sk, // EntityType::TeamGroup(new Uuid)
            permissions: team_group_permissions,
            ..
        }: TeamGroup,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::UserTeamGroup(team_group_sk.to_string()),
            team_group_permissions,
            team_pk,
        }
    }
}
