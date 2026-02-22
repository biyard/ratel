use crate::*;

translate! {
    UserSidemenuTranslate;

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

#[component]
pub fn UserSidemenu() -> Element {
    let tr: UserSidemenuTranslate = use_translate();
    let user_ctx = ratel_auth::hooks::use_user_context();

    let logged_in = user_ctx().is_logged_in();
    if !logged_in {
        return rsx! { div {} };
    }

    let user = user_ctx().user.clone().unwrap_or_default();
    let username = user.username.clone();

    rsx! {
        div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
            // Profile section
            div { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border",
                div { class: "flex flex-col items-center gap-2",
                    if !user.profile_url.is_empty() {
                        img {
                            src: "{user.profile_url}",
                            alt: "{user.display_name}",
                            class: "w-16 h-16 rounded-full object-cover object-top",
                        }
                    } else {
                        div { class: "w-16 h-16 bg-neutral-500 rounded-full" }
                    }
                    div { class: "text-base font-medium text-c-primary text-center",
                        "{user.display_name}"
                    }
                    if !user.description.is_empty() {
                        div { class: "text-sm text-c-secondary text-center",
                            "{user.description}"
                        }
                    }
                }
            }

            // Navigation
            nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-c-primary",
                SidemenuLink {
                    to: format!("/{username}/posts"),
                    label: tr.my_posts,
                    icon: rsx! {
                        icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/drafts"),
                    label: tr.drafts,
                    icon: rsx! {
                        icons::file::AddFile { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/spaces"),
                    label: tr.my_spaces,
                    icon: rsx! {
                        icons::user::UserGroup { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/credentials"),
                    label: tr.credentials,
                    icon: rsx! {
                        icons::security::ShieldGood { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: "/membership".to_string(),
                    label: tr.membership,
                    icon: rsx! {
                        icons::shopping::Gift { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/rewards"),
                    label: tr.rewards,
                    icon: rsx! {
                        icons::game::Trophy { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/settings"),
                    label: tr.settings,
                    icon: rsx! {
                        icons::settings::Settings { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                    },
                }
            }
        }
    }
}

#[component]
fn SidemenuLink(to: String, label: &'static str, icon: Element) -> Element {
    rsx! {
        Link {
            class: "flex items-center gap-3 px-2 py-2.5 hover:bg-hover rounded-md text-c-primary",
            to: to,
            {icon}
            span { "{label}" }
        }
    }
}
