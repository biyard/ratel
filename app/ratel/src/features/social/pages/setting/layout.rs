use crate::common::*;
use crate::features::social::pages::setting::i18n::TeamSettingsTranslate;
use crate::features::social::*;
use super::SettingsSaveContext;

#[component]
pub fn TeamSettingLayout(username: String) -> Element {
    crate::common::contexts::TeamContext::init();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let teams_future = use_server_future(move || async move {
        crate::get_user_teams_handler(None)
            .await
            .map(|r| r.items)
            .unwrap_or_default()
    })?;
    use_effect(move || {
        if let Some(teams) = teams_future.value().read().clone() {
            team_ctx.set_teams(teams);
        }
    });

    // Provide save context so child pages can hook into the header Save button
    let mut save_ctx = use_context_provider(|| SettingsSaveContext {
        save_trigger: Signal::new(0u64),
        is_saving: Signal::new(false),
    });

    let current_route = use_route::<Route>();
    let is_general_settings = matches!(current_route, Route::TeamSetting { .. });

    // Check if user has edit permission (for showing Save button)
    let can_edit = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|t| t.username == username).map_or(false, |t| {
            let mut mask = 0i64;
            for v in &t.permissions {
                mask |= 1i64 << (*v as i32);
            }
            crate::features::social::pages::member::dto::TeamRole::from_legacy_permissions(mask)
                .is_admin_or_owner()
        })
    };

    rsx! {
        div {
            class: "grid overflow-hidden grid-cols-1 w-full h-screen tablet:grid-cols-[250px_1fr] bg-team-bg text-text-primary",
            "data-testid": "team-setting-layout",
            div { class: "hidden tablet:flex",
                SettingsSidemenu { username: username.clone() }
            }
            div { class: "flex flex-col min-w-0 min-h-0 bg-background rounded-tl-[10px]",
                // Top header bar
                div { class: "flex items-center shrink-0 px-6 py-4 border-b border-border",
                    span { class: "text-base font-bold text-text-primary", "Settings" }
                    if is_general_settings && can_edit {
                        Button {
                            size: ButtonSize::Medium,
                            style: ButtonStyle::Primary,
                            loading: (save_ctx.is_saving)(),
                            onclick: move |_| {
                                *(save_ctx.save_trigger).write() += 1;
                            },
                            "Save"
                        }
                    }
                }
                div { class: "flex overflow-auto flex-1 justify-center px-6 py-8",
                    div { class: "w-full max-w-2xl", Outlet::<Route> {} }
                }
            }
        }
    }
}

#[component]
fn SettingsSidemenu(username: String) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone().unwrap_or_default();
    let team_ctx = crate::common::contexts::use_team_context();
    let tr: TeamSettingsTranslate = use_translate();

    let team_item = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|t| t.username == username).cloned()
    };

    let (team_display, team_profile, permissions_val) = match &team_item {
        Some(t) => {
            let mut mask = 0i64;
            for v in &t.permissions {
                mask |= 1i64 << (*v as i32);
            }
            (t.nickname.clone(), t.profile_url.clone(), mask)
        }
        None => (username.clone(), String::new(), 0i64),
    };

    let role = crate::features::social::pages::member::dto::TeamRole::from_legacy_permissions(
        permissions_val,
    );
    let is_admin = role.is_owner();
    let can_edit = role.is_admin_or_owner();
    let user_role = if is_admin {
        "Creator"
    } else if can_edit {
        "Manager"
    } else {
        "Member"
    };

    let user_display = user.display_name.clone();
    let user_profile = user.profile_url.clone();
    let team_home_route = Route::TeamHome {
        username: username.clone(),
    }
    .to_string();
    let settings_route = Route::TeamSetting {
        username: username.clone(),
    }
    .to_string();
    let management_route = Route::TeamSettingMember {
        username: username.clone(),
    }
    .to_string();
    let subscription_route = Route::TeamSettingSubscription {
        username: username.clone(),
    }
    .to_string();

    rsx! {
        div { class: "flex flex-col w-full h-full overflow-hidden",
            // Back to page
            div { class: "px-4 pt-4 pb-2 shrink-0",
                Link {
                    to: "{team_home_route}",
                    class: "flex items-center gap-1.5 text-sm text-foreground-muted hover:text-text-primary transition-colors",
                    lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>polyline]:stroke-current shrink-0" }
                    "{tr.back_to_page}"
                }
            }

            if !team_display.is_empty() {
                div { class: "px-5 pt-1 pb-0 shrink-0",
                    span { class: "text-xs text-foreground-muted truncate", "{team_display}" }
                }
            }

            div { class: "px-5 py-3 shrink-0",
                span { class: "text-base font-bold text-text-primary", "{tr.settings}" }
            }

            div { class: "flex flex-col flex-1 overflow-y-auto px-3 pb-4 gap-0.5",
                if can_edit {
                    SettingNavItem {
                        label: tr.general_settings.to_string(),
                        route: settings_route,
                    }
                }
                SettingNavItem {
                    label: tr.team_management.to_string(),
                    route: management_route,
                }
                if is_admin {
                    SettingNavItem {
                        label: tr.subscription.to_string(),
                        route: subscription_route,
                    }
                }
            }

            div { class: "shrink-0 border-t border-separator px-3 py-3",
                div { class: "flex items-center gap-3 px-2 py-2 rounded-lg",
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
                        span { class: "text-sm font-semibold text-text-primary truncate",
                            "{user_display}"
                        }
                        span { class: "text-xs text-foreground-muted", "{user_role}" }
                    }
                }
            }
        }
    }
}

#[component]
fn SettingNavItem(label: String, route: String) -> Element {
    let current_route = use_route::<Route>();
    let is_active = current_route.to_string() == route;
    let active_class = if is_active {
        "bg-white/10 text-text-primary"
    } else {
        "text-foreground-muted hover:bg-white/5 hover:text-text-primary"
    };

    rsx! {
        Link {
            to: "{route}",
            class: "flex items-center gap-2 w-full px-3 py-2.5 rounded-lg text-sm font-medium transition-colors {active_class}",
            "{label}"
        }
    }
}
