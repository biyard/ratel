use crate::types::*;
use crate::*;
#[cfg(feature = "server")]
use ratel_auth::UserTeamGroup;

#[cfg(feature = "server")]
use super::{TeamGroup, TeamOwner};

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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

    #[dynamo(index = "gsi1", sk)]
    pub display_name: String,
    pub profile_url: String,
    #[dynamo(
        prefix = "USERNAME",
        name = "find_by_username_prefix",
        index = "gsi2",
        pk
    )]
    pub username: String,

    #[dynamo(index = "gsi6", sk)]
    #[serde(default)]
    pub followers: i64,
    #[serde(default)]
    pub followings: i64,

    pub description: String,

    pub dao_address: Option<String>,
}

#[cfg(feature = "server")]
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
        let now = common::utils::time::get_now_timestamp_millis();

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
            TeamGroupPermissions::all(),
        );

        let user_pk = user.pk.clone();
        let team_pk = team.pk.clone();

        let user_team_group = ratel_auth::UserTeamGroup::new(
            user_pk.clone(),
            team_group.sk.clone(),
            team_group.permissions,
            team_pk.clone(),
        );
        let user_team = ratel_auth::UserTeam::new(
            user_pk,
            team_pk.clone(),
            team.display_name.clone(),
            team.profile_url.clone(),
            team.username.clone(),
            team.dao_address.clone(),
        );

        cli.transact_write_items()
            .set_transact_items(Some(vec![
                team.create_transact_write_item(),
                team_owner.create_transact_write_item(),
                team_group.create_transact_write_item(),
                user_team_group.create_transact_write_item(),
                user_team.create_transact_write_item(),
            ]))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        Ok(team_pk)
    }

    pub async fn get_permitted_team(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: Partition,
        user_pk: Partition,
        perm: TeamGroupPermission,
    ) -> Result<Self> {
        if !Self::has_permission(cli, &team_pk, &user_pk, perm).await? {
            return Err(Error::NotFound("Team not found".to_string()));
        }

        let team = Self::get(cli, team_pk, Some(EntityType::Team))
            .await?
            .ok_or(Error::NotFound("Team not found".to_string()))?;

        Ok(team)
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
        perm: TeamGroupPermission,
    ) -> Result<bool> {
        // Check if the user is the team owner first
        let owner = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await?;
        if let Some(owner) = owner {
            if owner.user_pk == *user_pk {
                return Ok(true);
            }
        }

        let opt = UserTeamGroup::opt().sk(user_pk.to_string()).limit(1);

        let (group, _bookmark) = UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await?;

        let group = group
            .first()
            .cloned()
            .ok_or(Error::NotFound("Team not found".to_string()))?;

        let permissions: TeamGroupPermissions = group.team_group_permissions.into();

        Ok(permissions.contains(perm))
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
                return Ok(TeamGroupPermissions::all());
            }
        }

        let opt = UserTeamGroup::opt().sk(user_pk.to_string()).limit(50);

        let (groups, _bookmark) =
            UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await?;

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
}
