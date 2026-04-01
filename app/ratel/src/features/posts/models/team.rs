use crate::features::posts::types::*;
use crate::features::posts::*;
#[cfg(feature = "server")]
use crate::features::auth::OptionalUser;
#[cfg(feature = "server")]
use crate::features::auth::UserTeamGroup;

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

    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub allow_invite: bool,
    #[serde(default)]
    pub allow_create_space: bool,
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
        let now = crate::common::utils::time::get_now_timestamp_millis();

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
            thumbnail_url: None,
            allow_invite: false,
            allow_create_space: false,
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

        let admin_group = TeamGroup::new(
            team.pk.clone(),
            "Admin".to_string(),
            "Administrators group with all permissions".to_string(),
            TeamGroupPermissions::all(),
        );

        let member_group = TeamGroup::new(
            team.pk.clone(),
            "Member".to_string(),
            "Default group for team members".to_string(),
            TeamGroupPermissions::member(),
        );

        let user_pk = user.pk.clone();
        let team_pk = team.pk.clone();

        let user_team_group = crate::features::auth::UserTeamGroup::new(
            user_pk.clone(),
            admin_group.sk.clone(),
            admin_group.permissions,
            team_pk.clone(),
        );
        let user_team = crate::features::auth::UserTeam::new(
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
                admin_group.create_transact_write_item(),
                member_group.create_transact_write_item(),
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

        let (groups, _bookmark) = UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await?;

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

#[cfg(feature = "server")]
fn extract_team_identifier(parts: &Parts) -> Result<String> {
    let path = parts.uri.path().trim_matches('/');
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // Pattern 1: /api/teams/:team_id/... (client-side API call)
    // Require the preceding segment to be "api" to avoid colliding with
    // SSR routes for a team whose username happens to be "teams".
    for i in 1..segments.len() {
        if segments[i] == "teams" && segments[i - 1] == "api" {
            if let Some(&value) = segments.get(i + 1) {
                return Ok(value.to_string());
            }
        }
    }

    // /:username/... (SSR page URL)
    if let Some(&first) = segments.first() {
        if first.eq_ignore_ascii_case("api") {
            return Err(Error::BadRequest(
                "Invalid team path: missing team identifier".to_string(),
            ));
        }
        return Ok(first.to_string());
    }

    Err(Error::BadRequest(
        "Missing team identifier in path".to_string(),
    ))
}

#[cfg(feature = "server")]
async fn resolve_team_from_identifier(
    cli: &aws_sdk_dynamodb::Client,
    team_id: &str,
) -> Result<Team> {
    if let Ok(team_pk) = team_id.parse::<TeamPartition>() {
        let team_pk: Partition = team_pk.into();
        if let Some(team) = Team::get(cli, &team_pk, Some(EntityType::Team)).await? {
            return Ok(team);
        }
    }

    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix).limit(1);
    let (teams, _) =
        Team::find_by_username_prefix(cli, team_id.to_string(), team_query_option).await?;

    teams
        .into_iter()
        .find(|t| t.username == team_id)
        .ok_or(Error::NotFound("Team not found".to_string()))
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for Team
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        if let Some(team) = parts.extensions.get::<Team>() {
            return Ok(team.clone());
        }

        let team_id = extract_team_identifier(parts)?;
        let cli = crate::features::posts::config::get().dynamodb();
        let team = resolve_team_from_identifier(cli, &team_id).await?;
        parts.extensions.insert(team.clone());

        Ok(team)
    }
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for TeamGroupPermissions
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        if let Some(permissions) = parts.extensions.get::<TeamGroupPermissions>() {
            return Ok(permissions.clone());
        }

        let team = Team::from_request_parts(parts, state).await?;
        let user: Option<crate::features::auth::User> =
            OptionalUser::from_request_parts(parts, state).await?.into();
        let cli = crate::features::posts::config::get().dynamodb();
        let permissions = if let Some(user) = user {
            Team::get_permissions_by_team_pk(cli, &team.pk, &user.pk)
                .await
                .unwrap_or_else(|_| TeamGroupPermissions::empty())
        } else {
            TeamGroupPermissions::empty()
        };

        parts.extensions.insert(permissions.clone());
        Ok(permissions)
    }
}
