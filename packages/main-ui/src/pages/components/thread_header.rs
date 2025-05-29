use bdk::prelude::{
    by_components::icons::{
        arrows::ArrowLeft, chat::RoundBubble2, other_devices::Bookmark, validations::Extra,
    },
    *,
};
use dto::{FeedType, by_components::icons::validations::Add};

use crate::{
    components::icons::{Badge, Feed2, Palace, RewardCoin},
    pages::components::Label,
    utils::time::format_prev_time,
};

#[component]
pub fn ThreadHeader(
    lang: Language,
    is_creator: bool,
    profile: String,
    proposer: String,
    title: String,
    number_of_comments: i64,
    number_of_rewards: i64,
    number_of_shared: i64,
    created_at: i64,
    feed_type: FeedType,
    exist_spaces: bool,
    enter_space: EventHandler<MouseEvent>,
    create_space: EventHandler<MouseEvent>,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-between items-start gap-10 max-tablet:gap-25",
            ThreadHeaderIcon {
                exist_spaces,
                enter_space,
                number_of_comments,
                number_of_rewards,
                number_of_shared,
                onprev,
            }
            ThreadHeaderContents {
                lang,
                is_creator,
                exist_spaces,
                feed_type,
                title,
                profile,
                proposer,
                created_at,
                create_space,
            }
        }
    }
}

#[component]
pub fn ThreadHeaderContents(
    lang: Language,
    is_creator: bool,
    exist_spaces: bool,
    feed_type: FeedType,
    title: String,
    profile: String,
    proposer: String,
    created_at: i64,
    create_space: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            div { class: "flex flex-row w-full justify-between items-center",
                Label { label: "Crypto" }
                if is_creator && !exist_spaces {
                    CreateSpaceButton { lang, create_space }
                }
            }
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
            if !profile.is_empty() {
                img {
                    class: "w-20 h-20 object-cover rounded-full",
                    src: profile,
                    alt: "{proposer}",
                }
            } else {
                div { class: "w-20 h-20 rounded-full bg-neutral-400" }
            }
            div { class: "font-semibold text-sm/20 text-white", {proposer.clone()} }
            Badge {}
        }
    }
}

#[component]
pub fn ThreadHeaderIcon(
    exist_spaces: bool,

    number_of_comments: i64,
    number_of_rewards: i64,
    number_of_shared: i64,
    enter_space: EventHandler<MouseEvent>,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center",
            div {
                class: "cursor-pointer w-fit h-fit",
                onclick: move |e| {
                    onprev.call(e);
                },
                ArrowLeft {
                    class: "[&>path]:stroke-white",
                    width: "24",
                    height: "24",
                }
            }

            div { class: "flex flex-row w-fit justify-start items-center gap-20",
                if exist_spaces {
                    div {
                        class: "cursor-pointer flex flex-row w-fit justify-start items-center gap-5",
                        onclick: move |e| {
                            enter_space.call(e);
                        },
                        Palace {
                            class: "[&>path]:stroke-neutral-500",
                            width: "24",
                            height: "24",
                        }
                        div { class: "font-medium text-white text-[15px]/18", "Space" }
                    }
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
pub fn CreateSpaceButton(lang: Language, create_space: EventHandler<MouseEvent>) -> Element {
    let tr: CreateSpaceButtonTranslate = translate(&lang);
    rsx! {
        div {
            class: "cursor-pointer create-space-button",
            onclick: move |e| {
                create_space.call(e);
            },
            Add {
                class: "[&>stroke]:fill-neutral-500",
                width: "20",
                height: "20",
            }
            div { {tr.create_space} }
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

translate! {
    CreateSpaceButtonTranslate;

    create_space: {
        ko: "Create a Space",
        en: "Create a Space"
    }
}
