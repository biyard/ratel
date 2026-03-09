use crate::features::users::*;

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

    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let mut popup = use_popup();
    let nav = use_navigator();
    let user = user_ctx().user.clone().unwrap_or_default();
    let teams = team_ctx.teams.read().clone();
    let is_self = user.username == username;
    let selected_label = if is_self {
        user.display_name.clone()
    } else {
        teams
            .iter()
            .find(|team| team.username == username)
            .map(|team| {
                if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                }
            })
            .unwrap_or_else(|| user.display_name.clone())
    };

    rsx! {
        crate::common::SideMenuContainer {
            // Profile section with team selector
            crate::common::SideMenuProfileCard {
                // Team selector dropdown
                crate::common::TeamSelector {
                    selected_label: selected_label.clone(),
                    user_display_name: user.display_name.clone(),
                    user_profile_url: user.profile_url.clone(),
                    user_href: format!("/{}", user.username),
                    teams: teams.clone(),
                    team_href_prefix: "/teams".to_string(),
                    team_href_suffix: "/home".to_string(),
                    on_select_team: move |idx| {
                        team_ctx.set_selected_index(idx);
                    },
                    on_create_team: move |title| {
                        popup.open(rsx! {
                            TeamCreationPopup {}
                        });
                        popup.with_title(title);
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
                    }
                }

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
                    Link {
                        class: "flex gap-1 hover:text-text-primary transition-colors",
                        to: "/my-follower",
                        span { class: "font-semibold text-text-primary", "{user.followers_count}" }
                        span { class: "text-text-primary", "{tr.followers}" }
                    }
                    Link {
                        class: "flex gap-1 hover:text-text-primary transition-colors",
                        to: "/my-follower",
                        span { class: "font-semibold text-text-primary", "{user.followings_count}" }
                        span { class: "text-text-primary", "{tr.following}" }
                    }
                }
            }

            // Navigation
            crate::common::SideMenuNav {
                crate::common::SideMenuLink {
                    to: format!("/{username}/posts"),
                    label: tr.my_posts,
                    icon: rsx! {
                        icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

                if user.username == username {
                    crate::common::SideMenuLink {
                        to: format!("/{username}/drafts"),
                        label: tr.drafts,
                        icon: rsx! {
                            icons::file::AddFile { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        },
                    }

                    crate::common::SideMenuLink {
                        to: format!("/{username}/spaces"),
                        label: tr.my_spaces,
                        icon: rsx! {
                            icons::user::UserGroup { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        },
                    }

                    crate::common::SideMenuLink {
                        to: format!("/{username}/credentials"),
                        label: tr.credentials,
                        icon: rsx! {
                            icons::security::ShieldGood { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        },
                    }

                    crate::common::SideMenuLink {
                        to: format!("/{username}/memberships"),
                        label: tr.membership,
                        icon: rsx! {
                            icons::shopping::Gift { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        },
                    }

                    crate::common::SideMenuLink {
                        to: format!("/{username}/rewards"),
                        label: tr.rewards,
                        icon: rsx! {
                            icons::game::Trophy { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        },
                    }

                    crate::common::SideMenuLink {
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
}
