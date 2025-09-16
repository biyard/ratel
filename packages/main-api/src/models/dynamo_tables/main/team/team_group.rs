use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct TeamGroup {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,

    pub name: String,
    pub description: String,

    pub members: i64,

    pub permissions: i64,

    pub creator_pk: Partition,
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
            sk: EntityType::TeamGroup,
            name,
            description,
            created_at: now,
            permissions: permissions.into(),
            members: 0,
            creator_pk,
        }
    }
}
