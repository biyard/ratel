use super::super::dto::*;
use super::super::*;

use icons::{folder, validations};

#[component]
pub fn ListGroups(
    groups: Vec<TeamGroupResponse>,
    can_delete: bool,
    on_delete: EventHandler<String>,
) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    let mut open_menu = use_signal(|| Option::<String>::None);

    let rows = groups
        .into_iter()
        .filter(|group| !is_blocked_text(&group.name))
        .map({
            let mut open_menu = open_menu.clone();
            move |group| {
            let group_id = group.id.clone();
            let group_pw = format!("group-item-{}", group_id);
            let options_pw = format!("group-options-{}", group_id);
            let delete_pw = format!("delete-group-{}", group_id);
            let menu_open = open_menu().as_ref() == Some(&group_id);

            let on_toggle_menu = {
                let mut open_menu = open_menu.clone();
                let group_id = group_id.clone();
                move |_| {
                    if open_menu().as_ref() == Some(&group_id) {
                        open_menu.set(None);
                    } else {
                        open_menu.set(Some(group_id.clone()));
                    }
                }
            };

            let on_delete_group = {
                let mut open_menu = open_menu.clone();
                let on_delete = on_delete.clone();
                let group_id = group_id.clone();
                move |_| {
                    open_menu.set(None);
                    on_delete.call(group_id.clone());
                }
            };

            rsx! {
                div {
                    key: "{group_id}",
                    class: "flex flex-row w-full h-fit justify-between items-center bg-transparent rounded-sm border border-card-enable-border p-5",
                    div { class: "flex flex-row w-fit gap-[15px]",
                        folder::Folder {
                            width: "48",
                            height: "48",
                            class: "w-12 h-12 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                        }
                        div { class: "flex flex-col justify-between items-start",
                            div { class: "font-bold text-text-primary text-base/[20px]",
                                {group.name}
                            }
                            div { class: "font-semibold text-desc-text text-sm/[20px]",
                                "{group.members} {tr.member}"
                            }
                        }
                    }

                    if can_delete {
                        div { class: "relative",
                            button {
                                class: "p-1 hover:bg-hover rounded-full focus:outline-none transition-colors",
                                onclick: on_toggle_menu,
                                validations::Extra { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary [&>circle]:fill-icon-primary [&>path]:fill-transparent" }
                            }

                            if menu_open {
                                div { class: "absolute right-0 mt-2 w-40 border border-gray-700 bg-popover rounded-md shadow-lg z-10",
                                    button {
                                        class: "flex items-center w-full px-4 py-2 text-sm text-text-primary hover:bg-hover cursor-pointer",
                                        onclick: on_delete_group,
                                        {tr.delete_group}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            }
        });

    rsx! {
        div { class: "flex flex-col w-full px-4 py-5 gap-[10px] bg-component-bg rounded-lg",
            for row in rows {
                {row}
            }
        }
    }
}

fn is_blocked_text(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("test") || value.contains("테스트")
}
