use crate::features::social::users::*;

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
pub async fn create_team_handler(body: CreateTeamRequest) -> crate::features::social::users::Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await
        .map_err(|e| crate::features::social::users::Error::Unauthorized(e.to_string()))?
        .ok_or(crate::features::social::users::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::features::social::users::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    // Validate username format: lowercase alphanumeric + underscores, min 3 chars
    let username = body.username.to_lowercase();
    if username.len() < 3
        || !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(crate::features::social::users::Error::BadRequest(
            "Username must be at least 3 characters and contain only lowercase letters, digits, or underscores".to_string(),
        ));
    }

    // Check username uniqueness
    use ratel_post::models::Team;
    let opt = ratel_post::models::TeamQueryOption::builder()
        .sk(username.clone())
        .limit(1);
    let (existing, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &username, opt).await?;

    if !existing.is_empty() {
        return Err(crate::features::social::users::Error::BadRequest(
            "Username is already taken".to_string(),
        ));
    }

    // Get the user
    use common::models::User;
    let user: User = User::get(cli, user_pk.clone(), Some(common::types::EntityType::User))
        .await?
        .ok_or(crate::features::social::users::Error::Unauthorized("User not found".to_string()))?;

    let team_pk: common::types::Partition = Team::create_new_team(
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

#[post("/api/user-shell/teams/list", session: Extension<tower_sessions::Session>)]
pub async fn get_user_teams_handler() -> crate::features::social::users::Result<Vec<common::contexts::TeamItem>> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await
        .map_err(|e| crate::features::social::users::Error::Unauthorized(e.to_string()))?
        .ok_or(crate::features::social::users::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::features::social::users::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    let sk_prefix = "UserTeam".to_string();
    let opt = ratel_auth::UserTeamQueryOption::builder().sk(sk_prefix);
    let (user_teams, _): (Vec<ratel_auth::UserTeam>, _) =
        ratel_auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<common::contexts::TeamItem> = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        let (permissions, description) = if team_pk.is_empty() {
            (Vec::new(), String::new())
        } else {
            let team_pk: common::types::Partition = team_pk.parse().unwrap_or_default();
            let perms = ratel_post::models::Team::get_permissions_by_team_pk(
                cli,
                &team_pk,
                &user_pk,
            )
            .await
            .unwrap_or_else(|_| ratel_post::types::TeamGroupPermissions::empty());
            let description = ratel_post::models::Team::get(
                cli,
                &team_pk,
                Some(common::types::EntityType::Team),
            )
            .await
            .ok()
            .flatten()
            .map(|team| team.description)
            .unwrap_or_default();
            (perms.0.into_iter().map(|p| p as u8).collect(), description)
        };

        items.push(common::contexts::TeamItem {
            pk: team_pk,
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            user_type: common::types::UserType::Team,
            permissions,
            description,
        });
    }

    Ok(items)
}
