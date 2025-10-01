use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct Team {
    pub pk: Partition,
    #[dynamo(
        prefix = "TEAM_NAME_IDX",
        name = "find_by_name_prefix",
        index = "gsi1",
        pk
    )]
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    // TODO: When chaning display_name, profile_url and username, UserTeam items should be updated as well.
    #[dynamo(index = "gsi1", sk)]
    pub display_name: String,
    pub profile_url: String,
    // NOTE: username is linked with gsi2-index of user model.
    #[dynamo(
        prefix = "USERNAME",
        name = "find_by_username_prefix",
        index = "gsi2",
        pk
    )]
    pub username: String, // Team Name

    pub description: String,
}

impl Team {
    pub fn new(
        display_name: String,
        profile_url: String,
        username: String,
        description: String,
    ) -> Self {
        let team_id = uuid::Uuid::new_v4().to_string();
        let pk = Partition::Team(team_id);
        let sk = EntityType::Team;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name,
            profile_url,
            username,
            description,
        }
    }

    pub async fn get_permitted_team(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: Partition,
        _user_pk: Partition,
        _perm: TeamGroupPermission,
    ) -> Result<Self, crate::Error2> {
        // TODO: Implement permission check logic

        let team = Self::get(cli, team_pk, Some(EntityType::Team))
            .await?
            .ok_or(crate::Error2::TeamNotFound)?;

        Ok(team)
    }

    pub async fn has_permission(
        _cli: &aws_sdk_dynamodb::Client,
        _team_pk: &Partition,
        _user_pk: &Partition,
        _perm: TeamGroupPermission,
    ) -> Result<bool, crate::Error2> {
        // TODO: Implement permission check logic

        Ok(true)
    }
}
