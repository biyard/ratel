use crate::common::*;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::features::social::pages::setting::i18n::TeamSettingsTranslate;
use crate::features::social::*;

/// Context shared between the settings layout header and child pages.
/// Child pages (e.g. AdminPage) increment `save_trigger` to be notified
/// when the header Save button is clicked, and set `is_saving` while the
/// request is in-flight so the header button shows a loading state.
#[derive(Clone, Copy)]
pub struct SettingsSaveContext {
    pub save_trigger: Signal<u64>,
    pub is_saving: Signal<bool>,
}

#[component]
pub fn TeamSettingLayout(username: String) -> Element {
    crate::common::contexts::TeamContext::init()?;

    // Provide save context so child pages can hook into the header Save button
    let mut save_ctx = use_context_provider(|| SettingsSaveContext {
        save_trigger: Signal::new(0u64),
        is_saving: Signal::new(false),
    });

    let current_route = use_route::<Route>();
    let is_general_settings = matches!(current_route, Route::TeamSetting { .. });

    rsx! {
        div { class: "grid overflow-hidden grid-cols-1 w-full h-screen tablet:grid-cols-[250px_1fr] bg-team-bg text-text-primary",
            div { class: "hidden tablet:flex",
                SettingsSidemenu { username: username.clone() }
            }
            div { class: "flex flex-col min-w-0 min-h-0 bg-background rounded-tl-[10px]",
                // Top header bar
                div { class: "flex justify-between items-center py-4 px-6 border-b shrink-0 border-border",
                    span { class: "text-base font-bold text-text-primary", "Settings" }
                    if is_general_settings {
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
                // Scrollable content, centered
                div { class: "flex overflow-auto flex-1 justify-center py-8 px-6",
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
        let teams = (team_ctx.teams)();
        teams.iter().find(|t| t.username == username).cloned()
    };

    let (team_display, _team_profile, permissions_val) = match &team_item {
        Some(t) => {
            let mut mask = 0i64;
            for v in &t.permissions {
                mask |= 1i64 << (*v as i32);
            }
            (t.nickname.clone(), t.profile_url.clone(), mask)
        }
        None => (username.clone(), String::new(), 0i64),
    };

    let permissions: TeamGroupPermissions = permissions_val.into();
    let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
    let can_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);

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

    rsx! {
        div { class: "flex overflow-hidden flex-col w-full h-full",
            // Back to page
            div { class: "px-4 pt-4 pb-2 shrink-0",
                Link {
                    to: "{team_home_route}",
                    class: "flex gap-1.5 items-center text-sm transition-colors text-foreground-muted hover:text-text-primary",
                    lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>polyline]:stroke-current shrink-0" }
                    "{tr.back_to_page}"
                }
            }

            if !team_display.is_empty() {
                div { class: "px-5 pt-1 pb-0 shrink-0",
                    span { class: "text-xs text-foreground-muted truncate", "{team_display}" }
                }
            }

            div { class: "py-3 px-5 shrink-0",
                span { class: "text-base font-bold text-text-primary", "{tr.settings}" }
            }

            div { class: "flex overflow-y-auto flex-col flex-1 gap-0.5 px-3 pb-4",
                SettingNavItem {
                    label: tr.general_settings.to_string(),
                    route: settings_route,
                }
                SettingNavItem {
                    label: "Team management".to_string(),
                    route: management_route,
                }
            }

            div { class: "py-3 px-3 border-t shrink-0 border-separator",
                div { class: "flex gap-3 items-center py-2 px-2 rounded-lg",
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
            class: "flex gap-2 items-center py-2.5 px-3 w-full text-sm font-medium rounded-lg transition-colors {active_class}",
            "{label}"
        }
    }
}
