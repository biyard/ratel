use crate::*;

// Re-export TeamItem, TeamContext, and use_team_context from common
pub use crate::common::contexts::{use_team_context, TeamContext, TeamItem};

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

#[post("/api/teams/list?bookmark", session: Extension<tower_sessions::Session>)]
pub async fn get_user_teams_handler(
    bookmark: Option<String>,
) -> crate::Result<crate::common::types::ListResponse<TeamItem>> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await?
        .ok_or(crate::Error::NoSessionFound)?;

    let cli = crate::config::get().dynamodb();
    let user_pk: crate::common::types::Partition = user_pk.parse().unwrap_or_default();

    let sk_prefix = crate::common::EntityType::UserTeam(String::new()).to_string();
    let opt = crate::features::auth::UserTeam::opt_with_bookmark(bookmark)
        .sk(sk_prefix)
        .limit(20);
    let (user_teams, next_bookmark): (Vec<crate::features::auth::UserTeam>, _) =
        crate::features::auth::UserTeam::query(cli, &user_pk, opt).await?;

    let mut items: Vec<TeamItem> = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            crate::common::types::EntityType::UserTeam(team_pk) => team_pk,
            _ => String::new(),
        };
        let (permissions, description) = if team_pk.is_empty() {
            (Vec::new(), String::new())
        } else {
            let team_pk: crate::common::types::Partition = team_pk.parse().unwrap_or_default();
            let perms =
                crate::features::posts::models::Team::get_permissions_by_team_pk(cli, &team_pk, &user_pk)
                    .await
                    .unwrap_or_else(|_| crate::features::posts::types::TeamGroupPermissions::empty());
            let description =
                crate::features::posts::models::Team::get(cli, &team_pk, Some(crate::common::types::EntityType::Team))
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
            user_type: crate::common::types::UserType::Team,
            permissions,
            description,
        });
    }

    Ok((items, next_bookmark).into())
}

#[post("/api/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(body: CreateTeamRequest) -> crate::Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_id")
        .await?
        .ok_or(crate::Error::NoSessionFound)?;

    let cli = crate::config::get().dynamodb();
    let user_pk: crate::common::types::Partition = user_pk.parse().unwrap_or_default();

    // Validate username format: lowercase alphanumeric, min 3 chars
    let username = body.username.to_lowercase();
    if username.len() < 3
        || !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(crate::Error::InvalidTeamContext);
    }

    // Check username uniqueness
    use crate::features::posts::models::Team;
    let opt = crate::features::posts::models::TeamQueryOption::builder()
        .sk(username.clone())
        .limit(1);
    let (existing, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &username, opt).await?;

    if !existing.is_empty() {
        return Err(crate::Error::InvalidTeamContext);
    }

    // Get the user
    use crate::common::models::User;
    let user: User = User::get(cli, user_pk, Some(crate::common::types::EntityType::User))
        .await?
        .ok_or(crate::Error::UserNotFoundInContext)?;

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
