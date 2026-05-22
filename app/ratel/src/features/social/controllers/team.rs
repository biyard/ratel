use crate::features::social::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub username: String,
    pub nickname: String,
    pub profile_url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamResponse {
    pub team_pk: String,
    /// Epoch-ms of the new team's creation. Returned so the client can
    /// optimistically insert a fully-formed `TeamItem` without re-fetching.
    #[serde(default)]
    pub created_at: i64,
}

#[post("/api/user-shell/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(
    body: CreateTeamRequest,
) -> crate::features::social::Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await
        .map_err(|e| {
            crate::error!("session: {e}");
            crate::features::social::types::SocialError::SessionNotFound
        })?
        .ok_or(crate::features::social::types::SocialError::SessionNotFound)?;

    let cli = crate::features::social::config::get().dynamodb();
    let user_pk: crate::common::types::Partition = user_pk.parse().unwrap_or_default();

    // Validate username format: lowercase alphanumeric + underscores, min 3 chars
    let username = body.username.to_lowercase();
    if username.len() < 3
        || !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(crate::features::social::types::SocialError::InvalidTeamName.into());
    }

    // Check username uniqueness against teams
    use crate::features::posts::models::Team;
    let opt = Team::opt().limit(1);
    let (existing, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &username, opt).await?;

    if existing.iter().any(|t| t.username == username) {
        return Err(crate::features::social::types::SocialError::TeamNameTaken.into());
    }

    // Check username uniqueness against users
    let (existing_users, _) = crate::common::models::User::find_by_username(
        cli,
        &username,
        crate::features::auth::UserQueryOption::builder()
            .sk("TS#".to_string())
            .limit(1),
    )
    .await?;

    if !existing_users.is_empty() {
        return Err(crate::features::social::types::SocialError::TeamNameTaken.into());
    }

    // Get the user
    use crate::common::models::User;
    let user: User = User::get(
        cli,
        user_pk.clone(),
        Some(crate::common::types::EntityType::User),
    )
    .await?
    .ok_or(crate::features::social::types::SocialError::UserNotFound)?;

    let (team_pk, created_at) = Team::create_new_team(
        &user,
        cli,
        body.nickname,
        body.profile_url,
        username,
        body.description,
    )
    .await?;

    Ok(CreateTeamResponse {
        team_pk: team_pk.to_string(),
        created_at,
    })
}

#[mcp_tool(
    name = "list_teams",
    description = "List all teams the user belongs to with their role and permissions. Use team_id from the result as the team_id parameter in create_post to post under a team."
)]
#[get("/api/me/teams", user: crate::features::auth::User)]
pub async fn get_user_teams_handler(
) -> crate::features::social::Result<crate::common::types::ListResponse<crate::common::TeamItem>> {
    let cli = crate::features::social::config::get().dynamodb();
    let user_pk = user.pk.clone();

    let sk_prefix = crate::common::types::EntityType::UserTeam(String::new()).to_string();
    let opt = crate::features::auth::UserTeam::opt_all().sk(sk_prefix);
    let (user_teams, next_bookmark): (Vec<crate::features::auth::UserTeam>, _) =
        crate::features::auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<crate::common::TeamItem> = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            crate::common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        let (permissions, description) = if team_pk.is_empty() {
            (Vec::new(), String::new())
        } else {
            let team_pk: crate::common::types::Partition = team_pk.parse().unwrap_or_default();
            let role = crate::features::posts::models::Team::get_user_role(cli, &team_pk, &user_pk)
                .await
                .ok()
                .flatten()
                .unwrap_or_default();
            let perms: crate::features::posts::types::TeamGroupPermissions =
                role.to_legacy_permissions().into();
            let description = crate::features::posts::models::Team::get(
                cli,
                &team_pk,
                Some(crate::common::types::EntityType::Team),
            )
            .await
            .ok()
            .flatten()
            .map(|team| team.description)
            .unwrap_or_default();
            (perms.0.into_iter().map(|p| p as u8).collect(), description)
        };

        items.push(crate::common::TeamItem {
            pk: team_pk,
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            user_type: crate::common::types::UserType::Team,
            permissions,
            description,
            created_at: 0,
            member_count: 0,
        });
    }

    Ok((items, next_bookmark).into())
}

// ── List teams where the caller has admin/owner role ────────────
//
// Filters `UserTeam` rows by the persisted `role` enum (canonical
// authority for team membership — `permissions` is legacy). Used by
// the sub-team apply page's team picker so the dropdown only shows
// teams the user can submit an application for.
#[post("/api/user-shell/teams/admin?bookmark", user: crate::features::auth::User)]
pub async fn list_admin_teams_handler(
    bookmark: Option<String>,
) -> crate::features::social::Result<crate::common::types::ListResponse<crate::common::TeamItem>> {
    let cli = crate::features::social::config::get().dynamodb();
    let user_pk = user.pk.clone();

    let sk_prefix = crate::common::types::EntityType::UserTeam(String::new()).to_string();
    let opt = crate::features::auth::UserTeam::opt_with_bookmark(bookmark)
        .sk(sk_prefix)
        .limit(20);
    let (user_teams, next_bookmark): (Vec<crate::features::auth::UserTeam>, _) =
        crate::features::auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<crate::common::TeamItem> = Vec::new();
    for ut in user_teams {
        if !ut.role.is_admin_or_owner() {
            continue;
        }
        let team_pk = match ut.sk.clone() {
            crate::common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        // For each admin/owner team we also fetch the Team row to get
        // `description` + `created_at` and run a paged member count.
        // The apply page's eligibility panel uses `created_at` to evaluate
        // the parent's `min_sub_team_age_days` requirement and
        // `member_count` for `min_sub_team_members`.
        let (permissions, description, created_at, member_count) = if team_pk.is_empty() {
            (Vec::new(), String::new(), 0i64, 0i64)
        } else {
            let team_pk_part: crate::common::types::Partition = team_pk.parse().unwrap_or_default();
            let perms: crate::features::posts::types::TeamGroupPermissions =
                ut.role.to_legacy_permissions().into();
            let team_row = crate::features::posts::models::Team::get(
                cli,
                &team_pk_part,
                Some(crate::common::types::EntityType::Team),
            )
            .await
            .ok()
            .flatten();
            let description = team_row
                .as_ref()
                .map(|t| t.description.clone())
                .unwrap_or_default();
            let created_at = team_row.as_ref().map(|t| t.created_at).unwrap_or(0);
            let member_count =
                crate::features::sub_team::services::count_team_members(cli, &team_pk_part)
                    .await
                    .unwrap_or(0);
            (
                perms.0.into_iter().map(|p| p as u8).collect(),
                description,
                created_at,
                member_count,
            )
        };

        items.push(crate::common::TeamItem {
            pk: team_pk,
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            user_type: crate::common::types::UserType::Team,
            permissions,
            description,
            created_at,
            member_count,
        });
    }

    Ok((items, next_bookmark).into())
}
