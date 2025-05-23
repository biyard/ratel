use bdk::prelude::{
    by_components::icons::{
        arrows::ArrowLeft, chat::RoundBubble2, other_devices::Bookmark, validations::Extra,
    },
    *,
};

use crate::{
    components::icons::{Badge, Feed2, RewardCoin},
    dto::content_type::ContentType,
    pages::components::Label,
    utils::time::format_prev_time,
};

#[component]
pub fn ThreadHeader(
    lang: Language,
    profile: String,
    proposer: String,
    title: String,
    number_of_comments: i64,
    number_of_rewards: i64,
    number_of_shared: i64,
    created_at: i64,
    content_type: ContentType,
    onback: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-between items-start gap-10 max-tablet:gap-25",
            ThreadHeaderIcon {
                number_of_comments,
                number_of_rewards,
                number_of_shared,
                onback,
            }
            ThreadHeaderContents {
                lang,
                content_type,
                title,
                profile,
                proposer,
                created_at,
            }
        }
    }
}

#[component]
pub fn ThreadHeaderContents(
    lang: Language,
    content_type: ContentType,
    title: String,
    profile: String,
    proposer: String,
    created_at: i64,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            Label { label: content_type.translate(&lang) }
            div { class: "flex flex-row w-full justify-between items-center gap-20",
                div { class: "font-bold text-[20px]/30 text-white", {title} }
                div { class: "w-20 h-20",
                    Bookmark {
                        class: "[&>path]:stroke-neutral-500",
                        width: "20",
                        height: "20",
                    }
                }
            }
            div { class: "flex flex-row w-full justify-between items-center gap-10",
                Profile { profile, proposer }
                div { class: "font-light text-sm/17 text-white", {format_prev_time(created_at)} }
            }
        }
    }
}

#[component]
pub fn Profile(profile: String, proposer: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit justify-start items-center gap-8",
            img {
                class: "w-20 h-20 object-cover rounded-full",
                src: profile,
                alt: "{proposer}",
            }
            div { class: "font-semibold text-sm/20 text-white", {proposer.clone()} }
            Badge {}
        }
    }
}

#[component]
pub fn ThreadHeaderIcon(
    number_of_comments: i64,
    number_of_rewards: i64,
    number_of_shared: i64,
    onback: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center",
            div {
                class: "cursor-pointer w-fit h-fit",
                onclick: move |e| {
                    onback.call(e);
                },
                ArrowLeft {
                    class: "[&>path]:stroke-white",
                    width: "24",
                    height: "24",
                }
            }

            div { class: "flex flex-row w-fit justify-start items-center gap-20",
                IconBox {
                    icon: rsx! {
                        RoundBubble2 {
                            class: "[&>path]:stroke-neutral-500 [&>line]:stroke-neutral-500",
                            width: "20",
                            height: "20",
                            fill: "none",
                        }
                    },
                    text: number_of_comments.to_string(),
                }
                IconBox {
                    icon: rsx! {
                        RewardCoin { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                    },
                    text: format!("{}K", number_of_rewards),
                }
                IconBox {
                    icon: rsx! {
                        Feed2 { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                    },
                    text: number_of_shared.to_string(),
                }
                Extra {
                    class: "[&>circle]:fill-neutral-500",
                    width: "24",
                    height: "24",
                }
            }
        }
    }
}

#[component]
pub fn IconBox(icon: Element, text: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit justify-start items-center gap-4",
            {icon}
            div { class: "font-medium text-[15px]/18 text-white", {text} }
        }
    }
}
