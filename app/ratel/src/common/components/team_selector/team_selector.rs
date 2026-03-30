use crate::*;

#[component]
pub fn TeamSelector() -> Element {
    let tr: TeamSelectorTranslate = use_translate();
    let mut open = use_signal(|| false);
    let mut team_ctx = use_team_context();
    let teams = team_ctx.teams();
    let mut popup = use_popup();
    let nav = use_navigator();

    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone().unwrap_or_default();

    rsx! {
        div { class: "relative",
            // Trigger button - shows current user/team name with chevron
            button {
                class: "flex justify-between items-center py-2 px-2 w-full cursor-pointer focus:outline-none",
                onclick: move |_| {
                    open.set(!open());
                },
                span { class: "font-bold text-[18px] text-text-primary truncate", "{user.display_name}" }
                icons::arrows::ChevronDown {
                    width: "16",
                    height: "16",
                    class: "[&>path]:stroke-text-primary",
                }
            }

            if open() {
                // Backdrop overlay to close on click outside
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| {
                        open.set(false);
                    },
                }

                div { class: "absolute left-0 top-full p-2 w-full rounded-lg border min-w-[200px] border-divider bg-background z-999",
                    // Teams label
                    div { class: "py-1 px-2 text-xs font-medium text-text-primary",
                        "{tr.teams}"
                    }

                    // Scrollable team list
                    div { class: "overflow-y-auto pr-1 -mr-1 max-h-[300px]",
                        // User entry (personal profile)
                        Link {
                            class: "flex gap-2 items-center py-2 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                            to: Route::UserHomeRoot {
                                username: user.username,
                            },
                            onclick: move |_| {
                                open.set(false);
                            },
                            if !user.profile_url.is_empty() {
                                img {
                                    src: "{user.profile_url}",
                                    alt: "{user.display_name}",
                                    class: "object-cover object-top w-6 h-6 rounded-full",
                                }
                            } else {
                                div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
                            }
                            span { class: "text-sm text-text-primary truncate", "{user.display_name}" }
                        }

                        // Team entries
                        for (idx , team) in teams.iter().enumerate() {
                            Link {
                                class: "flex gap-2 items-center py-2 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                                to: Route::UserHomeRoot {
                                    username: team.username.clone(),
                                },
                                onclick: move |_| {
                                    open.set(false);

                                    team_ctx.set_selected_index(idx);
                                },
                                if !team.profile_url.is_empty() {
                                    img {
                                        src: "{team.profile_url}",
                                        alt: team.label(),
                                        class: "object-cover object-top w-6 h-6 rounded-full",
                                    }
                                } else {
                                    div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
                                }
                                span { class: "text-sm text-text-primary truncate", {team.label()} }
                            }
                        }
                    }

                    // Separator
                    div { class: "my-1.5 h-px bg-divider" }

                    button {
                        class: "flex gap-2 items-center py-2 px-2 w-full text-sm text-left rounded-md cursor-pointer text-text-primary hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);

                            popup.open(rsx! {
                                TeamCreationPopup {}
                            }).with_title(tr.create_team.to_string());
                        },
                        "{tr.create_team}"
                    }

                    button {
                        class: "flex gap-2 items-center py-2 px-2 w-full text-sm text-left rounded-md cursor-pointer text-text-primary hover:bg-hover",
                        onclick: move |_| async move {
                            open.set(false);
                            let _ = crate::features::auth::controllers::logout_handler().await;
                            nav.push("/");
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.location().reload();
                                }
                            }
                        },
                        "{tr.logout}"
                    }
                }
            }
        }
    }
}

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
