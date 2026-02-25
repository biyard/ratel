use crate::*;
use ratel_auth::LoginModal;

translate! {
    MobileMenuTranslate;

    my_posts: {
        en: "My Posts",
        ko: "내 게시글",
    },

    drafts: {
        en: "Drafts",
        ko: "임시글",
    },

    my_spaces: {
        en: "My Spaces",
        ko: "내 스페이스",
    },

    credentials: {
        en: "Credentials",
        ko: "자격 증명",
    },

    settings: {
        en: "Settings",
        ko: "설정",
    },

    user: {
        en: "User",
        ko: "사용자",
    },

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

    home: {
        en: "Home",
        ko: "홈",
    },

    sign_in: {
        en: "Sign In",
        ko: "로그인",
    },

    join_the_movement: {
        en: "Join the Movement",
        ko: "참여하기",
    },
}

#[component]
pub fn MobileSideMenu(is_open: Signal<bool>) -> Element {
    let tr: MobileMenuTranslate = use_translate();
    let user_ctx = ratel_auth::hooks::use_user_context();
    let team_ctx = use_team_context();
    let mut popup = use_popup();
    let nav = use_navigator();

    if !is_open() {
        return rsx! {};
    }

    let logged_in = user_ctx().is_logged_in();

    if !logged_in {
        // Unauthenticated menu
        return rsx! {
            div { class: "hidden fixed left-0 z-50 w-screen top-[var(--header-height)] h-[calc(100vh-var(--header-height))] bg-bg max-tablet:block",
                div { class: "flex overflow-y-auto flex-col gap-6 py-6 px-4 w-full h-full",
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: "/",
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.home}"
                    }

                    div { class: "h-px bg-divider" }

                    button {
                        class: "py-2.5 px-3 w-full text-base text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            is_open.set(false);
                            popup.open(rsx! {
                                LoginModal {}
                            }).with_title(tr.join_the_movement);
                        },
                        "{tr.sign_in}"
                    }
                }
            }
        };
    }

    let user = user_ctx().user.clone().unwrap_or_default();
    let username = user.username.clone();
    let teams = team_ctx.teams.read().clone();

    rsx! {
        div { class: "hidden fixed left-0 z-50 w-screen top-[var(--header-height)] h-[calc(100vh-var(--header-height))] bg-bg max-tablet:block",
            div { class: "flex overflow-y-auto flex-col gap-6 py-6 px-4 w-full h-full",
                // User navigation links
                div { class: "flex flex-col gap-2",
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: format!("/{}/posts", username),
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.my_posts}"
                    }
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: format!("/{}/drafts", username),
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.drafts}"
                    }
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: format!("/{}/spaces", username),
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.my_spaces}"
                    }
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: format!("/{}/credentials", username),
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.credentials}"
                    }
                    Link {
                        class: "py-2.5 px-3 w-full text-base rounded-md text-c-secondary hover:bg-hover",
                        to: format!("/{}/settings", username),
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        "{tr.settings}"
                    }
                }

                // Divider
                div { class: "h-px bg-divider" }

                // User profile section
                div { class: "flex flex-col gap-3",
                    div { class: "px-2 text-xs text-c-secondary", "{tr.user}" }
                    Link {
                        class: "flex gap-3 items-center py-2.5 px-3 rounded-md hover:bg-hover",
                        to: "/",
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        if !user.profile_url.is_empty() {
                            img {
                                src: "{user.profile_url}",
                                alt: "{user.display_name}",
                                class: "object-cover object-top w-8 h-8 rounded-full",
                            }
                        } else {
                            div { class: "w-8 h-8 rounded-full bg-neutral-600" }
                        }
                        span { class: "text-base text-c-secondary", "{user.display_name}" }
                    }
                }

                // Teams section
                if !teams.is_empty() {
                    div { class: "h-px bg-divider" }

                    div { class: "flex flex-col gap-3",
                        div { class: "px-2 text-xs text-c-secondary", "{tr.teams}" }
                        div { class: "flex flex-col gap-2",
                            for team in teams.iter() {
                                Link {
                                    class: "flex gap-3 items-center py-2.5 px-3 rounded-md hover:bg-hover",
                                    to: format!("/teams/{}", team.username),
                                    onclick: move |_| {
                                        is_open.set(false);
                                    },
                                    if !team.profile_url.is_empty() {
                                        img {
                                            src: "{team.profile_url}",
                                            alt: "{team.nickname}",
                                            class: "object-cover object-top w-8 h-8 rounded-full",
                                        }
                                    } else {
                                        div { class: "w-8 h-8 rounded-full bg-neutral-600" }
                                    }
                                    span { class: "text-base text-c-secondary", "{team.nickname}" }
                                }
                            }
                        }
                    }
                }

                // Divider
                div { class: "h-px bg-divider" }

                // Actions
                div { class: "flex flex-col gap-2",
                    button {
                        class: "py-2.5 px-3 w-full text-base text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            is_open.set(false);
                            popup.open(rsx! {
                                TeamCreationPopup {}
                            }).with_title(tr.create_team);
                        },
                        "{tr.create_team}"
                    }

                    button {
                        class: "py-2.5 px-3 w-full text-base text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            is_open.set(false);
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
