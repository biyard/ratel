use crate::features::teams::controllers::dto::TeamResponse;
use crate::features::teams::controllers::find_team::find_team_handler;
use crate::features::teams::*;
use crate::features::posts::controllers::dto::CategoryResponse;
use crate::features::posts::controllers::list_categories::list_categories_handler;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::common::*;

#[component]
pub fn TeamLayout(teamname: String) -> Element {
    crate::common::contexts::TeamContext::init();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let logged_in = user_ctx().user.is_some();

    // Provide selected category context shared with child routes
    use_context_provider(|| Signal::new(Option::<String>::None));

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
            div { class: "hidden tablet:flex",
                TeamSidemenu { key: "{teamname}", teamname: teamname.clone(), logged_in }
            }
            div { class: "flex flex-col min-w-0 min-h-0",
                div { class: "flex overflow-auto flex-1 p-5 w-full bg-background rounded-tl-[10px]",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn TeamSidemenu(teamname: String, logged_in: bool) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let nav = use_navigator();
    let user = user_ctx().user.clone().unwrap_or_default();

    // Selected category context (shared with child routes)
    let mut selected_category = use_context::<Signal<Option<String>>>();

    // Load team info
    let resource = use_loader(use_reactive((&teamname,), |(name,)| async move {
        Ok::<_, crate::features::teams::Error>(find_team_handler(name).await.map_err(|e| e.to_string()))
    }))?;

    // Load categories
    let categories = use_resource(|| async move {
        list_categories_handler(None).await.map(|r| r.items).unwrap_or_default()
    });

    // Hook must be at top level - not inside conditionals
    let mut show_user_menu = use_signal(|| false);

    let data = resource.read();
    let fallback_team = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|team| team.username == teamname).cloned()
    };

    let render_menu = |profile_url: String,
                       display_name: String,
                       permissions_vec: Vec<u8>,
                       _teams: Vec<crate::common::contexts::TeamItem>| {
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
            div { class: "flex flex-col w-full h-full overflow-hidden",
                // Header: avatar + name
                div { class: "flex items-center justify-between px-4 py-4 shrink-0",
                    div { class: "flex items-center gap-3 min-w-0",
                        if !profile_url.is_empty() {
                            img {
                                src: "{profile_url}",
                                alt: "{display_name}",
                                class: "w-10 h-10 rounded-[10px] object-cover object-top shrink-0",
                            }
                        } else {
                            div { class: "w-10 h-10 rounded-[10px] bg-neutral-600 shrink-0" }
                        }
                        span { class: "font-bold text-base text-text-primary truncate", "{display_name}" }
                    }
                }

                // Auth buttons (guest only)
                if !logged_in {
                    div { class: "flex flex-col gap-2 px-4 pb-4 shrink-0",
                        Link {
                            to: "/auth/",
                            class: "flex items-center justify-center w-full py-2.5 rounded-full bg-primary text-[#000000] font-semibold text-sm transition-opacity hover:opacity-90",
                            "Sign up"
                        }
                        Link {
                            to: "/auth/",
                            class: "flex items-center justify-center w-full py-2.5 rounded-full border border-border text-text-primary font-semibold text-sm transition-colors hover:bg-white/5",
                            "Log in"
                        }
                    }
                }

                // Category section
                div { class: "flex flex-col flex-1 overflow-y-auto px-3 pb-4",
                    span { class: "px-2 pb-2 text-xs font-semibold text-foreground-muted uppercase tracking-wider",
                        "Category"
                    }

                    // "All" item
                    {
                        let is_active = selected_category().is_none();
                        let active_class = if is_active { "bg-white/10 text-text-primary" } else { "text-foreground-muted hover:bg-white/5 hover:text-text-primary" };
                        rsx! {
                            button {
                                class: "flex items-center gap-2.5 w-full px-2 py-2 rounded-lg text-sm font-medium transition-colors text-left {active_class}",
                                onclick: move |_| selected_category.set(None),
                                span { class: "text-foreground-muted text-base font-bold", "#" }
                                span { "All" }
                            }
                        }
                    }

                    for cat in cats.iter() {
                        {
                            let cat_name = cat.name.clone();
                            let cat_name2 = cat.name.clone();
                            let is_active = selected_category().as_deref() == Some(cat_name.as_str());
                            let active_class = if is_active { "bg-white/10 text-text-primary" } else { "text-foreground-muted hover:bg-white/5 hover:text-text-primary" };
                            rsx! {
                                button {
                                    key: "{cat_name}",
                                    class: "flex items-center gap-2.5 w-full px-2 py-2 rounded-lg text-sm font-medium transition-colors text-left {active_class}",
                                    onclick: move |_| selected_category.set(Some(cat_name2.clone())),
                                    span { class: "text-foreground-muted text-base font-bold", "#" }
                                    span { "{cat_name}" }
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
                            div { class: "relative shrink-0 border-t border-separator px-3 py-3",
                                if show_user_menu() {
                                    div {
                                        class: "fixed inset-0 z-10",
                                        onclick: move |_| show_user_menu.set(false),
                                    }
                                }
                                div { class: "flex items-center gap-3 px-2 py-2 rounded-lg hover:bg-white/5 transition-colors",
                                    if !user_profile.is_empty() {
                                        img {
                                            src: "{user_profile}",
                                            alt: "{user_display}",
                                            class: "w-9 h-9 rounded-full object-cover shrink-0",
                                        }
                                    } else {
                                        div { class: "w-9 h-9 rounded-full bg-neutral-600 shrink-0" }
                                    }
                                    div { class: "flex flex-col min-w-0 flex-1",
                                        span { class: "text-sm font-semibold text-text-primary truncate", "{user_display}" }
                                        span { class: "text-xs text-foreground-muted", "{user_role}" }
                                    }
                                    button {
                                        class: "flex items-center justify-center w-7 h-7 rounded-md hover:bg-white/10 transition-colors shrink-0 relative z-20",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            show_user_menu.toggle();
                                        },
                                        lucide_dioxus::Ellipsis {
                                            class: "w-4 h-4 [&>circle]:fill-foreground-muted [&>circle]:stroke-none",
                                        }
                                    }
                                }
                                if show_user_menu() {
                                    div {
                                        class: "absolute bottom-full left-3 right-3 mb-1 bg-popover border border-border rounded-lg shadow-lg z-30 py-1 overflow-hidden",
                                        onclick: move |e| e.stop_propagation(),
                                        button {
                                            class: "flex items-center gap-2 px-3 py-2 text-sm text-destructive hover:bg-white/10 transition-colors w-full text-left",
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
                                            lucide_dioxus::LogOut {
                                                class: "w-[15px] h-[15px] [&>path]:stroke-destructive shrink-0",
                                            }
                                            "Log out"
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
                        .find(|item| item.username == teamname)
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
                render_menu(team.profile_url.clone(), selected_label, team.permissions.clone(), teams)
            } else {
                render_menu("".to_string(), teamname.clone(), vec![], vec![])
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
                render_menu(team.profile_url.clone(), selected_label, team.permissions.clone(), teams)
            } else {
                debug!("TeamSidemenu error: {}. No fallback.", err);
                render_menu("".to_string(), teamname.clone(), vec![], vec![])
            }
        }
    }
}

translate! {
    TeamMenuTranslate;

    settings: {
        en: "Settings",
        ko: "설정",
    },
}
