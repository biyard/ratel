use crate::features::teams::controllers::dto::TeamResponse;
use crate::features::teams::controllers::find_team::find_team_handler;
use crate::features::teams::*;
use icons::{edit, folder, game, home, settings as settings_icon, user};
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn TeamLayout(teamname: String) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let logged_in = user_ctx().user.is_some();

    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white bg-bg max-w-desktop max-tablet:px-2.5",
            if logged_in {
                TeamSidemenu { key: "{teamname}", teamname: teamname.clone() }
            }
            div {
                class: "flex flex-col grow px-5",
                key: "team-content-{teamname}",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn TeamSidemenu(teamname: String) -> Element {
    let tr: TeamMenuTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let nav = use_navigator();
    let user = user_ctx().user.clone().unwrap_or_default();
    let resource = use_loader(use_reactive((&teamname,), |(name,)| async move {
        Ok::<_, crate::features::teams::Error>(find_team_handler(name).await.map_err(|e| e.to_string()))
    }))?;

    let data = resource.read();
    let fallback_team = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|team| team.username == teamname).cloned()
    };

    let render_menu = |profile_url: String,
                       display_name: String,
                       description: String,
                       permissions_vec: Vec<u8>,
                       teams: Vec<crate::common::contexts::TeamItem>| {
        let mut mask = 0i64;
        for value in &permissions_vec {
            mask |= 1i64 << (*value as i32);
        }
        let permissions: TeamGroupPermissions = mask.into();
        let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
        let can_post_write = is_admin || permissions.contains(TeamGroupPermission::PostWrite);
        let can_team_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);
        let can_group_edit = is_admin || permissions.contains(TeamGroupPermission::GroupEdit);
        debug!(
            "TeamSidemenu perms={:?} is_admin={} can_post_write={} can_team_edit={} can_group_edit={} teamname={}",
            permissions_vec, is_admin, can_post_write, can_team_edit, can_group_edit, teamname
        );

        rsx! {
            crate::common::SideMenuContainer {
                // Profile card
                crate::common::SideMenuProfileCard {
                    crate::common::TeamSelector {
                        selected_label: display_name.clone(),
                        user_display_name: user.display_name.clone(),
                        user_profile_url: user.profile_url.clone(),
                        user_href: format!("/"),
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
                    div { class: "relative",
                        if !profile_url.is_empty() {
                            img {
                                src: "{profile_url}",
                                alt: "{display_name}",
                                class: "w-20 h-20 rounded-full border-2 object-cover object-top",
                            }
                        } else {
                            div { class: "w-20 h-20 rounded-full border border-neutral-500 bg-neutral-500" }
                        }
                    }
                    div { class: "font-medium text-text-primary", "{display_name}" }
                    if !description.is_empty() {
                        div { class: "text-xs text-text-primary", "{description}" }
                    }
                }

                // Navigation
                crate::common::SideMenuNav {
                    // Home - always visible
                    crate::common::SideMenuLink {
                        to: Route::TeamHome {
                            teamname: teamname.clone(),
                        }
                            .to_string(),
                        label: tr.home,
                        icon: rsx! {
                            home::Home1 {
                                width: "24",
                                height: "24",
                                class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                            }
                        },
                    }

                    // Drafts - PostWrite or Admin
                    if can_post_write {
                        crate::common::SideMenuLink {
                            to: Route::TeamDraft {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.drafts,
                            icon: rsx! {
                                edit::EditContent {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
                        }
                    }

                    // Manage Group - TeamEdit or Admin
                    if can_team_edit {
                        crate::common::SideMenuLink {
                            to: Route::TeamGroup {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.manage_group,
                            icon: rsx! {
                                folder::Folder {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
                        }
                    }

                    // Members - GroupEdit or Admin
                    if can_group_edit {
                        crate::common::SideMenuLink {
                            to: Route::TeamMember {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.members,
                            icon: rsx! {
                                user::UserGroup {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
                        }
                    }

                    // DAO - Admin only
                    if is_admin {
                        crate::common::SideMenuLink {
                            to: Route::TeamDao {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.dao,
                            icon: rsx! {
                                game::Controller {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
                        }
                    }

                    // Rewards - Admin only
                    if is_admin {
                        crate::common::SideMenuLink {
                            to: Route::TeamReward {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.rewards,
                            icon: rsx! {
                                game::Trophy {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
                        }
                    }

                    // Settings - TeamEdit or Admin
                    if can_team_edit {
                        crate::common::SideMenuLink {
                            to: Route::TeamSetting {
                                teamname: teamname.clone(),
                            }
                                .to_string(),
                            label: tr.settings,
                            icon: rsx! {
                                settings_icon::Settings {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                                }
                            },
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
                let description = if team.html_contents.is_empty() {
                    fallback_for_ok
                        .as_ref()
                        .map(|item| item.description.clone())
                        .unwrap_or_default()
                } else {
                    team.html_contents.clone()
                };

                render_menu(
                    profile_url,
                    selected_label,
                    description,
                    permissions_vec,
                    teams,
                )
            } else if let Some(team) = fallback_for_ok {
                debug!("TeamSidemenu failed to load team from server. Falling back to context.");
                let selected_label = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };
                let teams = team_ctx.teams.read().clone();
                render_menu(
                    team.profile_url.clone(),
                    selected_label,
                    team.description.clone(),
                    team.permissions.clone(),
                    teams,
                )
            } else {
                rsx! {
                    crate::common::SideMenuContainer {
                        div { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary text-center",
                            "Loading team..."
                        }
                    }
                }
            }
        }
        Err(err) => {
            if let Some(team) = fallback_team {
                debug!(
                    "TeamSidemenu failed to load team from server: {}. Falling back to context.",
                    err
                );
                let selected_label = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };
                let teams = team_ctx.teams.read().clone();
                render_menu(
                    team.profile_url.clone(),
                    selected_label,
                    team.description.clone(),
                    team.permissions.clone(),
                    teams,
                )
            } else {
                rsx! {
                    crate::common::SideMenuContainer {
                        div { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary text-center",
                            "Loading team..."
                        }
                    }
                }
            }
        }
    }
}

translate! {
    TeamMenuTranslate;

    home: {
        en: "Home",
        ko: "홈",
    },

    drafts: {
        en: "Drafts",
        ko: "임시글",
    },

    manage_group: {
        en: "Manage Group",
        ko: "그룹 관리",
    },

    members: {
        en: "Members",
        ko: "멤버",
    },

    dao: {
        en: "DAO",
        ko: "DAO",
    },

    rewards: {
        en: "Rewards",
        ko: "리워드",
    },

    settings: {
        en: "Settings",
        ko: "설정",
    },
}
