#[cfg(feature = "server")]
use crate::features::auth::OptionalUser;
#[cfg(feature = "server")]
use crate::features::auth::UserTeamGroup;
use crate::features::posts::types::*;
use crate::features::posts::*;

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

    // Sub-team governance — per-team flag that opts this team into the
    // parent-eligible program (advertises itself as accepting sub-team
    // applications). Existing rows deserialize as `false` via serde(default).
    #[serde(default)]
    pub is_parent_eligible: bool,

    // Minimum member count an applicant team must have before its Submit
    // button is enabled. `0` means no minimum.
    #[serde(default)]
    pub min_sub_team_members: i32,

    // Parent-child scalars. Invariants:
    //   recognized sub-team ⇔ parent_team_id.is_some()
    //   pending sub-team    ⇔ pending_parent_team_id.is_some() && parent_team_id.is_none()
    //   standalone team     ⇔ both None
    // Stored as the raw team UUID, not the SPACE#-prefixed Partition string.
    #[serde(default)]
    pub parent_team_id: Option<String>,
    #[serde(default)]
    pub pending_parent_team_id: Option<String>,
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
            is_parent_eligible: false,
            min_sub_team_members: 0,
            parent_team_id: None,
            pending_parent_team_id: None,
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

        let user_pk = user.pk.clone();
        let team_pk = team.pk.clone();

        let user_team = crate::features::auth::UserTeam::new(
            user_pk,
            team_pk.clone(),
            team.display_name.clone(),
            team.profile_url.clone(),
            team.username.clone(),
            team.dao_address.clone(),
            // Team creator is the owner.
            crate::features::social::pages::member::dto::TeamRole::Owner,
        );

        cli.transact_write_items()
            .set_transact_items(Some(vec![
                team.create_transact_write_item(),
                team_owner.create_transact_write_item(),
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
            return Err(PostError::TeamNotFound.into());
        }

        let team = Self::get(cli, team_pk, Some(EntityType::Team))
            .await?
            .ok_or::<Error>(PostError::TeamNotFound.into())?;

        Ok(team)
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
        perm: TeamGroupPermission,
    ) -> Result<bool> {
        // Resolve membership via the TeamRole model instead of the legacy
        // UserTeamGroup table (which is no longer populated on new teams).
        // Role → legacy permissions mapping keeps `perm` argument honoured
        // so existing callsites (PostWrite, SpaceEdit, etc.) work unchanged.
        let Some(role) = Self::get_user_role(cli, team_pk, user_pk).await? else {
            return Ok(false);
        };
        let permissions: TeamGroupPermissions = role.to_legacy_permissions().into();
        Ok(permissions.contains(perm))
    }

    /// Returns the calling user's role on this team. TeamOwner record takes
    /// precedence over UserTeam.role (defensive); falls back to UserTeam.role
    /// if present, else Member. Replaces the legacy permissions extractor.
    /// Returns `Some(role)` when the user has a membership on the team,
    /// `None` when the user is not a member at all. Callers that need a
    /// strict "member vs non-member" distinction should rely on the
    /// `Option`; treating a non-member as `TeamRole::Member` (the enum
    /// default) was the old behaviour and incorrectly hid the follow
    /// button / granted Member-level UI to logged-in strangers.
    pub async fn get_user_role(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<Option<crate::features::social::pages::member::dto::TeamRole>> {
        use crate::features::social::pages::member::dto::TeamRole;
        if let Some(owner) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await? {
            if owner.user_pk == *user_pk {
                return Ok(Some(TeamRole::Owner));
            }
        }
        let user_team_sk = EntityType::UserTeam(team_pk.to_string());
        let user_team =
            crate::features::auth::UserTeam::get(cli, user_pk, Some(&user_team_sk)).await?;
        Ok(user_team.map(|ut| ut.role))
    }

    pub async fn get_permissions_by_team_pk(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<TeamGroupPermissions> {
        let role = Self::get_user_role(cli, team_pk, user_pk)
            .await?
            .unwrap_or_default();
        Ok(role.to_legacy_permissions().into())
    }
}

#[cfg(feature = "server")]
fn extract_team_identifier(parts: &Parts) -> Result<String> {
    let path = parts.uri.path().trim_matches('/');
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // Pattern 1: /api/teams/:team_id/... (client-side API call)
    // Pattern 2: /v3/teams/:team_id/... (membership/payment API call)
    // Require the preceding segment to be "api" or "v3" to avoid colliding with
    // SSR routes for a team whose username happens to be "teams".
    for i in 1..segments.len() {
        if segments[i] == "teams" && matches!(segments[i - 1], "api" | "v3") {
            if let Some(&value) = segments.get(i + 1) {
                return Ok(value.to_string());
            }
        }
    }

    // /:username/... (SSR page URL)
    if let Some(&first) = segments.first() {
        if first.eq_ignore_ascii_case("api") {
            return Err(PostError::InvalidTeamContext.into());
        }
        return Ok(first.to_string());
    }

    Err(PostError::InvalidTeamContext.into())
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
        .ok_or::<Error>(PostError::TeamNotFound.into())
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

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for crate::features::social::pages::member::dto::TeamRole
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        use crate::features::social::pages::member::dto::TeamRole;
        if let Some(role) = parts.extensions.get::<TeamRole>() {
            return Ok(*role);
        }

        let team = Team::from_request_parts(parts, state).await?;
        let user: Option<crate::features::auth::User> =
            OptionalUser::from_request_parts(parts, state).await?.into();
        let user = user.ok_or(Error::UnauthorizedAccess)?;
        let cli = crate::features::posts::config::get().dynamodb();

        if let Some(owner) = TeamOwner::get(cli, &team.pk, Some(&EntityType::TeamOwner)).await? {
            if owner.user_pk == user.pk {
                parts.extensions.insert(TeamRole::Owner);
                return Ok(TeamRole::Owner);
            }
        }
        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        let user_team =
            crate::features::auth::UserTeam::get(cli, &user.pk, Some(&user_team_sk)).await?;
        let role = user_team
            .map(|ut| ut.role)
            .ok_or(Error::UnauthorizedAccess)?;

        parts.extensions.insert(role);
        Ok(role)
    }
}
