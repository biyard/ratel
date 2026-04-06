use super::super::HomeViewMode;
use crate::common::hooks::use_infinite_query;
use crate::common::*;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::posts::controllers::list_user_posts::list_team_posts_handler;
use dioxus::prelude::*;

#[component]
pub fn TeamPostsPanel(
    username: String,
    view_mode: HomeViewMode,
    selected_category: Option<String>,
) -> Element {
    let mut username_signal = use_signal(|| username.clone());
    let mut category_signal = use_signal(|| selected_category.clone());
    let mut v = use_infinite_query(move |bookmark| {
        let username = username_signal();
        let category = category_signal();
        async move { list_team_posts_handler(username, category, bookmark).await }
    })?;

    let mut v_clone = v.clone();
    use_effect(use_reactive(
        (&username, &selected_category),
        move |(name, cat)| {
            let changed = *username_signal.peek() != name || *category_signal.peek() != cat;
            if changed {
                username_signal.set(name);
                category_signal.set(cat);
                v_clone.restart();
            }
        },
    ));

    let items = v.items();

    if items.is_empty() {
        return rsx! {
            div { class: "flex justify-center items-center w-full py-20 text-foreground-muted text-base",
                "No posts yet"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col gap-0",
            {
                match view_mode {
                    HomeViewMode::Card => rsx! {
                        div { class: "grid grid-cols-2 max-mobile:grid-cols-1 items-stretch gap-5 max-tablet:gap-4",
                            for (idx, post) in items.iter().cloned().enumerate() {
                                TeamPostCard {
                                    key: "card-{post.pk}",
                                    post,
                                    full_width: items.len() % 2 == 1 && idx == items.len() - 1,
                                }
                            }
                        }
                    },
                    HomeViewMode::List => rsx! {
                        div { class: "flex flex-col",
                            for post in items {
                                TeamPostListItem { key: "list-{post.pk}", post: post.clone() }
                            }
                        }
                    },
                }
            }

            if v.has_more() {
                {v.more_element()}
            }
        }
    }
}

// --- Card view item ---

#[component]
fn TeamPostCard(post: PostResponse, #[props(default = false)] full_width: bool) -> Element {
    let post_url = post.url();
    let post_pk = post.pk.clone();
    let nav = use_navigator();
    let title = post.title.clone();
    let created_at = post.created_at;
    let comments = post.comments;
    let thumbnail = post.urls.first().cloned();
    let html_contents = post.html_contents.clone();
    let post_categories = post.categories.clone();
    let mut optimistic_liked = use_signal(|| post.liked);
    let mut optimistic_likes = use_signal(|| post.likes);
    let mut is_like_processing = use_signal(|| false);

    rsx! {
        div {
            class: if full_width {
                "col-span-2 max-mobile:col-span-1 block h-full cursor-pointer"
            } else {
                "block h-full cursor-pointer"
            },
            onclick: move |_| {
                nav.push(post_url.clone());
            },
            div { class: "flex h-full flex-col gap-4 border-b border-post-divider py-3",
                if !post_categories.is_empty() {
                    div { class: "flex min-w-0 items-center gap-2 flex-wrap",
                        for cat in post_categories.iter() {
                            div {
                                key: "{cat}",
                                class: "flex h-[25px] items-center justify-center rounded-[8px] border border-tag-stroke px-2 py-[3px]",
                                span { class: "font-bold font-raleway text-[12px]/[14px] tracking-[-0.12px] text-text-primary",
                                    "{cat}"
                                }
                            }
                        }
                    }
                }

                h2 { class: "font-bold font-raleway text-[20px]/[25px] tracking-[0.5px] text-text-primary line-clamp-1",
                    "{title}"
                }

                div { class: "flex items-center justify-between",
                    span { class: "font-inter text-[15px]/[22px] text-foreground-muted",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex items-center gap-[10px]",
                        LikeStatButton {
                            post_pk: post_pk.clone(),
                            liked: optimistic_liked,
                            likes: optimistic_likes,
                            is_processing: is_like_processing,
                            list_variant: false,
                        }
                        div { class: "flex items-center gap-[5px]",
                            icons::chat::SquareChat { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-inter text-[15px]/[18px] font-medium text-text-primary",
                                "{comments}"
                            }
                        }
                    }
                }

                if let Some(thumb_url) = thumbnail {
                    img {
                        src: "{thumb_url}",
                        class: "h-[280px] w-full shrink-0 rounded-[24px] object-cover",
                    }
                }

                div {
                    class: "min-h-0 flex-1 overflow-hidden font-raleway text-[15px]/[22px] text-foreground",
                    style: "display: -webkit-box; -webkit-line-clamp: 12; -webkit-box-orient: vertical;",
                    dangerous_inner_html: html_contents,
                }
            }
        }
    }
}

// --- List view item ---

#[component]
fn TeamPostListItem(post: PostResponse) -> Element {
    let post_url = post.url();
    let post_pk = post.pk.clone();
    let nav = use_navigator();
    let title = post.title.clone();
    let created_at = post.created_at;
    let comments = post.comments;
    let post_categories = post.categories.clone();
    let mut optimistic_liked = use_signal(|| post.liked);
    let mut optimistic_likes = use_signal(|| post.likes);
    let mut is_like_processing = use_signal(|| false);

    rsx! {
        div {
            class: "block w-full cursor-pointer",
            onclick: move |_| {
                nav.push(post_url.clone());
            },
            div { class: "flex w-full flex-col border-b border-post-divider py-6",
                div { class: "flex items-start justify-between gap-6",
                    if !post_categories.is_empty() {
                        div { class: "flex min-w-0 items-center gap-2 flex-wrap",
                            for cat in post_categories.iter() {
                                div {
                                    key: "{cat}",
                                    class: "flex h-[25px] items-center justify-center rounded-[8px] border border-tag-stroke px-2 py-[3px]",
                                    span { class: "font-bold font-raleway text-[12px]/[14px] tracking-[-0.12px] text-text-primary",
                                        "{cat}"
                                    }
                                }
                            }
                        }
                    } else {
                        div {}
                    }
                }

                h2 { class: "font-bold font-raleway text-[26px]/[30px] text-text-primary line-clamp-2 mt-3 mb-2.5",
                    "{title}"
                }

                div { class: "flex items-center justify-between gap-4 max-mobile:flex-col max-mobile:items-start",
                    span { class: "font-inter text-[15px]/[22px] text-foreground-muted",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex shrink-0 items-center gap-6",
                        LikeStatButton {
                            post_pk: post_pk.clone(),
                            liked: optimistic_liked,
                            likes: optimistic_likes,
                            is_processing: is_like_processing,
                            list_variant: true,
                        }
                        div { class: "flex items-center gap-1.25",
                            icons::chat::SquareChat { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-inter text-[15px]/[18px] font-medium text-text-primary",
                                "{comments}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LikeStatButton(
    post_pk: FeedPartition,
    liked: Signal<bool>,
    likes: Signal<i64>,
    is_processing: Signal<bool>,
    list_variant: bool,
) -> Element {
    let button_class = if list_variant {
        "flex items-center gap-1.25 disabled:opacity-50"
    } else {
        "flex items-center gap-1 disabled:opacity-50"
    };
    let text_class = if list_variant {
        "font-inter text-[15px]/[18px] font-medium text-text-primary"
    } else {
        "text-[15px] font-medium text-text-primary"
    };

    rsx! {
        button {
            class: "{button_class}",
            disabled: is_processing(),
            onclick: move |e| {
                e.stop_propagation();
                e.prevent_default();

                if is_processing() {
                    return;
                }

                let next_liked = !liked();
                let previous_liked = liked();
                let previous_likes = likes();
                let delta: i64 = if next_liked { 1 } else { -1 };

                liked.set(next_liked);
                likes.set((previous_likes + delta).max(0));
                is_processing.set(true);

                let post_pk = post_pk.clone();
                spawn(async move {
                    if like_post_handler(post_pk, next_liked).await.is_err() {
                        liked.set(previous_liked);
                        likes.set(previous_likes);
                    }
                    is_processing.set(false);
                });
            },
            if liked() {
                if list_variant {
                    icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                } else {
                    icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                }
            } else {
                if list_variant {
                    icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                } else {
                    icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                }
            }
            span { class: "{text_class}", "{likes()}" }
        }
    }
}

fn format_post_date(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    match Utc.timestamp_millis_opt(timestamp_ms).single() {
        Some(dt) => dt.format("%b %-d, %Y").to_string(),
        None => String::new(),
    }
}
