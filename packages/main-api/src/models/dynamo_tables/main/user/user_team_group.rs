use crate::{models::team::team_group::TeamGroup, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeamGroup {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(
        prefix = "TEAM_GROUP_PK",
        name = "find_by_team_group_pk",
        index = "gsi1",
        pk
    )]
    pub team_group_pk: Partition,
    pub team_group_permissions: i64,
}

impl UserTeamGroup {
    pub fn new(
        pk: Partition,
        TeamGroup {
            pk: team_group_pk,
            permissions: team_group_permissions,
            ..
        }: TeamGroup,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::UserTeamGroup,
            team_group_permissions,
            team_group_pk,
        }
    }
}
