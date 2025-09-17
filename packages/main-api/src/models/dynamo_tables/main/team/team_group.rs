use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct TeamGroup {
    pub pk: Partition,

    #[dynamo(
        prefix = "TEAM_GROUP_PK",
        name = "find_by_team_group_pk",
        index = "gsi1",
        pk
    )]
    pub sk: EntityType,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub name: String,
    pub description: String,

    pub members: i64,

    pub permissions: i64,
}

impl TeamGroup {
    pub fn new(
        pk: Partition,
        name: String,
        description: String,
        permissions: TeamGroupPermissions,
        User { pk: creator_pk, .. }: User,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::TeamGroup(creator_pk.to_string()),
            name,
            description,
            created_at: now,
            permissions: permissions.into(),
            members: 0,
        }
    }
}
