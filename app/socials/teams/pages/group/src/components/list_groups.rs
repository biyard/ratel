use crate::{controllers::TeamGroupResponse, *};

use icons::{folder, validations};

#[component]
pub fn ListGroups(
    groups: Vec<TeamGroupResponse>,
    can_delete: bool,
    on_delete: EventHandler<String>,
) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    let mut open_menu = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "flex flex-col w-full px-4 py-5 gap-[10px] bg-component-bg rounded-lg",
            for group in groups
                .into_iter()
                .filter(|group| !is_blocked_text(&group.name))
            {
                let group_id = group.id.clone();
                let group_pw = format!("group-item-{}", group_id);
                let options_pw = format!("group-options-{}", group_id);
                let delete_pw = format!("delete-group-{}", group_id);
                let menu_open = open_menu().as_ref() == Some(&group_id);

                div {
                    key: "{group_id}",
                    data_pw: group_pw,
                    class: "flex flex-row w-full h-fit justify-between items-center bg-transparent rounded-sm border border-card-enable-border p-5",
                    div { class: "flex flex-row w-fit gap-[15px]",
                        folder::Folder { width: "48", height: "48", class: "stroke-neutral-400" }
                        div { class: "flex flex-col justify-between items-start",
                            div { class: "font-bold text-text-primary text-base/[20px]",
                                "{group.name}"
                            }
                            div { class: "font-semibold text-desc-text text-sm/[20px]",
                                "{group.members} {tr.member}"
                            }
                        }
                    }

                    if can_delete {
                        div { class: "relative",
                            button {
                                data_pw: options_pw,
                                class: "p-1 hover:bg-hover rounded-full focus:outline-none transition-colors",
                                aria_label: "Group options",
                                onclick: move |_| {
                                    if menu_open {
                                        open_menu.set(None);
                                    } else {
                                        open_menu.set(Some(group_id.clone()));
                                    }
                                },
                                validations::Extra { class: "size-6 text-gray-400" }
                            }

                            if menu_open {
                                div { class: "absolute right-0 mt-2 w-40 border border-gray-700 bg-popover rounded-md shadow-lg z-10",
                                    button {
                                        data_pw: delete_pw,
                                        class: "flex items-center w-full px-4 py-2 text-sm text-text-primary hover:bg-hover cursor-pointer",
                                        onclick: move |_| {
                                            open_menu.set(None);
                                            on_delete.call(group_id.clone());
                                        },
                                        "{tr.delete_group}"
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
