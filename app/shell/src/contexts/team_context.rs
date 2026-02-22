use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamItem {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
    pub user_type: common::types::UserType,
}

#[derive(Clone, Copy)]
pub struct TeamContext {
    pub teams: Signal<Vec<TeamItem>>,
    pub selected_index: Signal<usize>,
}

impl TeamContext {
    pub fn init() {
        let ctx = Self {
            teams: use_signal(Vec::new),
            selected_index: use_signal(|| 0),
        };
        use_context_provider(move || ctx);
    }

    pub fn set_teams(&mut self, teams: Vec<TeamItem>) {
        self.teams.set(teams);
    }
}

pub fn use_team_context() -> TeamContext {
    use_context::<TeamContext>()
}

#[post("/api/teams/list", session: Extension<tower_sessions::Session>)]
pub async fn get_user_teams_handler() -> crate::Result<Vec<TeamItem>> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_pk")
        .await?
        .ok_or(crate::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    let sk_prefix = "UserTeam".to_string();
    let opt = ratel_auth::UserTeamQueryOption::builder().sk(sk_prefix);
    let (user_teams, _): (Vec<ratel_auth::UserTeam>, _) =
        ratel_auth::UserTeam::query(cli, &user_pk, opt).await?;

    let items: Vec<TeamItem> = user_teams
        .into_iter()
        .map(|ut| TeamItem {
            pk: ut.pk.to_string(),
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            user_type: common::types::UserType::Team,
        })
        .collect();

    Ok(items)
}

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

#[post("/api/teams/create", session: Extension<tower_sessions::Session>)]
pub async fn create_team_handler(
    body: CreateTeamRequest,
) -> crate::Result<CreateTeamResponse> {
    let Extension(session) = session;
    let user_pk: String = session
        .get::<String>("user_pk")
        .await?
        .ok_or(crate::Error::Unauthorized("no session".to_string()))?;

    let cli = crate::config::get().dynamodb();
    let user_pk: common::types::Partition = user_pk.parse().unwrap_or_default();

    // Validate username format: lowercase alphanumeric, min 3 chars
    let username = body.username.to_lowercase();
    if username.len() < 3 || !username.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return Err(crate::Error::BadRequest(
            "Username must be at least 3 characters and contain only lowercase letters, digits, or underscores".to_string(),
        ));
    }

    // Check username uniqueness
    use ratel_post::models::Team;
    let opt = ratel_post::models::TeamQueryOption::builder()
        .sk(username.clone())
        .limit(1);
    let (existing, _): (Vec<Team>, _) =
        Team::find_by_username_prefix(cli, &username, opt).await?;

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

    let team_pk: common::types::Partition =
        Team::create_new_team(&user, cli, body.nickname, body.profile_url, username, body.description)
            .await?;

    Ok(CreateTeamResponse {
        team_pk: team_pk.to_string(),
    })
}
