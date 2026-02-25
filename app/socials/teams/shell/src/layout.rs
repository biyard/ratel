use crate::controllers::dto::TeamResponse;
use crate::controllers::find_team::find_team_handler;
use crate::*;
use icons::{edit, folder, game, home, settings as settings_icon, user};
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn TeamLayout(teamname: String) -> Element {
    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white max-w-desktop max-tablet:px-2.5",
            TeamSidemenu { teamname }
            div { class: "flex flex-col grow p-5", Outlet::<Route> {} }
        }
    }
}

#[component]
fn TeamSidemenu(teamname: String) -> Element {
    let tr: TeamMenuTranslate = use_translate();

    let teamname_clone = teamname.clone();
    let resource = use_server_future(move || {
        let name = teamname_clone.clone();
        async move { find_team_handler(name).await }
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    match data.as_ref() {
        Ok(team) => {
            let permissions: TeamGroupPermissions = team.permissions.unwrap_or(0).into();
            let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
            let can_post_write = is_admin || permissions.contains(TeamGroupPermission::PostWrite);
            let can_team_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);
            let can_group_edit = is_admin || permissions.contains(TeamGroupPermission::GroupEdit);

            rsx! {
                div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
                    // Profile card
                    div { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border",
                        div { class: "flex flex-col items-center gap-2",
                            if let Some(ref url) = team.profile_url {
                                if !url.is_empty() {
                                    img {
                                        src: "{url}",
                                        alt: "{team.nickname}",
                                        class: "w-16 h-16 rounded-full object-cover object-top",
                                    }
                                } else {
                                    div { class: "w-16 h-16 bg-neutral-500 rounded-full" }
                                }
                            } else {
                                div { class: "w-16 h-16 bg-neutral-500 rounded-full" }
                            }
                            div { class: "text-base font-medium text-c-primary text-center",
                                "{team.nickname}"
                            }
                            if !team.html_contents.is_empty() {
                                div { class: "text-sm text-c-secondary text-center",
                                    "{team.html_contents}"
                                }
                            }
                        }
                    }

                    // Navigation
                    nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary",
                        // Home - always visible
                        TeamSidemenuLink {
                            to: Route::TeamHome {
                                teamname: teamname.clone(),
                                rest: vec![],
                            },
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
                            TeamSidemenuLink {
                                to: Route::TeamDraft {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
                            TeamSidemenuLink {
                                to: Route::TeamGroup {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
                            TeamSidemenuLink {
                                to: Route::TeamMember {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
                            TeamSidemenuLink {
                                to: Route::TeamDao {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
                            TeamSidemenuLink {
                                to: Route::TeamReward {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
                            TeamSidemenuLink {
                                to: Route::TeamSetting {
                                    teamname: teamname.clone(),
                                    rest: vec![],
                                },
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
        }
        Err(_) => {
            rsx! {
                div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
                    div { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary text-center",
                        "Team not found"
                    }
                }
            }
        }
    }
}

#[component]
fn TeamSidemenuLink(to: Route, label: &'static str, icon: Element) -> Element {
    rsx! {
        Link { class: "sidemenu-link text-text-primary", to,
            span { class: "w-6 h-6 inline-flex items-center justify-center", {icon} }
            span { "{label}" }
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
