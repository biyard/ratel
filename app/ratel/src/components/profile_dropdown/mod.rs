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

    my_profile: {
        en: "My Profile",
        ko: "내 프로필",
    },
}

#[component]
pub fn ProfileDropdown() -> Element {
    let tr: ProfileDropdownTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let team_ctx = use_team_context();
    let mut open = use_signal(|| false);
    let mut popup = use_popup();
    let nav = use_navigator();

    let user = user_ctx().user.clone();
    let Some(user) = user else {
        return rsx! {
            div {}
        };
    };

    let profile_url = user.profile_url.clone();
    let display_name = user.display_name.clone();

    let teams = team_ctx.teams.read().clone();

    rsx! {
        div { class: "relative",
            // Trigger button
            button {
                class: "flex flex-col justify-center items-center p-2.5 cursor-pointer group",
                onclick: move |_| {
                    open.set(!open());
                },
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "User Profile",
                        class: "object-cover w-6 h-6 rounded-full",
                    }
                } else {
                    div { class: "w-6 h-6 rounded-full bg-neutral-500" }
                }
                span { class: "font-medium transition-colors text-menu-text text-[15px] max-w-20 truncate group-hover:text-menu-text/80",
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

                div { class: "absolute right-0 top-full p-2.5 rounded-lg border w-[250px] border-divider bg-bg z-999",
                    // Teams label
                    div { class: "px-2 text-xs text-c-secondary py-1", "{tr.teams}" }

                    // Scrollable team list
                    div { class: "overflow-y-auto pr-2 -mr-2 max-h-[300px]",
                        // User entry (index 0)
                        Link {
                            class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                            to: format!("/"),
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
                                div { class: "w-6 h-6 rounded-full bg-neutral-600" }
                            }
                            span { class: "text-sm text-c-secondary truncate", "{user.display_name}" }
                        }

                        // Team entries
                        for team in teams.iter() {
                            Link {
                                class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                                to: format!("/{}/home", team.username),
                                onclick: move |_| {
                                    open.set(false);
                                },
                                if !team.profile_url.is_empty() {
                                    img {
                                        src: "{team.profile_url}",
                                        alt: "{team.nickname}",
                                        class: "object-cover object-top w-6 h-6 rounded-full",
                                    }
                                } else {
                                    div { class: "w-6 h-6 rounded-full bg-neutral-600" }
                                }
                                span { class: "text-sm text-c-secondary truncate", "{team.nickname}" }
                            }
                        }
                    }

                    // Separator
                    div { class: "my-2 h-px bg-divider" }

                    // My Profile
                    Link {
                        class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                        to: Route::GlobalPlayerProfilePage {},
                        onclick: move |_| {
                            open.set(false);
                        },
                        "data-testid": "my-profile-link-mobile",
                        lucide_dioxus::User {
                            size: 16,
                            class: "shrink-0 [&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary",
                        }
                        span { class: "text-sm text-c-secondary", "{tr.my_profile}" }
                    }

                    // Separator
                    div { class: "my-2 h-px bg-divider" }

                    // Create Team
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
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
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            spawn(async move {
                                let _ = crate::features::auth::controllers::logout_handler().await;
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
