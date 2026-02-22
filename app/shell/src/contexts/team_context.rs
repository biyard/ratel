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
