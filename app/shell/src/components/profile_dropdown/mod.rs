use crate::*;

translate! {
    ProfileDropdownTranslate;

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
pub fn ProfileDropdown() -> Element {
    let tr: ProfileDropdownTranslate = use_translate();
    let user_ctx = ratel_auth::hooks::use_user_context();
    let team_ctx = use_team_context();
    let mut open = use_signal(|| false);
    let mut popup = use_popup();
    let nav = use_navigator();

    let user = user_ctx().user.clone();
    let Some(user) = user else {
        return rsx! { div {} };
    };

    let profile_url = user.profile_url.clone();
    let display_name = user.display_name.clone();

    let teams = team_ctx.teams.read().clone();

    rsx! {
        div { class: "relative",
            // Trigger button
            button {
                class: "flex flex-col items-center justify-center p-2.5 group cursor-pointer",
                onclick: move |_| {
                    open.set(!open());
                },
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "User Profile",
                        class: "w-6 h-6 rounded-full object-cover",
                    }
                } else {
                    div { class: "w-6 h-6 bg-neutral-500 rounded-full" }
                }
                span { class: "text-menu-text group-hover:text-menu-text/80 text-[15px] font-medium transition-colors max-w-20 truncate",
                    "{display_name}"
                }
            }

            // Dropdown
            if open() {
                // Backdrop to close on click outside
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| {
                        open.set(false);
                    },
                }

                div { class: "absolute right-0 top-full w-[250px] rounded-lg border border-divider bg-bg p-2.5 z-999",
                    // Teams label
                    div { class: "text-xs text-c-secondary px-2 py-1",
                        "{tr.teams}"
                    }

                    // Scrollable team list
                    div { class: "max-h-[300px] overflow-y-auto pr-2 -mr-2",
                        // User entry (index 0)
                        Link {
                            class: "flex items-center gap-2 w-full px-2 py-1.5 hover:bg-hover rounded-md cursor-pointer",
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
                                div { class: "w-6 h-6 bg-neutral-600 rounded-full" }
                            }
                            span { class: "text-sm text-c-secondary truncate",
                                "{user.display_name}"
                            }
                        }

                        // Team entries
                        for team in teams.iter() {
                            Link {
                                class: "flex items-center gap-2 w-full px-2 py-1.5 hover:bg-hover rounded-md cursor-pointer",
                                to: format!("/teams/{}", team.username),
                                onclick: move |_| {
                                    open.set(false);
                                },
                                if !team.profile_url.is_empty() {
                                    img {
                                        src: "{team.profile_url}",
                                        alt: "{team.nickname}",
                                        class: "w-6 h-6 rounded-full object-cover object-top",
                                    }
                                } else {
                                    div { class: "w-6 h-6 bg-neutral-600 rounded-full" }
                                }
                                span { class: "text-sm text-c-secondary truncate",
                                    "{team.nickname}"
                                }
                            }
                        }
                    }

                    // Separator
                    div { class: "my-2 bg-divider h-px" }

                    // Create Team
                    button {
                        class: "w-full px-2 py-1.5 hover:bg-hover rounded-md text-sm text-c-secondary cursor-pointer text-left",
                        onclick: move |_| {
                            open.set(false);
                            popup
                                .open(rsx! { TeamCreationPopup {} })
                                .with_title(tr.create_team);
                        },
                        "{tr.create_team}"
                    }

                    // Logout
                    button {
                        class: "w-full px-2 py-1.5 hover:bg-hover rounded-md text-sm text-c-secondary cursor-pointer text-left",
                        onclick: move |_| {
                            open.set(false);
                            spawn(async move {
                                let _ = ratel_auth::controllers::logout_handler().await;
                                nav.push("/");
                                // Force page reload to clear auth state
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
