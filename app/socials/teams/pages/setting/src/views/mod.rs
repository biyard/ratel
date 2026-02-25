use dioxus::prelude::*;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

use crate::controllers::get_team_settings_handler;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn Home(teamname: String) -> Element {
    let teamname_clone = teamname.clone();
    let resource = use_server_future(move || {
        let name = teamname_clone.clone();
        async move { get_team_settings_handler(name).await }
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    match data.as_ref() {
        Ok(team) => {
            let permissions: TeamGroupPermissions = team.permissions.unwrap_or(0).into();
            let can_edit = permissions.contains(TeamGroupPermission::TeamEdit)
                || permissions.contains(TeamGroupPermission::TeamAdmin);

            if can_edit {
                rsx! {
                    AdminPage { teamname, team: team.clone() }
                }
            } else {
                rsx! {
                    ViewerPage { teamname }
                }
            }
        }
        Err(_) => {
            rsx! {
                ViewerPage { teamname }
            }
        }
    }
}
