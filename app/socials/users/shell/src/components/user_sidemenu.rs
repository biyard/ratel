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

    followers: {
        en: "Followers",
        ko: "팔로워",
    },

    following: {
        en: "Following",
        ko: "팔로잉",
    },
}

#[component]
pub fn UserSidemenu(username: String) -> Element {
    let tr: UserSidemenuTranslate = use_translate();

    let user_ctx = ratel_auth::hooks::use_user_context();
    let user = user_ctx().user.clone().unwrap_or_default();

    rsx! {
        div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0",
            // Profile section with team selector
            div { class: "flex flex-col gap-5 px-4 py-5 w-full border rounded-[10px] bg-card-bg border-card-border",
                // Team selector dropdown
                TeamSelector { username: username.clone() }

                // Profile image
                div { class: "relative",
                    if !user.profile_url.is_empty() {
                        img {
                            src: "{user.profile_url}",
                            alt: "{user.display_name}",
                            class: "w-20 h-20 rounded-full border-2 object-cover object-top",
                        }
                    } else {
                        div { class: "w-20 h-20 rounded-full border border-neutral-500 bg-neutral-500" }
                    }
                }

                // Display name
                div { class: "font-medium text-text-primary", "{user.display_name}" }

                // Description
                if !user.description.is_empty() {
                    div { class: "text-xs text-text-primary", "{user.description}" }
                }

                // Followers/Following counts
                div { class: "flex gap-4 text-sm",
                    div { class: "flex gap-1",
                        span { class: "font-semibold text-text-primary", "{user.followers_count}" }
                        span { class: "text-text-primary", "{tr.followers}" }
                    }
                    div { class: "flex gap-1",
                        span { class: "font-semibold text-text-primary", "{user.followings_count}" }
                        span { class: "text-text-primary", "{tr.following}" }
                    }
                }
            }

            // Navigation
            nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary",
                SidemenuLink {
                    to: format!("/{username}/posts"),
                    label: tr.my_posts,
                    icon: rsx! {
                        icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/drafts"),
                    label: tr.drafts,
                    icon: rsx! {
                        icons::file::AddFile { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/spaces"),
                    label: tr.my_spaces,
                    icon: rsx! {
                        icons::user::UserGroup { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/credentials"),
                    label: tr.credentials,
                    icon: rsx! {
                        icons::security::ShieldGood { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/memberships"),
                    label: tr.membership,
                    icon: rsx! {
                        icons::shopping::Gift { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/rewards"),
                    label: tr.rewards,
                    icon: rsx! {
                        icons::game::Trophy { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                SidemenuLink {
                    to: format!("/{username}/settings"),
                    label: tr.settings,
                    icon: rsx! {
                        icons::settings::Settings { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
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
            class: "flex items-center gap-3 px-2 py-2.5 hover:bg-hover rounded-md text-text-primary",
            to,
            {icon}
            span { "{label}" }
        }
    }
}
