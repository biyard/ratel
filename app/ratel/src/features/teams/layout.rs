use crate::features::teams::controllers::dto::TeamResponse;
use crate::features::teams::controllers::find_team::find_team_handler;
use crate::features::teams::*;
use crate::features::posts::controllers::dto::CategoryResponse;
use crate::features::posts::controllers::list_categories::list_categories_handler;
use icons::settings as settings_icon;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

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
        div { class: "grid overflow-hidden grid-cols-1 w-full h-screen tablet:grid-cols-[250px_1fr] bg-bg text-white",
            if logged_in {
                div { class: "hidden tablet:flex",
                    TeamSidemenu { key: "{teamname}", teamname: teamname.clone() }
                }
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
fn TeamSidemenu(teamname: String) -> Element {
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

    let data = resource.read();
    let fallback_team = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|team| team.username == teamname).cloned()
    };

    let render_menu = |profile_url: String,
                       display_name: String,
                       permissions_vec: Vec<u8>,
                       teams: Vec<crate::common::contexts::TeamItem>| {
        let mut mask = 0i64;
        for value in &permissions_vec {
            mask |= 1i64 << (*value as i32);
        }
        let permissions: TeamGroupPermissions = mask.into();
        let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
        let can_team_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);

        let cats: Vec<CategoryResponse> = categories.read().as_ref().cloned().unwrap_or_default();

        let settings_route = if can_team_edit {
            Some(Route::TeamSetting { teamname: teamname.clone() }.to_string())
        } else {
            None
        };

        rsx! {
            div { class: "flex flex-col w-full h-full overflow-hidden",
                // Header: avatar + name + settings icon
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
                    if let Some(settings_to) = settings_route {
                        Link {
                            to: "{settings_to}",
                            class: "flex items-center justify-center w-8 h-8 rounded-lg hover:bg-white/10 transition-colors shrink-0",
                            settings_icon::Settings {
                                width: "18",
                                height: "18",
                                class: "w-[18px] h-[18px] [&>path]:stroke-icon-primary",
                            }
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
                        let active_class = if is_active { "bg-white/10 text-text-primary" } else { "text-text-secondary hover:bg-white/5" };
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
                            let active_class = if is_active { "bg-white/10 text-text-primary" } else { "text-text-secondary hover:bg-white/5" };
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

                // Bottom: user profile + team selector
                div { class: "shrink-0 border-t border-divider px-4 py-3",
                    crate::common::TeamSelector {
                        selected_label: display_name.clone(),
                        user_display_name: user.display_name.clone(),
                        user_profile_url: user.profile_url.clone(),
                        user_href: "/".to_string(),
                        teams: teams.clone(),
                        team_href_prefix: "/teams".to_string(),
                        team_href_suffix: "/home".to_string(),
                        on_select_team: move |idx| {
                            team_ctx.set_selected_index(idx);
                        },
                        on_logout: move |_| {
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
                rsx! {
                    div { class: "flex items-center justify-center w-full h-full text-text-primary text-sm",
                        "Loading..."
                    }
                }
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
                rsx! {
                    div { class: "flex items-center justify-center w-full h-full text-text-primary text-sm",
                        "Loading..."
                    }
                }
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
