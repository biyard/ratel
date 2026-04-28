use dioxus::{
    fullstack::{Loader, Loading},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::use_user_context,
    common::{types::UserType, Error},
    posts::types::TeamGroupPermissions,
    social::controllers::{create_team_handler, get_user_teams_handler, CreateTeamRequest},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamItem {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
    pub user_type: UserType,
    #[serde(default)]
    pub permissions: Vec<u8>,
    #[serde(default)]
    pub description: String,
}

impl TeamItem {
    pub fn permission_mask(&self) -> i64 {
        let mut mask = 0i64;
        for v in &self.permissions {
            mask |= 1i64 << (*v as i32);
        }
        mask
    }

    pub fn has_permission(
        &self,
        permission: crate::features::posts::types::TeamGroupPermission,
    ) -> bool {
        let permissions: crate::features::posts::types::TeamGroupPermissions =
            self.permission_mask().into();
        permissions.contains(permission)
    }
}

#[derive(Clone, Copy)]
pub struct TeamContext {
    pub teams: Loader<Vec<TeamItem>>,
    pub selected_index: Signal<usize>,
    pub create_team_action: Action<(CreateTeamRequest,), TeamItem>,
}

impl TeamContext {
    pub fn init() {
        let user_ctx = use_user_context();
        let mut teams = use_loader(move || async move {
            let logged_in = user_ctx().is_logged_in();

            if !logged_in {
                return Ok(Vec::new());
            }

            Ok::<_, Error>(get_user_teams_handler(None).await.unwrap_or_default().items)
        })?;

        let create_team_action = use_action(move |req: CreateTeamRequest| async move {
            match create_team_handler(req.clone()).await {
                Ok(response) => {
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
                    };
                    teams.push(team_item.clone());
                    Ok(team_item)
                }
                Err(e) => Err(e),
            }
        });

        use_context_provider(move || TeamContext {
            teams,
            selected_index: use_signal(|| 0),
            create_team_action,
        });
    }

    pub fn set_teams(&mut self, teams: Vec<TeamItem>) {
        self.teams.set(teams);
    }

    pub fn selected_team(&self) -> Option<TeamItem> {
        let teams = self.teams.read();
        let idx = *self.selected_index.read();
        teams.get(idx).cloned()
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index.set(index);
    }

    pub fn select_team(&mut self, team_pk: &str) {
        let teams = self.teams.read();
        if let Some((idx, _)) = teams.iter().enumerate().find(|(_, t)| t.pk == team_pk) {
            self.selected_index.set(idx);
        }
    }
}

#[track_caller]
pub fn use_team_context() -> TeamContext {
    use_context::<TeamContext>()
}
