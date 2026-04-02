use crate::common::*;
use crate::features::my_follower::controllers::{
    check_follow_status_handler, follow_user, unfollow_user,
};
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::types::TeamGroupPermission;
use crate::features::posts::*;
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::home::components::*;
use crate::features::social::pages::home::HomeViewMode;
use crate::features::social::Route;

translate! {
    HomeTranslate;

    create: {
        en: "Create",
        ko: "작성",
    },
}

#[component]
pub fn Home(username: String) -> Element {
    let tr: HomeTranslate = use_translate();
    let mut view_mode: Signal<HomeViewMode> = use_signal(|| HomeViewMode::List);
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let nav = use_navigator();
    let logged_in = user_ctx().user.is_some();

    // Load team info (including permissions) from server for all users
    let team_detail = use_server_future(use_reactive((&username,), |(name,)| async move {
        find_team_handler(name).await
    }))?;
    let team_detail_data = team_detail.read();

    let (display_name, profile_url, description, thumbnail_url, is_creator, team_pk_str) =
        match team_detail_data.as_ref().and_then(|r| r.as_ref().ok()) {
            Some(t) => {
                let perms = t.permissions.as_ref().cloned().unwrap_or_default();
                let is_creator = perms.contains(&TeamGroupPermission::TeamAdmin)
                    || perms.contains(&TeamGroupPermission::TeamEdit);
                (
                    if t.nickname.is_empty() {
                        t.username.clone()
                    } else {
                        t.nickname.clone()
                    },
                    t.profile_url.clone().unwrap_or_default(),
                    t.html_contents.clone(),
                    t.thumbnail_url.clone().unwrap_or_default(),
                    is_creator,
                    Some(t.pk.clone()),
                )
            }
            None => (
                username.clone(),
                String::new(),
                String::new(),
                String::new(),
                false,
                None,
            ),
        };

    // Follow status — use_server_future so target_pk is available when button renders
    let username_for_status = username.clone();
    let follow_status = use_server_future(move || {
        let name = username_for_status.clone();
        async move {
            check_follow_status_handler(name).await.ok()
        }
    })?;

    let follow_status_val = follow_status.read();
    let initial_follow = follow_status_val
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .map(|s| s.is_following)
        .unwrap_or(false);
    let follow_target_pk = follow_status_val
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .map(|s| s.target_pk.clone());

    let mut is_following = use_signal(move || initial_follow);
    let mut processing = use_signal(|| false);

    let settings_route = Route::TeamSetting {
        username: username.clone(),
    }
    .to_string();

    let selected_category = use_context::<Signal<Option<String>>>();

    let list_btn_class = if view_mode() == HomeViewMode::List {
        "bg-[#1a1a1a]"
    } else {
        "bg-[#282828] hover:bg-[#222222]"
    };
    let card_btn_class = if view_mode() == HomeViewMode::Card {
        "bg-[#1a1a1a]"
    } else {
        "bg-[#282828] hover:bg-[#222222]"
    };

    rsx! {
        div { class: "flex flex-col w-full min-w-0 gap-6 overflow-x-hidden",
            TeamHeader {
                display_name,
                profile_url,
                description,
                thumbnail_url,
                is_creator,
                settings_route,
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

            // Team drafts — shown only to creators so they can find and edit their drafts
            if is_creator {
                SuspenseBoundary {
                    TeamDraftTimeline { username: username.clone() }
                }
            }

            // View mode toggle + Create button
            div { class: "flex items-center justify-between w-full",
            div { class: "flex overflow-hidden rounded-[10px] w-fit",
                button {
                    class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors {list_btn_class}",
                    onclick: move |_| view_mode.set(HomeViewMode::List),
                    icons::alignments::AlignJustify {
                        class: "w-6 h-6 [&>path]:stroke-icon-primary",
                    }
                }
                button {
                    class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors {card_btn_class}",
                    onclick: move |_| view_mode.set(HomeViewMode::Card),
                    lucide_dioxus::LayoutGrid {
                        class: "w-6 h-6 [&>rect]:stroke-icon-primary [&>path]:stroke-icon-primary",
                    }
                }
            }

            // Create button — only visible to admins/editors
            if is_creator {
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    class: "flex flex-row items-center gap-2",
                    onclick: move |_| {
                        let team_pk = team_pk_str.clone();
                        let nav = nav.clone();
                        async move {
                            let team_id = team_pk.map(|pk| pk.parse().unwrap_or_default());
                            match create_post_handler(team_id).await {
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
                    icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-btn-primary-text [&>path]:fill-none" }
                    "{tr.create}"
                }
            }
            } // end flex items-center justify-between

            // Posts
            TeamPostsPanel {
                key: "posts-{username}",
                username: username.clone(),
                view_mode: view_mode(),
                selected_category: selected_category(),
            }
        }
    }
}
