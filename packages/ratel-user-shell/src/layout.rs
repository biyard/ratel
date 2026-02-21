use crate::*;
use icons::{file, game, settings as settings_icon, user};

#[component]
pub fn UserLayout(username: String) -> Element {
    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white max-w-desktop max-tablet:px-2.5",
            UserSidemenu { username }
            div { class: "flex flex-col grow p-5", Outlet::<Route> {} }
        }
    }
}

#[component]
fn UserSidemenu(username: String) -> Element {
    let tr: SideMenuTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
            div { class: "flex flex-col gap-2.5 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary p-4",
                div { class: "text-sm text-text-secondary", "Profile" }
                div { class: "text-base font-semibold", "{username}" }
            }

            nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary",
                UserSidemenuLink {
                    to: Route::UserPosts {
                        username: username.clone(),
                        rest: vec![],
                    },
                    label: tr.my_posts,
                    icon: rsx! {
                        file::File {
                            width: "24",
                            height: "24",
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
                UserSidemenuLink {
                    to: Route::UserDrafts {
                        username: username.clone(),
                        rest: vec![],
                    },
                    label: tr.drafts,
                    icon: rsx! {
                        file::CodeFile {
                            width: "24",
                            height: "24",
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
                UserSidemenuLink {
                    to: Route::UserSpaces {
                        username: username.clone(),
                        rest: vec![],
                    },
                    label: tr.my_spaces,
                    icon: rsx! {
                        user::UserGroup {
                            width: "24",
                            height: "24",
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
                UserSidemenuLink {
                    to: Route::UserCredentials {
                        username: username.clone(),
                        rest: vec![],
                    },
                    label: tr.credentials,
                    icon: rsx! {
                        icons::did::Age {
                            width: "24",
                            height: "24",
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
                UserSidemenuLink {
                    to: Route::UserMemberships {
                        username: username.clone(),
                        rest: vec![],
                    },
                    label: tr.membership,
                    icon: rsx! {
                        game::Fire {
                            width: "24",
                            height: "24",
                            class: "text-text-primary [&>path]:stroke-neutral-500",
                        }
                    },
                }
                UserSidemenuLink {
                    to: Route::UserRewards {
                        username: username.clone(),
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
                UserSidemenuLink {
                    to: Route::UserSettings {
                        username: username.clone(),
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
fn UserSidemenuLink(to: Route, label: &'static str, icon: Element) -> Element {
    rsx! {
        Link { class: "sidemenu-link text-text-primary", to,
            span { class: "w-6 h-6 inline-flex items-center justify-center", {icon} }
            span { "{label}" }
        }
    }
}

translate! {
    SideMenuTranslate;

    my_posts: {
        en: "My Posts",
        ko: "내 게시물",
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

    membership: {
        en: "Membership",
        ko: "멤버십",
    },

    rewards: {
        en: "Rewards",
        ko: "보상",
    },

    settings: {
        en: "Settings",
        ko: "설정",
    },
}
