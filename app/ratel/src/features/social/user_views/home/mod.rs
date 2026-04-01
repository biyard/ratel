mod user_posts_panel;
use user_posts_panel::UserPostsPanel;

use crate::common::*;
use crate::features::social::pages::home::components::TeamHeader;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::*;
use crate::features::my_follower::controllers::{check_follow_status_handler, follow_user, unfollow_user};
use crate::features::social::controllers::{find_user_handler, FindUserQueryType};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum HomeViewMode {
    #[default]
    List,
    Card,
}

translate! {
    UserHomeTranslate;

    create: {
        en: "Create",
        ko: "작성",
    },
}

#[component]
pub fn Home(username: String) -> Element {
    let tr: UserHomeTranslate = use_translate();
    let mut view_mode: Signal<HomeViewMode> = use_signal(|| HomeViewMode::List);
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let nav = use_navigator();
    let logged_in = user_ctx().user.is_some();

    let is_owner = user_ctx()
        .user
        .as_ref()
        .map(|u| u.username == username)
        .unwrap_or(false);

    // Load user profile
    let user_detail = use_resource(use_reactive((&username,), |(name,)| async move {
        find_user_handler(FindUserQueryType::Username, name).await.ok()
    }));

    let (display_name, profile_url, description) = {
        let detail = user_detail.read();
        match detail.as_ref().and_then(|opt| opt.as_ref()) {
            Some(u) => (
                if u.nickname.is_empty() { u.username.clone() } else { u.nickname.clone() },
                u.profile_url.clone(),
                u.description.clone(),
            ),
            None => (username.clone(), String::new(), String::new()),
        }
    };

    // Follow status
    let username_for_status = username.clone();
    let follow_status = use_server_future(move || {
        let name = username_for_status.clone();
        async move { check_follow_status_handler(name).await }
    })?;

    let follow_status_val = follow_status.read();
    let initial_status = follow_status_val.as_ref().unwrap();

    let mut is_following = use_signal(move || {
        initial_status.as_ref().map(|s| s.is_following).unwrap_or(false)
    });
    let mut processing = use_signal(|| false);

    let follow_target_pk = initial_status.as_ref().ok().map(|s| s.target_pk.clone());


    rsx! {
        div { class: "flex flex-col w-full gap-6",
            TeamHeader {
                display_name,
                profile_url,
                description,
                thumbnail_url: String::new(),
                is_creator: false,
                settings_route: String::new(),
                is_following: is_following(),
                processing: processing(),
                logged_in,
                on_follow: {
                    let pk = follow_target_pk.clone();
                    move |_| {
                        let pk = pk.clone();
                        processing.set(true);
                        spawn(async move {
                            if let Some(pk) = pk {
                                match follow_user(pk).await {
                                    Ok(_) => is_following.set(true),
                                    Err(e) => tracing::error!("Follow failed: {:?}", e),
                                }
                            }
                            processing.set(false);
                        });
                    }
                },
                on_unfollow: {
                    let pk = follow_target_pk.clone();
                    move |_| {
                        let pk = pk.clone();
                        processing.set(true);
                        spawn(async move {
                            if let Some(pk) = pk {
                                match unfollow_user(pk).await {
                                    Ok(_) => is_following.set(false),
                                    Err(e) => tracing::error!("Unfollow failed: {:?}", e),
                                }
                            }
                            processing.set(false);
                        });
                    }
                },
            }

            // View mode toggle + Create button
            div { class: "flex items-center justify-between w-full",
                div { class: "flex overflow-hidden rounded-[10px] w-fit",
                    button {
                        class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors bg-button-muted hover:bg-button-muted-hover aria-selected:bg-button-active",
                        "aria-selected": view_mode() == HomeViewMode::List,
                        onclick: move |_| view_mode.set(HomeViewMode::List),
                        icons::alignments::AlignJustify {
                            class: "w-6 h-6 [&>path]:stroke-icon-primary",
                        }
                    }
                    button {
                        class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors bg-button-muted hover:bg-button-muted-hover aria-selected:bg-button-active",
                        "aria-selected": view_mode() == HomeViewMode::Card,
                        onclick: move |_| view_mode.set(HomeViewMode::Card),
                        lucide_dioxus::LayoutGrid {
                            class: "w-6 h-6 [&>rect]:stroke-icon-primary [&>path]:stroke-icon-primary",
                        }
                    }
                }

                if is_owner {
                    button {
                        class: "flex items-center gap-2.5 bg-btn-secondary-bg hover:bg-btn-secondary-hover-bg text-btn-secondary-text px-5 py-3 h-[44px] rounded-full text-sm font-medium transition-colors cursor-pointer",
                        onclick: move |_| {
                            let nav = nav.clone();
                            async move {
                                match create_post_handler(None).await {
                                    Ok(resp) => {
                                        let post_pk: FeedPartition = resp.post_pk.into();
                                        nav.push(format!("/posts/{post_pk}/edit"));
                                    }
                                    Err(e) => {
                                        debug!("Failed to create post: {:?}", e);
                                    }
                                }
                            }
                        },
                        icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-neutral-900 light:[&>path]:stroke-white" }
                        span { "{tr.create}" }
                    }
                }
            }

            // Posts
            UserPostsPanel {
                key: "posts-{username}",
                username: username.clone(),
                view_mode: view_mode(),
            }
        }
    }
}
