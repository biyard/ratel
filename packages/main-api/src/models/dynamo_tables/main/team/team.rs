use crate::{
    models::{
        TeamOwner,
        user::{UserTeamGroup, UserTeamGroupQueryOption},
    },
    types::*,
};
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
    #[dynamo(index = "gsi6", name = "find_by_follwers", pk)]
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

    #[dynamo(index = "gsi6", sk)]
    pub followers: i64,
    pub followings: i64,

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
            followers: 0,
            followings: 0,
        }
    }

    pub async fn get_permissions_by_team_pk(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<TeamGroupPermissions, crate::Error> {
        // Check if the user is the team owner first
        let owner = TeamOwner::get(cli, team_pk, Some(EntityType::TeamOwner)).await?;
        if let Some(owner) = owner {
            if owner.user_pk == *user_pk {
                // Team owner has all permissions
                return Ok(TeamGroupPermissions::all());
            }
        }

        // NOTE: it only fetches up to 50 UserTeamGroup items.
        let (groups, _bookmark) = UserTeamGroup::find_by_team_pk(
            cli,
            team_pk,
            UserTeamGroupQueryOption::builder()
                .sk(user_pk.to_string())
                .limit(50),
        )
        .await?;

        let mut perms = 0i64;

        for UserTeamGroup {
            team_group_permissions,
            ..
        } in groups
        {
            perms |= team_group_permissions;
        }

        Ok(perms.into())
    }

    pub async fn get_permitted_team(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: Partition,
        _user_pk: Partition,
        _perm: TeamGroupPermission,
    ) -> Result<Self, crate::Error> {
        // TODO: Implement permission check logic

        let team = Self::get(cli, team_pk, Some(EntityType::Team))
            .await?
            .ok_or(crate::Error::TeamNotFound)?;

        Ok(team)
    }

    pub async fn has_permission(
        _cli: &aws_sdk_dynamodb::Client,
        _team_pk: &Partition,
        _user_pk: &Partition,
        _perm: TeamGroupPermission,
    ) -> Result<bool, crate::Error> {
        // TODO: Implement permission check logic

        Ok(true)
    }
}
