use crate::*;
use icons::{edit, folder, game, home, settings as settings_icon, user};

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

    rsx! {
        div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
            div { class: "flex flex-col gap-2.5 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary p-4",
                div { class: "text-sm text-text-secondary", "Team" }
                div { class: "text-base font-semibold", "{teamname}" }
            }

            nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary",
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
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
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
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
