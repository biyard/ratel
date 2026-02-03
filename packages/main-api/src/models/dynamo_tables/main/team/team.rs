use crate::models::{TeamGroup, UserTeam};
use crate::utils::time::get_now_timestamp_millis;
use crate::*;
use crate::{
    models::{
        TeamOwner,
        user::{UserTeamGroup, UserTeamGroupQueryOption},
    },
    types::*,
};

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

    #[dynamo(prefix = "TEAM", index = "gsi2", sk)]
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
    #[serde(default)]
    pub followers: i64,
    #[serde(default)]
    pub followings: i64,

    pub description: String,

    pub dao_address: Option<String>,
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

        let now = get_now_timestamp_millis();

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
            dao_address: None,
        }
    }

    pub async fn get_permissions_by_team_pk(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<TeamGroupPermissions> {
        // Check if the user is the team owner first
        let owner = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await?;
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
        user_pk: Partition,
        perm: TeamGroupPermission,
    ) -> Result<Self> {
        if !Self::has_permission(cli, &team_pk, &user_pk, perm).await? {
            return Err(crate::Error::TeamNotFound);
        }

        let team = Self::get(cli, team_pk, Some(EntityType::Team))
            .await?
            .ok_or(crate::Error::TeamNotFound)?;

        Ok(team)
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
        perm: TeamGroupPermission,
    ) -> Result<bool> {
        let (group, _bookmark) = UserTeamGroup::find_by_team_pk(
            cli,
            team_pk.clone(),
            UserTeamGroupQueryOption::builder()
                .sk(user_pk.to_string())
                .limit(1),
        )
        .await?;

        let group = group.first().cloned().ok_or(crate::Error::TeamNotFound)?;

        let permissions: TeamGroupPermissions = group.team_group_permissions.into();

        Ok(permissions.contains(perm))
    }

    pub async fn create_new_team(
        user: &User,
        cli: &aws_sdk_dynamodb::Client,
        display_name: String,
        profile_url: String,
        username: String,
        description: String,
    ) -> Result<Partition> {
        let team = Team::new(display_name, profile_url, username, description);

        let team_owner = TeamOwner::new(team.pk.clone(), user.clone());

        let team_group = TeamGroup::new(
            team.pk.clone(),
            "Admin".to_string(),
            "Administrators group with all permissions".to_string(),
            crate::types::TeamGroupPermissions::all(),
        );

        let user_pk = user.pk.clone();
        let team_pk = team.pk.clone();

        let user_team_group = UserTeamGroup::new(user_pk.clone(), team_group.clone());
        let user_team = UserTeam::new(user_pk, team.clone());

        transact_write!(
            cli,
            team.create_transact_write_item(),
            team_owner.create_transact_write_item(),
            team_group.create_transact_write_item(),
            user_team_group.create_transact_write_item(),
            user_team.create_transact_write_item(),
        )?;

        Ok(team_pk)
    }
}

#[async_trait::async_trait]
impl EntityPermissions for Team {
    async fn get_permissions_for(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        requester: &Partition,
    ) -> Permissions {
        let owner = TeamOwner::get(cli, &self.pk, Some(&EntityType::TeamOwner)).await;
        if owner.is_err() {
            return Permissions::empty();
        }

        let owner = owner.unwrap();
        if let Some(owner) = owner {
            if &owner.user_pk == requester {
                return Permissions::all();
            }
        }

        // NOTE: it only fetches up to 50 UserTeamGroup items.
        let res = UserTeamGroup::find_by_team_pk(
            cli,
            self.pk.clone(),
            UserTeamGroupQueryOption::builder()
                .sk(requester.to_string())
                .limit(50),
        )
        .await;

        if res.is_err() {
            return Permissions::empty();
        }

        let (groups, _bookmark) = res.unwrap();
        let mut perms = 0i64;

        for UserTeamGroup {
            team_group_permissions,
            ..
        } in groups
        {
            perms |= team_group_permissions;
        }

        perms.into()
    }
}
