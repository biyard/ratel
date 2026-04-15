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
}

#[post("/api/user-shell/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(body: CreateTeamRequest) -> crate::features::social::Result<CreateTeamResponse> {
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

    // Check username uniqueness
    use crate::features::posts::models::Team;
    let opt = crate::features::posts::models::TeamQueryOption::builder()
        .sk(username.clone())
        .limit(1);
    let (existing, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &username, opt).await?;

    if !existing.is_empty() {
        return Err(crate::features::social::types::SocialError::TeamNameTaken.into());
    }

    // Get the user
    use crate::common::models::User;
    let user: User = User::get(cli, user_pk.clone(), Some(crate::common::types::EntityType::User))
        .await?
        .ok_or(crate::features::social::types::SocialError::UserNotFound)?;

    let team_pk: crate::common::types::Partition = Team::create_new_team(
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
    })
}

#[mcp_tool(name = "list_teams", description = "List all teams the user belongs to with their role and permissions. Use team_id from the result as the team_id parameter in create_post to post under a team.")]
#[post("/api/user-shell/teams/list", user: crate::features::auth::User)]
pub async fn get_user_teams_handler() -> crate::features::social::Result<Vec<crate::common::contexts::TeamItem>> {
    let cli = crate::features::social::config::get().dynamodb();
    let user_pk = user.pk.clone();

    let sk_prefix = crate::common::types::EntityType::UserTeam(String::new()).to_string();
    let opt = crate::features::auth::UserTeamQueryOption::builder().sk(sk_prefix);
    let (user_teams, _): (Vec<crate::features::auth::UserTeam>, _) =
        crate::features::auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<crate::common::contexts::TeamItem> = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            crate::common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        let (permissions, description) = if team_pk.is_empty() {
            (Vec::new(), String::new())
        } else {
            let team_pk: crate::common::types::Partition = team_pk.parse().unwrap_or_default();
            let role = crate::features::posts::models::Team::get_user_role(
                cli, &team_pk, &user_pk,
            )
            .await
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

        items.push(crate::common::contexts::TeamItem {
            pk: team_pk,
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            user_type: crate::common::types::UserType::Team,
            permissions,
            description,
        });
    }

    Ok(items)
}
