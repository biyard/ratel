use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RewardRole {
    Admin,
    Editor,
    Viewer,
}

// FIXME: Fetch profile image instead of using a hardcoded default.
const DEFAULT_PROFILE_IMAGE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn GeneralPage(space_id: SpacePartition) -> Element {
    // FIXME: Use space_id when space-scoped data is added.
    let _ = space_id;
    let mut selected_role = use_signal(|| RewardRole::Admin);
    let mut allow_connections = use_signal(|| true);
    let mut email_input = use_signal(String::new);
    let mut invited_emails =
        use_signal(|| vec!["emailaddress".to_string(), "emailaddress".to_string()]);
    let allow_section_class = if allow_connections() {
        "flex w-full shrink-0 flex-col items-start gap-1 self-stretch rounded-[12px] border border-btn-primary-outline bg-btn-primary-bg/5 p-[17px] text-left"
    } else {
        "flex w-full shrink-0 flex-col items-start gap-1 self-stretch rounded-[12px] border border-separator bg-transparent p-[17px] text-left"
    };
    let allow_check_class = if allow_connections() {
        "flex size-6 items-center justify-center rounded-[4px] border border-btn-primary-outline bg-btn-primary-bg"
    } else {
        "flex size-6 items-center justify-center rounded-[4px] border border-gray-600 bg-transparent"
    };

    rsx! {
        div { class: "flex overflow-visible flex-col gap-5 self-start pb-6 w-full min-w-0 shrink-0 max-w-[1024px] max-tablet:gap-4 text-font-primary",
            h3 { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                "Space Setting"
            }

            div { class: "overflow-visible w-full shrink-0 rounded-[12px] bg-card",
                div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                    p { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                        "Invite New Admin"
                    }
                }

                div { class: "flex flex-col gap-5 items-start self-stretch p-5 bg-card max-mobile:p-4",
                    div { class: "flex items-start w-full gap-[10px] max-tablet:flex-col",
                        div { class: "flex flex-col flex-1 gap-2 justify-center items-start",
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                                "Email Address"
                            }
                            input {
                                class: "flex flex-col justify-center items-start px-3 py-2.5 w-full font-medium leading-6 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input sp-dash-font-raleway text-[15px] tracking-[0.5px] text-font-primary placeholder:text-card-more-muted",
                                placeholder: "admin@example.com",
                                value: email_input(),
                                oninput: move |evt| {
                                    email_input.set(evt.value().to_string());
                                },
                                onkeydown: move |evt| {
                                    if evt.key() == Key::Enter {
                                        let value = email_input().trim().to_string();
                                        if value.is_empty() {
                                            return;
                                        }

                                        invited_emails
                                            .with_mut(|emails| {
                                                emails.push(value);
                                            });
                                        email_input.set(String::new());
                                    }
                                },
                            }

                            div { class: "flex flex-wrap gap-2 items-center w-full",
                                for (idx , value) in invited_emails().iter().enumerate() {
                                    {
                                        let value = value.clone();
                                        rsx! {
                                            InviteEmailChip {
                                                key: "{idx}-{value}",
                                                value: value.clone(),
                                                on_remove: move |_| {
                                                    invited_emails
                                                        .with_mut(|emails| {
                                                            if idx < emails.len() {
                                                                emails.remove(idx);
                                                            }
                                                        });
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex flex-col flex-1 gap-2 justify-center items-start w-full",
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                                "Default Reward"
                            }
                            div { class: "grid grid-cols-3 gap-2 w-full max-mobile:grid-cols-1",
                                RewardRoleCard {
                                    selected: selected_role() == RewardRole::Admin,
                                    title: "Admin".to_string(),
                                    description: "Everything".to_string(),
                                    onclick: move |_| selected_role.set(RewardRole::Admin),
                                }
                                RewardRoleCard {
                                    selected: selected_role() == RewardRole::Editor,
                                    title: "Editor".to_string(),
                                    description: "Can edit overview and actions".to_string(),
                                    onclick: move |_| selected_role.set(RewardRole::Editor),
                                }
                                RewardRoleCard {
                                    selected: selected_role() == RewardRole::Viewer,
                                    title: "Viewer".to_string(),
                                    description: "Read-only".to_string(),
                                    onclick: move |_| selected_role.set(RewardRole::Viewer),
                                }
                            }
                        }
                    }

                    button {
                        r#type: "button",
                        class: "{allow_section_class}",
                        onclick: move |_| {
                            allow_connections.set(!allow_connections());
                        },
                        div { class: "flex items-start self-stretch gap-[10px]",
                            div { class: "{allow_check_class}",
                                if allow_connections() {
                                    icons::validations::Check {
                                        width: "24",
                                        height: "24",
                                        class: "text-btn-primary-text [&>path]:fill-none [&>path]:stroke-current",
                                    }
                                }
                            }
                            p { class: "font-medium leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-font-primary",
                                "Allow administrators to invite their connections"
                            }
                        }
                        p { class: "w-full font-normal leading-6 pl-[34px] sp-dash-font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                            "Group members can invite 1st degree connections to the group. All requests to join will still require admin approval."
                        }
                    }

                    div { class: "flex justify-end w-full max-tablet:justify-stretch",
                        Button {
                            style: ButtonStyle::Primary,
                            class: "font-normal leading-6 w-[211px] max-tablet:w-full rounded-[10px] sp-dash-font-raleway text-[15px] tracking-[0.5px] text-btn-primary-text",
                            "Invite"
                        }
                    }
                }
            }

            div { class: "overflow-visible w-full shrink-0 rounded-[12px] bg-card",
                div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                    p { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                        "Administrator"
                    }
                }

                div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                    AdministratorRow {
                        name: "Profile".to_string(),
                        caption: "Caption".to_string(),
                        time_ago: "1w ago".to_string(),
                    }
                    AdministratorRow {
                        name: "Profile".to_string(),
                        caption: "Caption".to_string(),
                        time_ago: "1w ago".to_string(),
                    }
                    AdministratorRow {
                        name: "Profile".to_string(),
                        caption: "Caption".to_string(),
                        time_ago: "1w ago".to_string(),
                    }
                }
            }

            div { class: "flex justify-end pt-5 w-full max-tablet:justify-stretch",
                Button {
                    style: ButtonStyle::Outline,
                    class: "flex flex-col justify-center items-center px-5 py-3 font-normal leading-6 border w-fit max-tablet:w-full gap-[10px] rounded-[10px] border-web-error sp-dash-font-raleway text-[15px] tracking-[0.5px] text-web-error hover:bg-transparent hover:border-web-error hover:text-web-error disabled:border-web-error/50 disabled:text-web-error/50",
                    "Delete Space"
                }
            }
        }
    }
}

#[component]
fn InviteEmailChip(value: String, on_remove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex gap-1 items-center pr-1 pl-3 h-7 rounded-[100px] bg-btn-primary-bg",
            span { class: "font-medium leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-btn-primary-text",
                "{value}"
            }
            button {
                class: "flex justify-center items-center rounded-full size-5 text-btn-primary-text/90",
                onclick: move |evt| on_remove.call(evt),
                "×"
            }
        }
    }
}

#[component]
fn RewardRoleCard(
    selected: bool,
    title: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let card_class = if selected {
        "flex h-[108px] w-full shrink-0 flex-col items-start gap-1 rounded-[12px] border border-btn-primary-outline bg-btn-primary-bg/5 px-4 py-[17px] text-left"
    } else {
        "flex h-[108px] w-full shrink-0 flex-col items-start gap-1 rounded-[12px] border border-separator bg-transparent px-4 py-[17px] text-left"
    };

    rsx! {
        button { class: "{card_class}", onclick: move |evt| onclick.call(evt),
            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                "{title}"
            }
            p { class: "font-normal leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                "{description}"
            }
        }
    }
}

#[component]
fn AdministratorRow(name: String, caption: String, time_ago: String) -> Element {
    rsx! {
        div { class: "flex justify-between items-center px-4 py-3 w-full border rounded-[12px] border-separator bg-card max-tablet:flex-col max-tablet:items-start max-tablet:gap-3",
            div { class: "flex items-center gap-[10px]",
                img {
                    src: "{DEFAULT_PROFILE_IMAGE}",
                    alt: "{name}",
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }

                div { class: "flex flex-col gap-1 items-start",
                    div { class: "flex gap-1 items-center",
                        p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                            "{name}"
                        }
                        icons::shapes::Badge2 { width: "18", height: "18", class: "" }
                    }
                    p { class: "font-semibold leading-4 sp-dash-font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                        "{caption}"
                    }
                }
            }

            div { class: "flex flex-col gap-1 items-end max-tablet:w-full max-tablet:flex-row max-tablet:items-center max-tablet:justify-between",
                button { class: "flex justify-center items-center text-web-font-neutral",
                    icons::edit::Edit1 {
                        width: "20",
                        height: "18",
                        class: "[&>path]:stroke-current",
                    }
                }
                p { class: "flex justify-center items-center h-4 font-semibold leading-4 w-[41px] sp-dash-font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                    "{time_ago}"
                }
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            GeneralPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-font-primary",
                "No permission"
            }
        }
    }
}
