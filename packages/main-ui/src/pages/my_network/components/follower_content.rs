#![allow(unused)]
use bdk::prelude::{
    by_components::icons::{
        chat::RoundBubble2, emoji::ThumbsUp, other_devices::Bookmark, validations::Extra,
    },
    *,
};
use dto::SpaceSummary;

use crate::{
    components::icons::{Badge, Feed2, RewardCoin},
    pages::my_network::controller::FollowerPerson,
    utils::time::format_prev_time,
};

#[component]
pub fn FollowerContent(
    lang: Language,
    mut person: FollowerPerson,
    onclick: EventHandler<i64>,
) -> Element {
    let mut person_signal = use_signal(|| person.clone());
    rsx! {
        div {
            class: "w-full flex justify-between items-center p-15 bg-[#191919] border-b-[0.1px] border-gray-500/20",     
                div {
                    class: "flex items-center gap-x-8", 
                    div {
                        class: "w-40 h-40 rounded-full overflow-hidden",
                        img {
                            class: "w-full h-full object-cover",
                            src: person.image_url,
                        }
                    }

                    div {
                        class: "flex flex-col",
                        span {
                            class: "text-white font-medium",
                            "{person.name}"
                        }
                        p {
                            class: "text-[#a1a1a1] text-s",
                            "{person.about}"
                        }
                    }
                }
                if person.followed {
                    button {
                            class : "cursor-pointer rounded-[48px] flex items-center jusify-center px-8 py-8 bg-[#a1a1a1] text-black",


                            span {
                                class: "text-center",
                                "following"
                            }
                    }

                } else {
                    button {
                        class : "cursor-pointer rounded-[48px] flex items-center jusify-center px-8 py-8 bg-white text-black",

                        span {
                            class: "text-center",
                            "follow"
                        }
                    }
                }
        }
    }
}

