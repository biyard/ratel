use super::super::*;
use dioxus::prelude::*;

use super::super::components::{CreateGroupModal, ListGroups};
use crate::features::social::pages::member::components::{InviteMemberModal, InviteResult};
use super::super::controllers::{create_group_handler, delete_group_handler, list_groups_handler};
use super::super::dto::*;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn AdminPage(username: String, team_pk: TeamPartition, permissions: i64) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    use_context_provider(|| PopupService::new());
    let mut popup = use_popup();

    let perms: TeamGroupPermissions = permissions.into();
    let can_edit_group = perms.contains(TeamGroupPermission::GroupEdit);
    let can_edit_team = perms.contains(TeamGroupPermission::TeamEdit);

    let mut refresh = use_signal(|| 0u64);
    let refresh_val = refresh();
    let group_resource = use_loader(use_reactive((&team_pk, &refresh_val), move |(team_pk, _)| {
        async move {
            Ok::<_, super::super::Error>(
                list_groups_handler(team_pk, None)
                    .await
                    .map_err(|e| e.to_string()),
            )
        }
    }))?;

    let data = group_resource.read();
    let groups = match data.as_ref() {
        Ok(list) => list.items.clone(),
        Err(_) => vec![],
    };

    let on_invite_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let username = username.clone();
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
                    let _ = result.role;
                    if result.total_added > 0 {
                        refresh.set(refresh() + 1);
                    }
                }
            };
            popup.open(rsx! {
                InviteMemberModal {
                    team_pk: team_pk.clone(),
                    username: username.clone(),
                    on_close,
                    on_invited,
                }
            });
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
                move |payload: super::super::components::CreateGroupPayload| {
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
        div { class: "flex flex-col gap-2.5 w-full",
            div { class: "flex flex-row gap-2.5 justify-end items-end w-full",
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
            class: "flex flex-row gap-1 justify-start items-center py-3 px-4 bg-white border cursor-pointer w-fit border-foreground rounded-[100px]",
            onclick: on_click,
            icons::user::User {
                class: "w-4 h-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                width: "16",
                height: "16",
            }
            div { class: "font-bold text-base/[22px] text-neutral-900 light:text-black",
                {tr.invite_member}
            }
        }
    }
}

#[component]
fn CreateGroupButton(on_click: EventHandler<MouseEvent>) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    rsx! {
        div {
            class: "flex flex-row gap-1 justify-start items-center py-3 px-4 bg-white border cursor-pointer w-fit border-foreground rounded-[100px]",
            onclick: on_click,
            icons::edit::Edit1 {
                width: "16",
                height: "16",
                class: "w-4 h-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
            }
            div { class: "font-bold text-base/[22px] text-neutral-900 light:text-black",
                {tr.create_group}
            }
        }
    }
}
