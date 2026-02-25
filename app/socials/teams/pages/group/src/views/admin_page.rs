use crate::*;
use dioxus::prelude::*;

use crate::components::{CreateGroupModal, InviteMemberModal, InviteResult, ListGroups};
use crate::controllers::{
    CreateGroupRequest, create_group_handler, delete_group_handler, list_groups_handler,
};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn AdminPage(teamname: String, team_pk: Partition, permissions: i64) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    use_context_provider(|| PopupService::new());
    let mut popup = use_popup();

    let perms: TeamGroupPermissions = permissions.into();
    let can_edit_group = perms.contains(TeamGroupPermission::GroupEdit);
    let can_edit_team = perms.contains(TeamGroupPermission::TeamEdit);

    let mut refresh = use_signal(|| 0u64);
    let team_pk_clone = team_pk.clone();
    let refresh_clone = refresh.clone();
    let group_resource = use_server_future(move || {
        let _ = refresh_clone();
        let team_pk = team_pk_clone.clone();
        async move { list_groups_handler(team_pk, None).await }
    })?;

    let resolved = group_resource.suspend()?;
    let data = resolved.read();
    let groups = match data.as_ref() {
        Ok(list) => list.items.clone(),
        Err(_) => vec![],
    };

    let on_invite_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let teamname = teamname.clone();
        let groups = groups.clone();
        let can_edit_group = can_edit_group;
        let mut refresh = refresh.clone();
        move |_evt: MouseEvent| {
            if !can_edit_group {
                return;
            }
            let on_close = {
                let mut popup = popup;
                move |_| {
                    popup.close();
                }
            };
            let on_invited = {
                let mut refresh = refresh.clone();
                move |result: InviteResult| {
                    if result.total_added > 0 {
                        refresh.set(refresh() + 1);
                    }
                }
            };
            popup
                .open(rsx! {
                    InviteMemberModal {
                        team_pk: team_pk.clone(),
                        teamname: teamname.clone(),
                        groups: groups.clone(),
                        on_close,
                        on_invited,
                    }
                })
                .without_backdrop_close();
        }
    };

    let on_create_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let mut refresh = refresh.clone();
        let can_edit_team = can_edit_team;
        move |_evt: MouseEvent| {
            if !can_edit_team {
                return;
            }
            let on_create = {
                let mut popup = popup;
                let mut refresh = refresh.clone();
                let team_pk = team_pk.clone();
                move |payload: crate::components::CreateGroupPayload| {
                    let mut popup = popup;
                    let mut refresh = refresh.clone();
                    let team_pk = team_pk.clone();
                    spawn(async move {
                        let result = create_group_handler(
                            team_pk,
                            CreateGroupRequest {
                                name: payload.name,
                                description: payload.description,
                                image_url: String::new(),
                                permissions: payload.permissions,
                            },
                        )
                        .await;
                        if result.is_ok() {
                            refresh.set(refresh() + 1);
                            popup.close();
                        }
                    });
                }
            };
            popup
                .open(rsx! {
                    CreateGroupModal { on_create }
                })
                .with_title(tr.create_group.to_string());
        }
    };

    let on_delete_group = {
        let team_pk = team_pk.clone();
        let mut refresh = refresh.clone();
        move |group_id: String| {
            let team_pk = team_pk.clone();
            let mut refresh = refresh.clone();
            spawn(async move {
                let result = delete_group_handler(team_pk, group_id).await;
                if result.is_ok() {
                    refresh.set(refresh() + 1);
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col w-full gap-2.5",
            div { class: "flex flex-row w-full justify-end items-end gap-2.5",
                InviteMemberButton { on_click: on_invite_click }
                if can_edit_team {
                    CreateGroupButton { on_click: on_create_click }
                }
            }

            ListGroups {
                groups,
                can_delete: can_edit_team,
                on_delete: on_delete_group,
            }
        }
        PopupZone {}
    }
}

#[component]
fn InviteMemberButton(on_click: EventHandler<MouseEvent>) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    rsx! {
        div {
            class: "cursor-pointer flex flex-row w-fit justify-start items-center px-4 py-3 bg-white border border-foreground rounded-[100px] gap-1",
            onclick: on_click,
            icons::user::User { width: "16", height: "16" }
            div { class: "font-bold text-base/[22px] text-neutral-900 light:text-black",
                "{tr.invite_member}"
            }
        }
    }
}

#[component]
fn CreateGroupButton(on_click: EventHandler<MouseEvent>) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    rsx! {
        div {
            class: "cursor-pointer flex flex-row w-fit justify-start items-center px-4 py-3 bg-white border border-foreground rounded-[100px] gap-1",
            onclick: on_click,
            icons::edit::Edit1 { width: "16", height: "16" }
            div { class: "font-bold text-base/[22px] text-neutral-900 light:text-black",
                "{tr.create_group}"
            }
        }
    }
}
