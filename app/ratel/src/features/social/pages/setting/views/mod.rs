use dioxus::prelude::*;

mod admin_page;
mod management_page;
mod subscription_page;
mod viewer_page;

use admin_page::*;
pub use management_page::ManagementPage;
pub use subscription_page::SubscriptionPage;
pub use viewer_page::*;

use super::controllers::get_team_settings_handler;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_server_future(use_reactive((&username,), |(name,)| async move {
        get_team_settings_handler(name).await
    }))?;

    let binding = resource.read();

    match binding.as_ref() {
        Some(Ok(team)) => {
            let permissions: TeamGroupPermissions = team.permissions.unwrap_or(0).into();
            let can_edit = permissions.contains(TeamGroupPermission::TeamEdit)
                || permissions.contains(TeamGroupPermission::TeamAdmin);

            if can_edit {
                rsx! {
                    AdminPage { username, team: team.clone() }
                }
            } else {
                rsx! {
                    ViewerPage { username }
                }
            }
        }
        _ => rsx! {
            ViewerPage { username }
        },
    }
}
