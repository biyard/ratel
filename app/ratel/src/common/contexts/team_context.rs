use dioxus::fullstack::{Loader, Loading};
use serde::{Deserialize, Serialize};

use crate::common::types::UserType;
use crate::*;

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
    pub fn label(&self) -> &str {
        if self.nickname.is_empty() {
            &self.username
        } else {
            &self.nickname
        }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct TeamContext {
    pub teams: Loader<Vec<TeamItem>>,
    pub selected_index: Signal<usize>,
}

impl TeamContext {
    pub fn init() -> Result<Self, Loading> {
        let teams = use_loader(move || async move {
            crate::get_user_teams_handler()
                .await
                .or_else(|_| Ok::<_, crate::Error>(Vec::new()))
        })?;

        let ctx = Self {
            teams,
            selected_index: use_signal(|| 0),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }

    pub fn selected_team(&self) -> Option<TeamItem> {
        let teams = (self.teams)();
        let idx = *self.selected_index.read();
        teams.get(idx).cloned()
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index.set(index);
    }
}

pub fn use_team_context() -> TeamContext {
    use_context::<TeamContext>()
}
