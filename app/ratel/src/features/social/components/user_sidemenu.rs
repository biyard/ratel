use crate::features::my_follower::controllers::check_follow_status::check_follow_status_handler;
use crate::features::my_follower::controllers::follow_user::follow_user;
use crate::features::my_follower::controllers::unfollow_user::unfollow_user;
use crate::features::social::*;

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

    follow: {
        en: "Follow",
        ko: "팔로우",
    },

    unfollow: {
        en: "Unfollow",
        ko: "언팔로우",
    },

    posts: {
        en: "Posts",
        ko: "게시글",
    },
}

#[component]
pub fn UserSidemenu(username: String) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone().unwrap_or_default();
    let is_self = user.username == username;

    if is_self {
        rsx! { SelfSidemenu { username } }
    } else {
        rsx! { OtherUserSidemenu { username } }
    }
}

/// Sidemenu for viewing your own profile (existing behavior).
#[component]
fn SelfSidemenu(username: String) -> Element {
    let tr: UserSidemenuTranslate = use_translate();

    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();
    let mut popup = use_popup();
    let nav = use_navigator();
    let user = user_ctx().user.clone().unwrap_or_default();
    let teams = team_ctx.teams.read().clone();

    rsx! {
        crate::common::SideMenuContainer {
            crate::common::SideMenuProfileCard {
                crate::common::TeamSelector {
                    selected_label: user.display_name.clone(),
                    user_display_name: user.display_name.clone(),
                    user_profile_url: user.profile_url.clone(),
                    user_href: format!("/{}", user.username),
                    teams: teams.clone(),
                    team_href_prefix: "".to_string(),
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

                ProfileImage { url: user.profile_url.clone(), name: user.display_name.clone() }

                div { class: "font-medium text-text-primary", "{user.display_name}" }

                if !user.description.is_empty() {
                    div { class: "text-xs text-text-primary", "{user.description}" }
                }

                FollowCounts {
                    followers: user.followers_count,
                    followings: user.followings_count,
                }
            }

            crate::common::SideMenuNav {
                crate::common::SideMenuLink {
                    to: format!("/{username}/posts"),
                    label: tr.my_posts,
                    icon: rsx! {
                        icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    },
                }

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

/// Sidemenu for viewing another user's profile. Shows target user's info
/// and a Follow/Unfollow button.
#[component]
fn OtherUserSidemenu(username: String) -> Element {
    let tr: UserSidemenuTranslate = use_translate();
    let username_clone = username.clone();

    let status_resource = use_server_future(move || {
        let uname = username_clone.clone();
        async move { check_follow_status_handler(uname).await }
    })?;

    let val = status_resource.read();
    let initial = val.as_ref().unwrap();

    let mut is_following = use_signal(move || {
        initial.as_ref().map(|s| s.is_following).unwrap_or(false)
    });
    let mut followers_count = use_signal(move || {
        initial
            .as_ref()
            .map(|s| s.target_followers_count)
            .unwrap_or(0)
    });
    let mut processing = use_signal(|| false);

    match initial.as_ref() {
        Ok(status) => {
            let target_pk = status.target_pk.clone();

            rsx! {
                crate::common::SideMenuContainer {
                    crate::common::SideMenuProfileCard {
                        ProfileImage {
                            url: status.target_profile_url.clone(),
                            name: status.target_display_name.clone(),
                        }

                        div { class: "font-medium text-text-primary",
                            "{status.target_display_name}"
                        }

                        div { class: "text-xs text-text-secondary", "@{username}" }

                        if !status.target_description.is_empty() {
                            div { class: "text-xs text-text-primary",
                                "{status.target_description}"
                            }
                        }

                        FollowCounts {
                            followers: *followers_count.read(),
                            followings: status.target_followings_count,
                        }

                        FollowToggleButton {
                            is_following: *is_following.read(),
                            processing: *processing.read(),
                            on_click: {
                                let target_pk = target_pk.clone();
                                move |_| {
                                    let currently_following = *is_following.read();
                                    let target_pk = target_pk.clone();
                                    processing.set(true);
                                    spawn(async move {
                                        let result = if currently_following {
                                            unfollow_user(target_pk).await
                                        } else {
                                            follow_user(target_pk).await
                                        };
                                        match result {
                                            Ok(_) => {
                                                is_following.set(!currently_following);
                                                let delta: i64 = if currently_following { -1 } else { 1 };
                                                let current = *followers_count.read();
                                                followers_count.set((current + delta).max(0));
                                            }
                                            Err(e) => {
                                                tracing::error!("Follow/unfollow failed: {:?}", e);
                                            }
                                        }
                                        processing.set(false);
                                    });
                                }
                            },
                        }
                    }

                    crate::common::SideMenuNav {
                        crate::common::SideMenuLink {
                            to: format!("/{username}/posts"),
                            label: tr.posts,
                            icon: rsx! {
                                icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            },
                        }

                        crate::common::SideMenuLink {
                            to: format!("/{username}/rewards"),
                            label: tr.rewards,
                            icon: rsx! {
                                icons::game::Trophy { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            },
                        }
                    }
                }
            }
        }
        Err(_) => {
            rsx! {
                crate::common::SideMenuContainer {
                    crate::common::SideMenuProfileCard {
                        div { class: "font-medium text-text-primary", "@{username}" }
                    }
                    crate::common::SideMenuNav {
                        crate::common::SideMenuLink {
                            to: format!("/{username}/posts"),
                            label: tr.posts,
                            icon: rsx! {
                                icons::edit::EditContent { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            },
                        }

                        crate::common::SideMenuLink {
                            to: format!("/{username}/rewards"),
                            label: tr.rewards,
                            icon: rsx! {
                                icons::game::Trophy { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProfileImage(url: String, name: String) -> Element {
    rsx! {
        div { class: "relative",
            if !url.is_empty() {
                img {
                    src: "{url}",
                    alt: "{name}",
                    class: "w-20 h-20 rounded-full border-2 object-cover object-top",
                }
            } else {
                div { class: "w-20 h-20 rounded-full border border-neutral-500 bg-neutral-500" }
            }
        }
    }
}

#[component]
fn FollowCounts(followers: i64, followings: i64) -> Element {
    let tr: UserSidemenuTranslate = use_translate();
    rsx! {
        div { class: "flex gap-4 text-sm",
            Link {
                class: "flex gap-1 hover:text-text-primary transition-colors",
                to: "/my-follower",
                span { class: "font-semibold text-text-primary", "{followers}" }
                span { class: "text-text-primary", "{tr.followers}" }
            }
            Link {
                class: "flex gap-1 hover:text-text-primary transition-colors",
                to: "/my-follower",
                span { class: "font-semibold text-text-primary", "{followings}" }
                span { class: "text-text-primary", "{tr.following}" }
            }
        }
    }
}

#[component]
fn FollowToggleButton(
    is_following: bool,
    processing: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let tr: UserSidemenuTranslate = use_translate();

    let (label, btn_class) = if is_following {
        (
            tr.unfollow,
            "w-full py-2 rounded-full text-sm font-semibold border border-neutral-600 text-text-primary hover:border-red-500 hover:text-red-500 transition-colors cursor-pointer",
        )
    } else {
        (
            tr.follow,
            "w-full py-2 rounded-full text-sm font-semibold bg-primary text-white hover:opacity-90 transition-opacity cursor-pointer",
        )
    };

    let disabled_class = if processing { " opacity-50 cursor-not-allowed" } else { "" };

    rsx! {
        button {
            class: "{btn_class}{disabled_class}",
            disabled: processing,
            onclick: move |e| on_click.call(e),
            {label}
        }
    }
}
