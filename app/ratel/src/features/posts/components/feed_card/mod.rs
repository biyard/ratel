use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::posts::types::*;
use crate::features::posts::*;
use dioxus::prelude::*;

fn convert_number_to_string(n: i64) -> String {
    let suffixes = ["K", "M", "B"];
    let mut value = n as f64;
    let mut i = 0;

    while value >= 1000.0 && i < suffixes.len() {
        value /= 1000.0;
        i += 1;
    }

    if i == 0 {
        format!("{}", n)
    } else {
        format!("{} {}", value as i64, suffixes[i - 1])
    }
}

pub fn time_ago(timestamp_millis: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_millis;

    if diff < 60 * 1000 {
        format!("{}s ago", diff / 1000)
    } else if diff < 3600 * 1000 {
        format!("{}m ago", diff / 1000 / 60)
    } else if diff < 86400 * 1000 {
        format!("{}h ago", diff / 1000 / 3600)
    } else if diff < 604800 * 1000 {
        format!("{}d ago", diff / 1000 / 86400)
    } else if diff < 31536000 * 1000 {
        format!("{}w ago", diff / 1000 / 604800)
    } else {
        format!("{}y ago", diff / 1000 / 31536000)
    }
}

#[component]
pub fn FeedCard(
    post: PostResponse,
    on_like: Option<EventHandler<bool>>,
    on_edit: Option<EventHandler<MouseEvent>>,
    href: Option<String>,
) -> Element {
    let mut optimistic_liked = use_signal(|| post.liked);
    let mut optimistic_likes = use_signal(|| post.likes);
    let mut is_processing = use_signal(|| false);
    let nav = use_navigator();

    let post_clone = post.clone();
    let post_pk = post.pk.clone();
    let on_like_callback = on_like.clone();

    use_effect(move || {
        optimistic_liked.set(post_clone.liked);
        optimistic_likes.set(post_clone.likes);
    });

    let link_href = href.unwrap_or_default();

    rsx! {
        div { class: "relative flex flex-col border rounded-[10px] bg-card-bg-secondary border-card-enable-border",
            Link { class: "block", to: post.url(),
                FeedBody { post: post.clone(), on_edit }
            }
            FeedFooter {
                href: link_href.clone(),
                booster_type: post.booster,
                likes: *optimistic_likes.read(),
                comments: post.comments,
                rewards: post.rewards.unwrap_or(0),
                shares: post.shares,
                is_liked: *optimistic_liked.read(),
                is_processing: *is_processing.read(),
                on_comment_click: {
                    let nav = nav.clone();
                    move || {
                        nav.push(post.url());
                    }
                },
                on_like_click: move |value: bool| {
                    if *is_processing.read() {
                        return;
                    }
                    let previous_liked = *optimistic_liked.read();
                    let previous_likes = *optimistic_likes.read();
                    let delta: i64 = if value { 1 } else { -1 };

                    optimistic_liked.set(value);
                    optimistic_likes.set((previous_likes + delta).max(0));
                    is_processing.set(true);

                    let post_pk = post_pk.clone();
                    let mut optimistic_liked = optimistic_liked.clone();
                    let mut optimistic_likes = optimistic_likes.clone();
                    let mut is_processing = is_processing.clone();
                    let on_like_callback = on_like_callback.clone();
                    spawn(async move {
                        let result = like_post_handler(post_pk, value).await;
                        if result.is_err() {
                            optimistic_liked.set(previous_liked);
                            optimistic_likes.set(previous_likes);
                        } else if let Some(handler) = on_like_callback {
                            handler.call(value);
                        }
                        is_processing.set(false);
                    });
                },
            }
        }
    }
}

#[component]
fn FeedBody(post: PostResponse, on_edit: Option<EventHandler<MouseEvent>>) -> Element {
    let PostResponse {
        title,
        html_contents,
        author_display_name,
        author_profile_url,
        author_type,
        created_at,
        space_pk,
        urls,
        ..
    } = post;

    rsx! {
        div { class: "flex flex-col gap-2.5 pt-5 pb-2.5",
            div { class: "flex flex-row justify-between px-5",
                div { class: "flex flex-row gap-2.5 justify-start items-center",
                    if space_pk.is_some() {
                        SpaceTag {}
                    }
                }
                if on_edit.is_some() {
                    EditButton {
                        onclick: move |e: MouseEvent| {
                            if let Some(handler) = &on_edit {
                                handler.call(e);
                            }
                        },
                    }
                }
            }
            h2 { class: "px-5 w-full font-bold align-middle line-clamp-2 text-xl/[25px] tracking-[0.5px] text-text-primary",
                {title}
            }
            div { class: "flex flex-row justify-between items-center px-5",
                UserBadge {
                    profile_url: author_profile_url,
                    name: author_display_name,
                    author_type,
                }
                p { class: "text-sm font-light align-middle text-text-primary",
                    {time_ago(created_at)}
                }
            }
            div { class: "flex flex-row justify-between px-5" }
            FeedContents { contents: html_contents, urls }
        }
    }
}

#[component]
pub fn FeedContents(contents: String, urls: Vec<String>) -> Element {
    rsx! {
        div { class: "break-all text-desc-text px-5 mt-2.5",
            div {
                class: "border-none",
                style: "min-height: 50px; max-height: 200px; overflow: hidden;",
                dangerous_inner_html: contents,
            }
        }
    }
}

#[component]
fn FeedFooter(
    href: String,
    booster_type: BoosterType,
    likes: i64,
    comments: i64,
    rewards: i64,
    shares: i64,
    is_liked: bool,
    is_processing: bool,
    on_comment_click: EventHandler<()>,
    on_like_click: EventHandler<bool>,
) -> Element {
    let like_class = if is_processing {
        "cursor-not-allowed opacity-50"
    } else {
        "cursor-pointer"
    };

    let liked = is_liked;

    rsx! {
        div { class: "flex flex-row justify-between items-center px-5 w-full border-t border-divider",
            div { class: "flex flex-row justify-between items-center w-full",
                IconText {
                    class: like_class,
                    onclick: move |_e: MouseEvent| {
                        if !is_processing {
                            on_like_click.call(!liked);
                        }
                    },
                    if is_liked {
                        icons::emoji::ThumbsUp { class: "[&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "[&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    }
                    {convert_number_to_string(likes)}
                }
                IconText {
                    class: "cursor-pointer",
                    onclick: move |_e: MouseEvent| {
                        on_comment_click.call(());
                    },
                    icons::chat::SquareChat { class: "[&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    {convert_number_to_string(comments)}
                }
                if booster_type != BoosterType::NoBoost {
                    IconText {
                        icons::money_payment::RewardCoin { class: "[&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        {convert_number_to_string(rewards)}
                    }
                }
                IconText {
                    icons::links_share::Share1 { class: "[&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    {convert_number_to_string(shares)}
                }
            }
        }
    }
}

#[component]
pub fn UserBadge(profile_url: String, name: String, author_type: crate::features::auth::UserType) -> Element {
    let img_class = if author_type == crate::features::auth::UserType::Team {
        "w-6 h-6 rounded-sm object-cover"
    } else {
        "w-6 h-6 rounded-full object-cover"
    };

    rsx! {
        div { class: "flex flex-row items-center w-fit med-16 text-text-primary gap-2.5",
            if !profile_url.is_empty() {
                img { src: profile_url, alt: "User Profile", class: img_class }
            }
            span { {name} }
        }
    }
}

#[component]
fn SpaceTag() -> Element {
    rsx! {
        span { class: "flex flex-row gap-1 justify-start items-center px-2 rounded-sm border border-label-color-border bg-label-color-bg",
            icons::home::Palace { class: "w-3.5 h-3.5 [&>path]:stroke-label-color-text [&_g>path:nth-child(n+2)]:stroke-label-color-text" }
            div { class: "font-semibold text-xs/[25px] text-label-color-text", "SPACE" }
        }
    }
}

#[component]
fn EditButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "p-1.5 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800",
            onclick: move |e: MouseEvent| {
                e.stop_propagation();
                e.prevent_default();
                onclick.call(e);
            },
            icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
        }
    }
}

#[component]
fn IconText(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "inline-flex flex-row gap-1.5 items-center py-3 px-3 leading-none whitespace-nowrap text-text-primary text-[15px] {class}",
            onclick: move |e: MouseEvent| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
