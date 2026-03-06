use dioxus::prelude::*;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

use crate::controllers::get_team_drafts_permission_handler;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn Home(teamname: String) -> Element {
    let resource = use_loader(use_reactive((&teamname,), |(name,)| async move {
        Ok::<_, crate::Error>(
            get_team_drafts_permission_handler(name)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let data = resource.read();

    match data.as_ref() {
        Ok(ctx) => {
            let permissions: TeamGroupPermissions = ctx.permissions.into();
            let can_edit = permissions.contains(TeamGroupPermission::TeamEdit)
                || permissions.contains(TeamGroupPermission::TeamAdmin);

            if can_edit {
                rsx! {
                    AdminPage { teamname, team_pk: ctx.team_pk.clone() }
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
