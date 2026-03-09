use crate::*;

// Re-export TeamItem, TeamContext, and use_team_context from common
pub use common::contexts::{use_team_context, TeamContext, TeamItem};

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

#[post("/api/teams/list", session: Extension<tower_sessions::Session>)]
pub async fn get_user_teams_handler() -> crate::Result<Vec<TeamItem>> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await?
        .ok_or(crate::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    let sk_prefix = common::EntityType::UserTeam(String::new()).to_string();
    let opt = ratel_auth::UserTeamQueryOption::builder().sk(sk_prefix);
    let (user_teams, _): (Vec<ratel_auth::UserTeam>, _) =
        ratel_auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<TeamItem> = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        let (permissions, description) = if team_pk.is_empty() {
            (Vec::new(), String::new())
        } else {
            let team_pk: common::types::Partition = team_pk.parse().unwrap_or_default();
            let perms =
                crate::features::posts::models::Team::get_permissions_by_team_pk(cli, &team_pk, &user_pk)
                    .await
                    .unwrap_or_else(|_| crate::features::posts::types::TeamGroupPermissions::empty());
            let description =
                crate::features::posts::models::Team::get(cli, &team_pk, Some(common::types::EntityType::Team))
                    .await
                    .ok()
                    .flatten()
                    .map(|team| team.description)
                    .unwrap_or_default();
            (perms.0.into_iter().map(|p| p as u8).collect(), description)
        };

        items.push(TeamItem {
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

#[post("/api/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(body: CreateTeamRequest) -> crate::Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await?
        .ok_or(crate::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    // Validate username format: lowercase alphanumeric, min 3 chars
    let username = body.username.to_lowercase();
    if username.len() < 3
        || !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(crate::Error::BadRequest(
            "Username must be at least 3 characters and contain only lowercase letters, digits, or underscores".to_string(),
        ));
    }

    // Check username uniqueness
    use crate::features::posts::models::Team;
    let opt = crate::features::posts::models::TeamQueryOption::builder()
        .sk(username.clone())
        .limit(1);
    let (existing, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &username, opt).await?;

    if !existing.is_empty() {
        return Err(crate::Error::BadRequest(
            "Username is already taken".to_string(),
        ));
    }

    // Get the user
    use common::models::User;
    let user: User = User::get(cli, user_pk, Some(common::types::EntityType::User))
        .await?
        .ok_or(crate::Error::Unauthorized("User not found".to_string()))?;

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
