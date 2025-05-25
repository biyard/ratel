use bdk::prelude::{
    by_components::icons::{
        chat::RoundBubble2, emoji::ThumbsUp, other_devices::Bookmark, validations::Extra,
    },
    *,
};
use dto::SpaceSummary;

use crate::{
    components::icons::{Badge, Feed2, RewardCoin},
    pages::components::Label,
    utils::time::format_prev_time,
};

#[component]
pub fn FeedContent(lang: Language, space: SpaceSummary, onclick: EventHandler<i64>) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-col w-full justify-start items-start px-20 pt-20 pb-10 bg-footer rounded-lg gap-10",
            onclick: move |_| {
                onclick.call(space.id);
            },
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                TopContent {
                    label: space.content_type.translate(&lang),
                    title: space.title.unwrap_or_default(),
                    image: space.proposer_profile.unwrap_or_default(),
                    nickname: space.proposer_nickname.unwrap_or_default(),
                    created_at: space.created_at,
                }

                ContentDescription { lang, html: space.html_contents }

                BottomContent {
                    like: space.likes,
                    comment: space.comments,
                    reward: space.rewards,
                    shared: space.shares,
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
pub fn ContentDescription(lang: Language, html: String) -> Element {
    let tr: ContentDescriptionTranslate = translate(&lang);
    let mut show_all = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div {
                class: "font-normal text-c-wg-30 text-[15px]/24",
                dangerous_inner_html: if show_all() { html.clone() } else { format!("<div style=\"max-height: 50px; overflow: hidden;\">{}</div>", html) },
            }
            div {
                class: "cursor-pointer underline font-normal text-white text-[15px]/24",
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
                    img {
                        class: "w-24 h-24 rounded-full object-cover",
                        src: image,
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
