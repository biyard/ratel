use crate::common::hooks::use_infinite_query;
use crate::common::*;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::list_user_posts::list_team_posts_handler;
use dioxus::prelude::*;
use super::super::HomeViewMode;

#[component]
pub fn TeamPostsPanel(username: String, view_mode: HomeViewMode, selected_category: Option<String>) -> Element {
    let mut username_signal = use_signal(|| username.clone());
    let mut category_signal = use_signal(|| selected_category.clone());
    let mut v = use_infinite_query(move |bookmark| {
        let username = username_signal();
        let category = category_signal();
        async move {
            match list_team_posts_handler(username, category, bookmark).await {
                Ok(result) => Ok(result),
                Err(_) => Ok(Default::default()),
            }
        }
    })?;

    let mut v_clone = v.clone();
    use_effect(use_reactive((&username, &selected_category), move |(name, cat)| {
        let changed = *username_signal.peek() != name || *category_signal.peek() != cat;
        if changed {
            username_signal.set(name);
            category_signal.set(cat);
            v_clone.restart();
        }
    }));

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
                        div { class: "grid grid-cols-2 gap-10",
                            for post in items {
                                TeamPostCard { key: "card-{post.pk}", post: post.clone() }
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
fn TeamPostCard(post: PostResponse) -> Element {
    let post_url = post.url();
    let title = post.title.clone();
    let created_at = post.created_at;
    let likes = post.likes;
    let comments = post.comments;
    let thumbnail = post.urls.first().cloned();
    let html_contents = post.html_contents.clone();
    let post_categories = post.categories.clone();

    rsx! {
        Link {
            to: "{post_url}",
            class: "block",
            div { class: "flex flex-col gap-4 py-6 border-b border-separator",
                // Header row: tag + actions
                div { class: "flex items-center justify-between",
                    // Category badges
                    if !post_categories.is_empty() {
                        div { class: "flex items-center gap-1 flex-wrap",
                            for cat in post_categories.iter() {
                                div {
                                    key: "{cat}",
                                    class: "flex items-center border border-border rounded-[8px] px-2 py-0.5",
                                    span { class: "text-[12px] font-bold text-text-primary leading-[14px] tracking-[-0.12px]",
                                        "{cat}"
                                    }
                                }
                            }
                        }
                    } else {
                        div {}
                    }
                    // Action icons
                    div { class: "flex items-center gap-2",
                        icons::edit::Edit1 {
                            class: "w-5 h-5 [&>path]:stroke-icon-primary cursor-pointer",
                        }
                    }
                }

                // Title
                h2 { class: "text-[20px] font-bold text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

                // Date + counts
                div { class: "flex items-center justify-between",
                    span { class: "text-[15px] text-foreground-muted leading-[22px]",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex items-center gap-2.5",
                        div { class: "flex items-center gap-1",
                            icons::emoji::ThumbsUp {
                                class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                            }
                            span { class: "text-[15px] font-medium text-text-primary", "{likes}" }
                        }
                        div { class: "flex items-center gap-1",
                            icons::chat::SquareChat {
                                class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                            }
                            span { class: "text-[15px] font-medium text-text-primary", "{comments}" }
                        }
                    }
                }

                // Thumbnail
                if let Some(thumb_url) = thumbnail {
                    img {
                        src: "{thumb_url}",
                        class: "w-full h-[280px] rounded-[24px] object-cover",
                    }
                } else {
                    div { class: "w-full h-[280px] rounded-[24px] bg-[#2a2a2a]" }
                }

                // Description (truncated)
                div {
                    class: "text-[15px] text-foreground leading-[22px]",
                    style: "overflow: hidden; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;",
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
    let title = post.title.clone();
    let created_at = post.created_at;
    let likes = post.likes;
    let comments = post.comments;
    let post_categories = post.categories.clone();

    rsx! {
        Link {
            to: "{post_url}",
            class: "block",
            div { class: "flex flex-col gap-3 py-5 border-b border-separator",
                // Category badges
                if !post_categories.is_empty() {
                    div { class: "flex items-center gap-1 flex-wrap",
                        for cat in post_categories.iter() {
                            div {
                                key: "{cat}",
                                class: "flex items-center border border-border rounded-[8px] px-2 py-0.5 w-fit",
                                span { class: "text-[12px] font-bold text-text-primary leading-[14px] tracking-[-0.12px]",
                                    "{cat}"
                                }
                            }
                        }
                    }
                }

                // Title
                h2 { class: "text-[20px] font-bold text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

                // Date + counts
                div { class: "flex items-center justify-between",
                    span { class: "text-[15px] text-foreground-muted leading-[22px]",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex items-center gap-2.5",
                        div { class: "flex items-center gap-1",
                            icons::emoji::ThumbsUp {
                                class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                            }
                            span { class: "text-[15px] font-medium text-text-primary", "{likes}" }
                        }
                        div { class: "flex items-center gap-1",
                            icons::chat::SquareChat {
                                class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent",
                            }
                            span { class: "text-[15px] font-medium text-text-primary", "{comments}" }
                        }
                    }
                }
            }
        }
    }
}

fn format_post_date(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    match Utc.timestamp_millis_opt(timestamp_ms).single() {
        Some(dt) => dt.format("%b %-d. %Y").to_string(),
        None => String::new(),
    }
}
