#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::Topic;

use crate::theme::Theme;

#[component]
pub fn UpcomingTopics(
    #[props(default ="upcoming_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    _topics: Vec<Topic>,
) -> Element {
    rsx! {
        div {
            id,
            class,
            div {
                class: "flex flex-col gap-[16px] items-start justify-start w-full",
                span {
                    class: "text-[18px] font-semibold",
                    "다가올 투표"
                }
                UpcomingTopic {}
            }
        }
    }
}

#[component]
pub fn UpcomingTopic() -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div {
            class: "w-full flex flex-col gap-[10px] items-start justify-start bg-[{theme_data.primary07}] px-[24px] py-[20px]",
            span {
                class: "text-[18px] font-semibold",
                "다가올 투표"
            }
        }
    }
}
