use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::types::UserType;

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

    pub fn has_permission(&self, permission: crate::features::posts::types::TeamGroupPermission) -> bool {
        let permissions: crate::features::posts::types::TeamGroupPermissions = self.permission_mask().into();
        permissions.contains(permission)
    }
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

    pub fn selected_team(&self) -> Option<TeamItem> {
        let teams = self.teams.read();
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
