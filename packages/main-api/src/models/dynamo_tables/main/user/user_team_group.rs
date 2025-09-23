use crate::{models::team::team_group::TeamGroup, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeamGroup {
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
}

impl UserTeamGroup {
    pub fn new(
        pk: Partition,
        TeamGroup {
            sk: team_group_sk,
            permissions: team_group_permissions,
            ..
        }: TeamGroup,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::UserTeamGroup(team_group_sk.to_string()),
            team_group_permissions,
        }
    }
}
