use bdk::prelude::{
    by_components::icons::{
        chat::RoundBubble2, emoji::ThumbsUp, other_devices::Bookmark, validations::Extra,
    },
    *,
};
use dto::FeedSummary;

use crate::{
    components::icons::{Badge, Feed2, RewardCoin},
    pages::components::Label,
    utils::time::format_prev_time,
};

#[component]
pub fn FeedContent(lang: Language, feed: FeedSummary, onclick: EventHandler<i64>) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-col w-full justify-start items-start px-20 pt-20 pb-10 bg-footer rounded-lg gap-10",
            onclick: move |_| {
                onclick.call(feed.id);
            },
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                TopContent {
                    label: feed.feed_type.translate(&lang),
                    title: feed.title.unwrap_or_default(),
                    image: feed.profile_image.unwrap_or_default(),
                    nickname: feed.proposer_name.unwrap_or_default(),
                    created_at: feed.created_at,
                }

                ContentDescription { id: feed.id, lang, html: feed.html_contents }

                BottomContent {
                    like: feed.likes,
                    comment: feed.comments,
                    reward: feed.rewards,
                    shared: feed.shares,
                }
            }
        }
    }
}

#[component]
pub fn BottomContent(like: i64, comment: i64, reward: i64, shared: i64) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-20 py-10",
            IconBox {
                icon: rsx! {
                    ThumbsUp { class: "[&>path]:stroke-neutral-500", width: "18", height: "18" }
                },
                text: like.to_string(),
            }
            IconBox {
                icon: rsx! {
                    RoundBubble2 {
                        class: "[&>path]:stroke-neutral-500 [&>line]:stroke-neutral-500",
                        width: "20",
                        height: "20",
                        fill: "none",
                    }
                },
                text: comment.to_string(),
            }
            IconBox {
                icon: rsx! {
                    RewardCoin { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                },
                text: format!("{}K", reward),
            }
            IconBox {
                icon: rsx! {
                    Feed2 { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                },
                text: shared.to_string(),
            }
        }
    }
}

#[component]
pub fn IconBox(icon: Element, text: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit justify-start items-center px-20 py-16 gap-10",
            {icon}
            div { class: "font-medium text-white text-[15px]/18", {text} }
        }
    }
}

#[component]
pub fn ContentDescription(id: i64, lang: Language, html: String) -> Element {
    let tr: ContentDescriptionTranslate = translate(&lang);
    let mut show_all = use_signal(|| false);
    let mut show_button = use_signal(|| false);

    let content_id = format!("content-description-html-{id}");

    use_effect({
        let content_id = content_id.clone();
        move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            if let Some(el) = document.get_element_by_id(&content_id) {
                let scroll_height = el.scroll_height();
                let client_height = el.client_height();
                if scroll_height > client_height {
                    show_button.set(true);
                }
            }
        }
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div {
                id: content_id,
                class: "font-normal text-c-wg-30 text-[15px]/24",
                dangerous_inner_html: if show_all() { html.clone() } else { format!("<div style=\"max-height: 50px; overflow: hidden;\">{}</div>", html) },
            }
            div {
                class: "cursor-pointer underline font-normal text-white text-[15px]/24 aria-hidden:!hidden",
                aria_hidden: !show_button(),
                onclick: move |_| show_all.set(!show_all()),
                if show_all() {
                    {tr.close}
                } else {
                    {tr.see_more}
                }
            }
        }
    }
}

#[component]
pub fn TopContent(
    label: String,
    title: String,
    image: String,
    nickname: String,
    created_at: i64,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            div { class: "flex flex-row w-full justify-between items-center",
                Label { label }
                div { class: "flex flex-row w-fit justify-start items-center gap-10",
                    Bookmark {
                        class: "[&>path]:stroke-neutral-500",
                        width: "20",
                        height: "20",
                    }

                    Extra {
                        class: "[&>circle]:fill-neutral-500",
                        width: "24",
                        height: "24",
                    }
                }
            }

            div { class: "font-bold text-white text-[20px]/25", {title} }

            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-row w-full justify-start items-center gap-10",
                    if image == "" {
                        div { class: "w-24 h-24 rounded-full bg-neutral-400" }
                    } else {
                        img {
                            class: "w-24 h-24 rounded-full object-cover",
                            src: image,
                        }
                    }
                    div { class: "flex flex-row w-fit justify-start items-center gap-4",
                        div { class: "font-medium text-white text-base/24", {nickname} }
                        Badge {}
                    }
                }

                div { class: "font-light text-sm/17 text-white whitespace-nowrap",
                    {format_prev_time(created_at)}
                }
            }
        }
    }
}

translate! {
    ContentDescriptionTranslate;

    close: {
        ko: "Close",
        en: "Close"
    },
    see_more: {
        ko: "See More",
        en: "See More"
    }
}
