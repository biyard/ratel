use crate::common::hooks::use_infinite_query;
use crate::common::*;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::list_user_posts::list_user_posts_handler;
use dioxus::prelude::*;
use super::HomeViewMode;

#[component]
pub fn UserPostsPanel(username: String, view_mode: HomeViewMode) -> Element {
    let mut username_signal = use_signal(|| username.clone());
    let mut v = use_infinite_query(move |bookmark| {
        let username = username_signal();
        async move { list_user_posts_handler(username, bookmark).await }
    })?;

    let mut v_clone = v.clone();
    use_effect(use_reactive((&username,), move |(name,)| {
        if *username_signal.peek() != name {
            username_signal.set(name);
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
                                UserPostCard { key: "card-{post.pk}", post: post.clone() }
                            }
                        }
                    },
                    HomeViewMode::List => rsx! {
                        div { class: "flex flex-col",
                            for post in items {
                                UserPostListItem { key: "list-{post.pk}", post: post.clone() }
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

#[component]
fn UserPostCard(post: PostResponse) -> Element {
    let post_url = post.url();
    let title = post.title.clone();
    let created_at = post.created_at;
    let likes = post.likes;
    let comments = post.comments;
    let thumbnail = post.urls.first().cloned();
    let html_contents = post.html_contents.clone();
    let category = post.categories.first().cloned().unwrap_or_default();

    rsx! {
        Link {
            to: "{post_url}",
            class: "block",
            div { class: "flex flex-col gap-4 py-6 border-b border-separator",
                div { class: "flex items-center justify-between",
                    if !category.is_empty() {
                        div { class: "flex items-center border border-tag-stroke rounded-[8px] px-2 py-0.5",
                            span { class: "text-[12px] font-bold text-text-primary leading-[14px] tracking-[-0.12px]",
                                "{category}"
                            }
                        }
                    } else {
                        div {}
                    }
                    div { class: "flex items-center gap-2",
                        icons::edit::Edit1 {
                            class: "w-5 h-5 [&>path]:stroke-icon-primary cursor-pointer",
                        }
                    }
                }

                h2 { class: "text-[20px] font-bold text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

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

                if let Some(thumb_url) = thumbnail {
                    img {
                        src: "{thumb_url}",
                        class: "w-full h-[280px] rounded-[24px] object-cover",
                    }
                } else {
                    div { class: "w-full h-[280px] rounded-[24px] bg-thumbnail-placeholder-bg" }
                }

                div {
                    class: "text-[15px] text-foreground leading-[22px]",
                    style: "overflow: hidden; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;",
                    dangerous_inner_html: html_contents,
                }
            }
        }
    }
}

#[component]
fn UserPostListItem(post: PostResponse) -> Element {
    let post_url = post.url();
    let title = post.title.clone();
    let created_at = post.created_at;
    let likes = post.likes;
    let comments = post.comments;
    let category = post.categories.first().cloned().unwrap_or_default();

    rsx! {
        Link {
            to: "{post_url}",
            class: "block",
            div { class: "flex flex-col gap-3 py-5 border-b border-separator",
                if !category.is_empty() {
                    div { class: "flex items-center border border-tag-stroke rounded-[8px] px-2 py-0.5 w-fit",
                        span { class: "text-[12px] font-bold text-text-primary leading-[14px] tracking-[-0.12px]",
                            "{category}"
                        }
                    }
                }

                h2 { class: "text-[20px] font-bold text-text-primary tracking-[0.5px] leading-[25px] line-clamp-2",
                    "{title}"
                }

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
