use super::super::*;
use crate::common::Switch;

use crate::features::posts::types::TeamGroupPermission;

#[derive(Clone)]
pub struct CreateGroupPayload {
    pub name: String,
    pub description: String,
    pub permissions: Vec<TeamGroupPermission>,
}

#[component]
pub fn CreateGroupModal(on_create: EventHandler<CreateGroupPayload>) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    let mut group_name = use_signal(String::new);
    let mut group_description = use_signal(String::new);
    let mut group_permissions = use_signal(Vec::<TeamGroupPermission>::new);

    let mut name_required = use_signal(|| false);
    let mut option_required = use_signal(|| false);

    let permission_groups = get_permission_groups(&tr);

    let is_blocked = is_blocked_text(&group_name()) || is_blocked_text(&group_description());
    let name_len = group_name().len();
    let desc_len = group_description().len();

    let on_create_click = {
        let mut name_required = name_required.clone();
        let mut option_required = option_required.clone();
        let group_name = group_name.clone();
        let group_description = group_description.clone();
        let group_permissions = group_permissions.clone();
        move |_| {
            name_required.set(false);
            option_required.set(false);
            if group_name().is_empty() {
                name_required.set(true);
                return;
            }
            if group_permissions().is_empty() {
                option_required.set(true);
                return;
            }
            if is_blocked_text(&group_name()) || is_blocked_text(&group_description()) {
                return;
            }
            on_create.call(CreateGroupPayload {
                name: group_name(),
                description: group_description(),
                permissions: group_permissions(),
            });
        }
    };

    rsx! {
        div { class: "flex flex-col w-tablet max-w-tablet min-w-[400px] max-h-[700px] max-mobile:!w-full max-mobile:!max-w-full gap-5 overflow-y-auto px-[20px] custom-scrollbar",
            GroupName {
                tr: tr.clone(),
                value: group_name(),
                on_change: move |e: FormEvent| group_name.set(e.value()),
                length: name_len,
            }

            GroupDescription {
                tr: tr.clone(),
                value: group_description(),
                on_change: move |e: FormEvent| group_description.set(e.value()),
                length: desc_len,
            }

            GroupPermissionSelector {
                tr: tr.clone(),
                permission_groups,
                permissions: group_permissions(),
                on_toggle: move |perm| {
                    let mut next = group_permissions();
                    if next.contains(&perm) {
                        next.retain(|p| p != &perm);
                    } else {
                        next.push(perm);
                    }
                    group_permissions.set(next);
                },
                on_toggle_group: move |perms: Vec<TeamGroupPermission>| {
                    let mut next = group_permissions();
                    let all_selected = perms.iter().all(|p| next.contains(p));
                    if all_selected {
                        next.retain(|p| !perms.contains(p));
                    } else {
                        for perm in perms {
                            if !next.contains(&perm) {
                                next.push(perm);
                            }
                        }
                    }
                    group_permissions.set(next);
                },
            }

            if name_required() {
                div { class: "font-normal text-post-required-marker text-sm",
                    "{tr.group_name_required}"
                }
            } else if option_required() {
                div { class: "font-normal text-post-required-marker text-sm",
                    "{tr.group_option_required}"
                }
            }

            div { class: "flex flex-row w-full justify-end items-center px-[30px] py-[25px]",
                button {
                    class: if is_blocked { "cursor-not-allowed bg-neutral-300 flex flex-row w-fit h-fit px-[40px] py-[15px] rounded-[10px] font-bold text-bg text-base" } else { "cursor-pointer bg-primary flex flex-row w-fit h-fit px-[40px] py-[15px] rounded-[10px] font-bold text-bg text-base" },
                    onclick: on_create_click,
                    "{tr.create}"
                }
            }
        }
    }
}

#[component]
fn GroupName(
    tr: TeamGroupTranslate,
    value: String,
    on_change: EventHandler<FormEvent>,
    length: usize,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
            div { class: "flex flex-row gap-1 items-center",
                div { class: "font-bold text-[15px]/[28px] text-modal-label-text", {tr.group_name} }
                div { class: "font-normal text-base/[24px] text-[#eb5757]", "*" }
            }
            input {
                r#type: "text",
                value,
                maxlength: 100,
                placeholder: tr.group_name_hint,
                class: "w-full px-5 py-[10.5px] rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary placeholder:text-neutral-600 text-[15px]/[22.5px] outline-none",
                oninput: on_change,
            }
            div { class: "w-full text-right text-[15px]/[22.5px] text-neutral-600",
                "{length}/100"
            }
        }
    }
}

#[component]
fn GroupDescription(
    tr: TeamGroupTranslate,
    value: String,
    on_change: EventHandler<FormEvent>,
    length: usize,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
            div { class: "font-bold text-[15px]/[28px] text-modal-label-text", {tr.description} }
            textarea {
                value,
                maxlength: 100,
                placeholder: {tr.description_hint},
                class: "w-full px-5 py-[10px] rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary placeholder:text-neutral-600 text-sm outline-none resize-none",
                oninput: on_change,
            }
            div { class: "w-full text-right text-[15px]/[22.5px] text-neutral-600",
                "{length}/100"
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct PermissionItem {
    label: String,
    value: TeamGroupPermission,
}

#[derive(PartialEq, Clone)]
struct PermissionGroup {
    name: String,
    items: Vec<PermissionItem>,
}

fn get_permission_groups(tr: &TeamGroupTranslate) -> Vec<PermissionGroup> {
    vec![
        PermissionGroup {
            name: tr.permission_group_post.to_string(),
            items: vec![
                PermissionItem {
                    label: tr.permission_post_read.to_string(),
                    value: TeamGroupPermission::PostRead,
                },
                PermissionItem {
                    label: tr.permission_post_write.to_string(),
                    value: TeamGroupPermission::PostWrite,
                },
                PermissionItem {
                    label: tr.permission_post_delete.to_string(),
                    value: TeamGroupPermission::PostDelete,
                },
            ],
        },
        PermissionGroup {
            name: tr.permission_group_admin.to_string(),
            items: vec![
                PermissionItem {
                    label: tr.permission_group_edit.to_string(),
                    value: TeamGroupPermission::GroupEdit,
                },
                PermissionItem {
                    label: tr.permission_team_edit.to_string(),
                    value: TeamGroupPermission::TeamEdit,
                },
                PermissionItem {
                    label: tr.permission_team_admin.to_string(),
                    value: TeamGroupPermission::TeamAdmin,
                },
            ],
        },
    ]
}

#[component]
fn GroupPermissionSelector(
    tr: TeamGroupTranslate,
    permission_groups: Vec<PermissionGroup>,
    permissions: Vec<TeamGroupPermission>,
    on_toggle: EventHandler<TeamGroupPermission>,
    on_toggle_group: EventHandler<Vec<TeamGroupPermission>>,
) -> Element {
    let group_views = permission_groups
        .into_iter()
        .enumerate()
        .map(|(idx, group)| {
            let group_perms = group
                .items
                .iter()
                .map(|item| item.value)
                .collect::<Vec<_>>();
            let all_checked = group_perms.iter().all(|perm| permissions.contains(perm));
            let group_class = if idx != 0 {
                "flex flex-col gap-1 w-full mt-[20px]".to_string()
            } else {
                "flex flex-col gap-1 w-full".to_string()
            };
            let select_all_pw = format!("permission-select-all-{}", group.name.to_lowercase());
            (group, group_perms, all_checked, group_class, select_all_pw)
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "flex flex-col w-full gap-6",
            div { class: "text-[15px]/[28px] font-bold text-modal-label-text", {tr.permission} }
            div { class: "px-[10px]",
                for (group , group_perms , all_checked , group_class , select_all_pw) in group_views {
                    div { class: {group_class},
                        div { class: "flex justify-between items-center mb-1",
                            div { class: "text-sm/[20px] font-semibold text-modal-label-text",
                                {group.name}
                            }
                            div { class: "flex items-center gap-1",
                                span { class: "text-sm/[20px] font-semibold text-modal-label-text",
                                    {tr.select_all}
                                }
                                button {
                                    class: if all_checked { "w-5 h-5 rounded border border-primary bg-primary" } else { "w-5 h-5 rounded border border-neutral-500 bg-transparent" },
                                    onclick: move |_| {
                                        on_toggle_group.call(group_perms.clone());
                                    },
                                }
                            }
                        }
                        div { class: "flex flex-col border-neutral-800 divide-y divide-divider",
                            for item in group.items {
                                div { class: "flex justify-between items-center py-2 h-[55px]",
                                    span { class: "text-[15px]/[24px] font-normal text-text-primary",
                                        {item.label}
                                    }
                                    Switch {
                                        active: permissions.contains(&item.value),
                                        on_toggle: move |_| on_toggle.call(item.value),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_blocked_text(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("test") || value.contains("테스트")
}
