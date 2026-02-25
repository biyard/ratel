use crate::*;

translate! {
    TeamSelectorTranslate;

    teams: {
        en: "Teams",
        ko: "팀",
    },

    create_team: {
        en: "Create Team",
        ko: "팀 생성",
    },

    logout: {
        en: "Log Out",
        ko: "로그아웃",
    },
}

#[component]
pub fn TeamSelector(username: String) -> Element {
    let tr: TeamSelectorTranslate = use_translate();
    let user_ctx = ratel_auth::hooks::use_user_context();
    let mut team_ctx = common::contexts::use_team_context();
    let mut open = use_signal(|| false);
    let nav = use_navigator();
    let mut popup = use_popup();

    let user = user_ctx().user.clone();
    let Some(user) = user else {
        return rsx! {};
    };

    let teams = team_ctx.teams.read().clone();
    let display_name = user.display_name.clone();
    let selected_label = team_ctx
        .selected_team()
        .map(|team| team.nickname)
        .filter(|name| !name.is_empty())
        .unwrap_or(display_name.clone());

    rsx! {
        div { class: "relative",
            // Trigger button - shows current user/team name with chevron
            button {
                class: "w-full flex items-center justify-between px-2 py-2 focus:outline-none cursor-pointer",
                onclick: move |_| {
                    open.set(!open());
                },
                span { class: "font-bold text-[18px] text-text-primary truncate", "{selected_label}" }
                icons::arrows::ChevronDown {
                    width: "16",
                    height: "16",
                    class: "[&>path]:stroke-text-primary",
                }
            }

            // Dropdown menu
            if open() {
                // Backdrop overlay to close on click outside
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| {
                        open.set(false);
                    },
                }

                div { class: "absolute left-0 top-full w-full min-w-[200px] rounded-lg border border-divider bg-background p-2 z-999",
                    // Teams label
                    div { class: "text-xs text-text-primary px-2 py-1 font-medium",
                        "{tr.teams}"
                    }

                    // Scrollable team list
                    div { class: "max-h-[300px] overflow-y-auto pr-1 -mr-1",
                        // User entry (personal profile)
                        Link {
                            class: "flex items-center gap-2 w-full px-2 py-2 hover:bg-hover rounded-md cursor-pointer",
                            to: "/",
                            onclick: move |_| {
                                open.set(false);
                            },
                            if !user.profile_url.is_empty() {
                                img {
                                    src: "{user.profile_url}",
                                    alt: "{user.display_name}",
                                    class: "w-6 h-6 rounded-full object-cover object-top",
                                }
                            } else {
                                div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
                            }
                            span { class: "text-sm text-text-primary truncate", "{user.display_name}" }
                        }

                        // Team entries
                        for (idx , team) in teams.iter().enumerate() {
                            if !team.nickname.is_empty() {
                                Link {
                                    class: "flex items-center gap-2 w-full px-2 py-2 hover:bg-hover rounded-md cursor-pointer",
                                    to: format!("/teams/{}", team.username),
                                    onclick: move |_| {
                                        open.set(false);
                                        team_ctx.set_selected_index(idx);
                                    },
                                    if !team.profile_url.is_empty() {
                                        img {
                                            src: "{team.profile_url}",
                                            alt: "{team.nickname}",
                                            class: "w-6 h-6 rounded-full object-cover object-top",
                                        }
                                    } else {
                                        div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
                                    }
                                    span { class: "text-sm text-text-primary truncate",
                                        "{team.nickname}"
                                    }
                                }
                            }
                        }
                    }

                    // Separator
                    div { class: "my-1.5 bg-divider h-px" }

                    // Create Team
                    button {
                        class: "w-full flex items-center gap-2 px-2 py-2 hover:bg-hover rounded-md text-sm text-text-primary cursor-pointer text-left",
                        onclick: move |_| {
                            open.set(false);
                            popup.open(rsx! {
                                TeamCreationPopup {}
                            });
                            popup.with_title(tr.create_team);
                        },
                        "{tr.create_team}"
                    }

                    // Logout
                    button {
                        class: "w-full flex items-center gap-2 px-2 py-2 hover:bg-hover rounded-md text-sm text-text-primary cursor-pointer text-left",
                        onclick: move |_| {
                            open.set(false);
                            spawn(async move {
                                let _ = ratel_auth::controllers::logout_handler().await;
                                nav.push("/");
                                #[cfg(target_arch = "wasm32")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.location().reload();
                                    }
                                }
                            });
                        },
                        "{tr.logout}"
                    }
                }
            }
        }
    }
}
