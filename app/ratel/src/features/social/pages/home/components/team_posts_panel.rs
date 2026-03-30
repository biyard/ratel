use super::super::HomeViewMode;
use crate::common::hooks::use_infinite_query;
use crate::common::*;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::list_user_posts::list_team_posts_handler;
use dioxus::prelude::*;

#[component]
pub fn TeamPostsPanel(username: ReadSignal<String>, view_mode: HomeViewMode) -> Element {
    let mut v = use_infinite_query(move |bookmark| {
        let username = username();
        // FIXME: It should reflect selected category.
        async move { list_team_posts_handler(username, None, bookmark).await }
    })?;

    let items = v.items();

    if items.is_empty() {
        return rsx! {
            div { class: "flex justify-center items-center py-20 w-full text-base text-foreground-muted",
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
        Link { to: "{post_url}", class: "block",
            div { class: "flex flex-col gap-4 py-6 border-b border-separator",
                // Header row: tag + actions
                div { class: "flex justify-between items-center",
                    // Category badges
                    if !post_categories.is_empty() {
                        div { class: "flex flex-wrap gap-1 items-center",
                            for cat in post_categories.iter() {
                                div {
                                    key: "{cat}",
                                    class: "flex items-center py-0.5 px-2 border border-border rounded-[8px]",
                                    span { class: "font-bold text-[12px] text-text-primary leading-[14px] tracking-[-0.12px]",
                                        "{cat}"
                                    }
                                }
                            }
                        }
                    } else {
                        div {}
                    }
                    // Action icons
                    div { class: "flex gap-2 items-center",
                        icons::edit::Edit1 { class: "w-5 h-5 cursor-pointer [&>path]:stroke-icon-primary" }
                    }
                }

                // Title
                h2 { class: "font-bold text-[20px] text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

                // Date + counts
                div { class: "flex justify-between items-center",
                    span { class: "text-[15px] text-foreground-muted leading-[22px]",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex gap-2.5 items-center",
                        div { class: "flex gap-1 items-center",
                            icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{likes}"
                            }
                        }
                        div { class: "flex gap-1 items-center",
                            icons::chat::SquareChat { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{comments}"
                            }
                        }
                    }
                }

                // Thumbnail
                if let Some(thumb_url) = thumbnail {
                    img {
                        src: "{thumb_url}",
                        class: "object-cover w-full h-[280px] rounded-[24px]",
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
        Link { to: "{post_url}", class: "block",
            div { class: "flex flex-col gap-3 py-5 border-b border-separator",
                // Category badges
                if !post_categories.is_empty() {
                    div { class: "flex flex-wrap gap-1 items-center",
                        for cat in post_categories.iter() {
                            div {
                                key: "{cat}",
                                class: "flex items-center py-0.5 px-2 border border-border rounded-[8px] w-fit",
                                span { class: "font-bold text-[12px] text-text-primary leading-[14px] tracking-[-0.12px]",
                                    "{cat}"
                                }
                            }
                        }
                    }
                }

                // Title
                h2 { class: "font-bold text-[20px] text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

                // Date + counts
                div { class: "flex justify-between items-center",
                    span { class: "text-[15px] text-foreground-muted leading-[22px]",
                        {format_post_date(created_at)}
                    }
                    div { class: "flex gap-2.5 items-center",
                        div { class: "flex gap-1 items-center",
                            icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{likes}"
                            }
                        }
                        div { class: "flex gap-1 items-center",
                            icons::chat::SquareChat { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{comments}"
                            }
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
