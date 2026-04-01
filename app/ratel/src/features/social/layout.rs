use crate::common::*;
use crate::features::auth::{LoginModal, SignupModal};
use crate::features::posts::controllers::dto::CategoryResponse;
use crate::features::posts::controllers::list_categories::list_categories_handler;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::features::social::controllers::dto::TeamResponse;
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::*;

/// Team layout — used exclusively for team routes (home, drafts, groups, dao, members, rewards).
/// Always renders TeamSidemenu with categories sidebar.
#[component]
pub fn SocialLayout(username: String) -> Element {
    crate::common::contexts::TeamContext::init();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let logged_in = user_ctx().user.is_some();

    // Provide selected category context shared with child routes
    use_context_provider(|| Signal::new(Option::<String>::None));
    use_context_provider(|| PopupService::new());

    let _teams_loader = use_resource(move || async move {
        let user = user_ctx().user.clone();
        if user.is_some() {
            match crate::get_user_teams_handler().await {
                Ok(teams) => {
                    team_ctx.set_teams(teams);
                }
                Err(e) => {
                    debug!("Failed to load teams: {:?}", e);
                }
            }
        }
    });

    rsx! {
        div { class: "grid overflow-hidden grid-cols-1 w-full h-screen tablet:grid-cols-[250px_1fr] bg-team-bg text-text-primary",
            div { class: "hidden tablet:flex h-screen overflow-hidden",
                TeamSidemenu { key: "{username}", username: username.clone(), logged_in }
            }
            div { class: "flex flex-col min-w-0 min-h-0 overflow-hidden",
                div { class: "flex overflow-auto flex-1 p-5 w-full bg-background rounded-tl-[10px] max-tablet:p-3 max-mobile:p-2",
                    Outlet::<Route> {}
                }
            }
        }
        PopupZone {}
    }
}

/// User layout — used for user profile routes (posts, rewards, settings, etc.).
/// Includes AppMenu header and UserSidemenu.
#[component]
pub fn UserLayout(username: String) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let logged_in = user_ctx().user.is_some();

    rsx! {
        div { class: "antialiased bg-bg",
            crate::AppMenu {}
            div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white bg-bg max-w-desktop max-tablet:px-2.5",
                if logged_in {
                    UserSidemenu { username: username.clone() }
                }
                div { class: "flex flex-col px-5 grow", Outlet::<Route> {} }
            }
        }
    }
}

#[component]
fn TeamSidemenu(username: String, logged_in: bool) -> Element {
    let tr: TeamMenuTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let team_ctx = crate::common::contexts::use_team_context();
    let nav = use_navigator();
    let current_route = use_route::<Route>();
    let user = user_ctx().user.clone().unwrap_or_default();

    // Selected category context (shared with child routes)
    let mut selected_category = use_context::<Signal<Option<String>>>();

    // Load team info
    let resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, crate::features::social::Error>(
            find_team_handler(name).await.map_err(|e| e.to_string()),
        )
    }))?;

    // Load categories
    let categories = use_resource(|| async move {
        list_categories_handler(None)
            .await
            .map(|r| r.items)
            .unwrap_or_default()
    });

    // Hook must be at top level - not inside conditionals
    let mut show_user_menu = use_signal(|| false);
    let mut popup = use_popup();

    let data = resource.read();
    let fallback_team = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|team| team.username == username).cloned()
    };

    let render_menu = |profile_url: String,
                       display_name: String,
                       permissions_vec: Vec<u8>,
                       _teams: Vec<crate::common::contexts::TeamItem>| {
        let team_home_route = Route::TeamHome {
            username: username.clone(),
        }
        .to_string();
        let is_reward_page = matches!(current_route, Route::TeamReward { .. });
        let mut mask = 0i64;
        for value in &permissions_vec {
            mask |= 1i64 << (*value as i32);
        }
        let permissions: TeamGroupPermissions = mask.into();
        let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
        let can_team_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);

        let user_role = if is_admin || can_team_edit {
            "Creator"
        } else {
            "Viewer"
        };

        let cats: Vec<CategoryResponse> = categories.read().as_ref().cloned().unwrap_or_default();

        rsx! {
            div { class: "flex overflow-hidden flex-col w-full h-full",
                if is_reward_page {
                    div { class: "px-4 pt-4 pb-2 shrink-0",
                        Link {
                            to: "{team_home_route}",
                            class: "flex items-center gap-1.5 text-sm text-foreground-muted hover:text-text-primary transition-colors",
                            lucide_dioxus::ChevronLeft {
                                class: "w-4 h-4 [&>polyline]:stroke-current shrink-0",
                            }
                            "{tr.back_to_page}"
                        }
                    }
                }
                // Header: avatar + name
                div { class: "flex justify-between items-center py-4 px-4 shrink-0",
                    div { class: "flex gap-3 items-center min-w-0",
                        if !profile_url.is_empty() {
                            img {
                                src: "{profile_url}",
                                alt: "{display_name}",
                                class: "object-cover object-top w-10 h-10 rounded-[10px] shrink-0",
                            }
                        } else {
                            div { class: "w-10 h-10 rounded-[10px] bg-neutral-600 shrink-0" }
                        }
                        span { class: "text-base font-bold text-text-primary truncate",
                            "{display_name}"
                        }
                    }
                }

                // Auth buttons (guest only)
                if !logged_in {
                    div { class: "flex flex-col gap-2 px-4 pb-4 shrink-0",
                        button {
                            class: "flex justify-center items-center py-2.5 w-full text-sm font-semibold rounded-full transition-opacity hover:opacity-90 bg-primary text-[#000000]",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    SignupModal {}
                                });
                            },
                            "{tr.sign_up}"
                        }
                        button {
                            class: "flex justify-center items-center py-2.5 w-full text-sm font-semibold rounded-full border transition-colors border-border text-text-primary hover:bg-hover",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    LoginModal {}
                                });
                            },
                            "{tr.log_in}"
                        }
                    }
                }

                // Category section
                div { class: "flex overflow-y-auto flex-col flex-1 min-h-0 px-3 pb-4",
                    span { class: "px-2 pb-2 text-xs font-semibold tracking-wider uppercase text-foreground-muted",
                        "Category"
                    }

                    // "All" item
                    {
                        let is_active = selected_category().is_none();
                        let active_class = if is_active {
                            "bg-hover text-text-primary"
                        } else {
                            "text-foreground-muted hover:bg-hover hover:text-text-primary"
                        };
                        rsx! {
                            button {
                                class: "flex gap-2.5 items-center py-2 px-2 w-full text-sm font-medium text-left rounded-lg transition-colors {active_class}",
                                onclick: move |_| selected_category.set(None),
                                span { class: "text-base font-bold text-foreground-muted", "#" }
                                span { "All" }
                            }
                        }
                    }

                    for cat in cats.iter() {
                        {
                            let cat_name = cat.name.clone();
                            let cat_name2 = cat.name.clone();
                            let is_active = selected_category().as_deref() == Some(cat_name.as_str());
                            let active_class = if is_active {
                                "bg-hover text-text-primary"
                            } else {
                                "text-foreground-muted hover:bg-hover hover:text-text-primary"
                            };
                            rsx! {
                                button {
                                    key: "{cat_name}",
                                    class: "flex gap-2.5 items-center py-2 px-2 w-full text-sm font-medium text-left rounded-lg transition-colors {active_class}",
                                    onclick: move |_| selected_category.set(Some(cat_name2.clone())),
                                    span { class: "text-base font-bold text-foreground-muted", "#" }
                                    span { "{cat_name}" }
                                }
                            }
                        }
                    }
                }

                // ADMIN section — visible only to team admins/editors
                if can_team_edit {
                    div { class: "flex flex-col px-3 pb-4 shrink-0",
                        {
                            let is_draft_active = matches!(current_route, Route::TeamDraft { .. });
                            let draft_class = if is_draft_active {
                                "bg-hover text-text-primary"
                            } else {
                                "text-foreground-muted hover:bg-hover hover:text-text-primary"
                            };
                            rsx! {
                                Link {
                                    to: Route::TeamDraft { username: username.clone() },
                                    class: "flex gap-2.5 items-center py-2 px-2 w-full text-sm font-medium text-left rounded-lg transition-colors {draft_class}",
                                    lucide_dioxus::FileText {
                                        class: "w-4 h-4 [&>path]:stroke-current [&>polyline]:stroke-current [&>line]:stroke-current shrink-0",
                                    }
                                    span { "{tr.drafts}" }
                                }
                            }
                        }
                    }
                }

                // Bottom: user info + settings (logged-in only)
                if logged_in {
                    {
                        let user_display = user.display_name.clone();
                        let user_profile = user.profile_url.clone();
                        rsx! {
                            div { class: "relative py-3 px-3 border-t shrink-0 border-separator",
                                if show_user_menu() {
                                    div {
                                        class: "fixed inset-0 z-10",
                                        onclick: move |_| show_user_menu.set(false),
                                    }
                                }
                                div { class: "flex gap-3 items-center py-2 px-2 rounded-lg transition-colors hover:bg-hover",
                                    if !user_profile.is_empty() {
                                        img {
                                            src: "{user_profile}",
                                            alt: "{user_display}",
                                            class: "object-cover w-9 h-9 rounded-full shrink-0",
                                        }
                                    } else {
                                        div { class: "w-9 h-9 rounded-full bg-neutral-600 shrink-0" }
                                    }
                                    div { class: "flex flex-col flex-1 min-w-0",
                                        span { class: "text-sm font-semibold text-text-primary truncate", "{user_display}" }
                                        span { class: "text-xs text-foreground-muted", "{user_role}" }
                                    }
                                    button {
                                        class: "flex relative z-20 justify-center items-center w-7 h-7 rounded-md transition-colors shrink-0 hover:bg-hover",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            show_user_menu.toggle();
                                        },
                                        lucide_dioxus::Ellipsis {
                                            class: "w-4 h-4 [&>circle]:fill-text-primary [&>circle]:stroke-none",
                                        }
                                    }
                                }
                                if show_user_menu() {
                                    div {
                                        class: "overflow-hidden absolute right-3 left-3 bottom-full z-30 py-1 mb-1 rounded-lg border shadow-lg bg-popover border-border",
                                        onclick: move |e| e.stop_propagation(),
                                        Link {
                                            to: Route::TeamReward { username: username.clone() },
                                            class: "flex items-center gap-2 px-3 py-2 text-sm text-text-primary hover:bg-hover transition-colors w-full text-left",
                                            onclick: move |_| show_user_menu.set(false),
                                            "data-pw": "team-reward-menu-link",
                                            icons::game::Trophy {
                                                class: "w-[15px] h-[15px] [&>path]:stroke-text-primary [&>path]:fill-transparent shrink-0",
                                            }
                                            {tr.rewards}
                                        }
                                        Link {
                                            to: Route::TeamSetting { username: username.clone() },
                                            class: "flex items-center gap-2 px-3 py-2 text-sm text-text-primary hover:bg-hover transition-colors w-full text-left",
                                            onclick: move |_| show_user_menu.set(false),
                                            lucide_dioxus::Settings {
                                                class: "w-[15px] h-[15px] [&>path]:stroke-text-primary [&>line]:stroke-text-primary [&>polyline]:stroke-text-primary [&>circle]:stroke-text-primary shrink-0",
                                            }
                                            {tr.settings}
                                        }
                                        button {
                                            class: "flex gap-2 items-center py-2 px-3 w-full text-sm text-left transition-colors text-destructive hover:bg-hover",
                                            onclick: move |_| {
                                                show_user_menu.set(false);
                                                spawn(async move {
                                                    let _ = crate::features::auth::controllers::logout_handler().await;
                                                    nav.push("/");
                                                    #[cfg(target_arch = "wasm32")]
                                                    {
                                                        if let Some(window) = web_sys::window() {
                                                            let _ = window.location().reload();
                                                        }
                                                    }
                                                });
                                            },
                                            lucide_dioxus::LogOut { class: "w-[15px] h-[15px] [&>path]:stroke-destructive shrink-0" }
                                            {tr.log_out}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let fallback_for_ok = fallback_team.clone();
    match data.as_ref() {
        Ok(team) => {
            if !team.pk.is_empty() && !team.username.is_empty() {
                let permissions_vec = {
                    let from_ctx = team_ctx
                        .teams
                        .read()
                        .iter()
                        .find(|item| item.username == username)
                        .map(|item| item.permissions.clone())
                        .unwrap_or_default();

                    if !from_ctx.is_empty() {
                        from_ctx
                    } else {
                        team.permissions
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|p| p as u8)
                            .collect()
                    }
                };

                let profile_url = team.profile_url.clone().unwrap_or_default();
                let mut teams = team_ctx.teams.read().clone();
                if !teams.iter().any(|item| item.username == team.username) {
                    teams.push(crate::common::contexts::TeamItem {
                        pk: team.pk.clone(),
                        nickname: team.nickname.clone(),
                        username: team.username.clone(),
                        profile_url: profile_url.clone(),
                        user_type: UserType::Team,
                        permissions: permissions_vec.clone(),
                        description: team.html_contents.clone(),
                    });
                }
                let selected_label = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };

                render_menu(profile_url, selected_label, permissions_vec, teams)
            } else if let Some(team) = fallback_for_ok {
                debug!("TeamSidemenu falling back to context.");
                let selected_label = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };
                let teams = team_ctx.teams.read().clone();
                render_menu(
                    team.profile_url.clone(),
                    selected_label,
                    team.permissions.clone(),
                    teams,
                )
            } else {
                render_menu("".to_string(), username.clone(), vec![], vec![])
            }
        }
        Err(err) => {
            if let Some(team) = fallback_team {
                debug!("TeamSidemenu server error: {}. Falling back.", err);
                let selected_label = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };
                let teams = team_ctx.teams.read().clone();
                render_menu(
                    team.profile_url.clone(),
                    selected_label,
                    team.permissions.clone(),
                    teams,
                )
            } else {
                debug!("TeamSidemenu error: {}. No fallback.", err);
                render_menu("".to_string(), username.clone(), vec![], vec![])
            }
        }
    }
}

translate! {
    TeamMenuTranslate;

    sign_up: {
        en: "Sign up",
        ko: "회원가입",
    },

    log_in: {
        en: "Log in",
        ko: "로그인",
    },

    back_to_page: {
        en: "Back to page",
        ko: "페이지로 돌아가기",
    },

    drafts: {
        en: "Draft",
        ko: "초안",
    },

    rewards: {
        en: "Rewards",
        ko: "리워드",
    },

    settings: {
        en: "Settings",
        ko: "설정",
    },

    log_out: {
        en: "Log out",
        ko: "로그아웃",
    },

}
