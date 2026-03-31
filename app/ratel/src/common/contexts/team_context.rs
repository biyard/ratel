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
}

impl TeamContext {
    pub fn init() -> Result<Self, Loading> {
        let teams = use_loader(move || async move {
            crate::get_user_teams_handler()
                .await
                .or_else(|_| Ok::<_, crate::Error>(Vec::new()))
        })?;

        let ctx = Self { teams };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}

pub fn use_team_context() -> TeamContext {
    use_context::<TeamContext>()
}
