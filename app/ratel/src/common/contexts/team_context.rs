use crate::{
    common::{types::UserType, Error, TeamItem},
    posts::types::TeamGroupPermissions,
    social::controllers::{create_team_handler, get_user_teams_handler, CreateTeamRequest},
    *,
};

#[derive(Clone, Copy, DioxusController)]
pub struct TeamContext {
    pub teams: Loader<Vec<TeamItem>>,
    pub selected_index: Signal<usize>,
}

impl TeamContext {
    pub async fn create_team(
        &mut self,
        payload: crate::common::TeamCreationPayload,
    ) -> crate::Result<TeamItem> {
        let crate::common::TeamCreationPayload {
            profile_url,
            username,
            nickname,
            description,
        } = payload;

        let req = CreateTeamRequest {
            username: username.clone(),
            nickname: nickname.clone(),
            profile_url: profile_url.clone(),
            description: description.clone(),
        };
        let response = create_team_handler(req.clone()).await?;
        let permissions: Vec<u8> = TeamGroupPermissions::all()
            .0
            .into_iter()
            .map(|p| p as u8)
            .collect();
        let CreateTeamRequest {
            profile_url,
            username,
            nickname,
            description,
        } = req;
        let team_item = TeamItem {
            pk: response.team_pk.clone(),
            nickname: nickname.clone(),
            username: username.clone(),
            profile_url: profile_url.clone(),
            user_type: UserType::Team,
            permissions: permissions.clone(),
            description: description.clone(),
            created_at: 0,
            member_count: 1,
        };
        self.teams.push(team_item.clone());

        Ok(team_item)
    }

    pub fn set_teams(&mut self, teams: Vec<TeamItem>) {
        self.teams.set(teams);
    }

    pub fn add_team(&mut self, team: TeamItem) {
        self.teams.push(team);
    }

    pub fn selected_team(&self) -> Option<TeamItem> {
        let teams = self.teams.read();
        let idx = *self.selected_index.read();
        teams.get(idx).cloned()
    }

    pub fn select_team_by_idx(&mut self, index: usize) {
        self.selected_index.set(index);
    }

    pub fn select_team(&mut self, team_pk: &str) {
        let teams = self.teams.read();
        if let Some((idx, _)) = teams.iter().enumerate().find(|(_, t)| t.pk == team_pk) {
            self.selected_index.set(idx);
        }
    }
}

pub fn use_team_context_provider() -> Result<TeamContext, Loading> {
    let AuthContext { logged_in, .. } = use_auth_context();
    let teams = use_loader(move || {
        let logged_in = logged_in();
        debug!("Loading teams, logged_in: {}", logged_in);

        async move {
            if !logged_in {
                return Ok(Vec::new());
            }

            Ok::<_, Error>(get_user_teams_handler().await.unwrap_or_default().items)
        }
    })?;
    let selected_index = use_signal(|| 0);

    let ctx = use_context_provider(move || TeamContext {
        teams,
        selected_index,
    });
    Ok(ctx)
}

#[track_caller]
pub fn use_team_context() -> TeamContext {
    use_context::<TeamContext>()
}

pub fn consume_team_context() -> TeamContext {
    consume_context::<TeamContext>()
}
