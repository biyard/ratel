use crate::{utils::time::get_now_timestamp_millis, *};

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
    ) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk: EntityType::TeamGroup(uuid::Uuid::new_v4().to_string()),
            name,
            description,
            created_at: now,
            permissions: permissions.into(),
            members: 1,
        }
    }
}
