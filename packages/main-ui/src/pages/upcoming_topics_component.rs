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
                UpcomingTopic {}
            }
        }
    }
}

#[component]
pub fn UpcomingTopic(
    #[props(default ="".to_string())] class: String,
    #[props(default = 19)] day: u8,
    #[props(default = 12)] month: u8,
    #[props(default = "https://dev.democrasee.me/images/sample.png".to_string())] image: String,
    #[props(default = "조국, 입시비리/감찰무마 징역 몇년으로 선고될까?".to_string())] title: String,
    #[props(default = "1월 19일".to_string())] date: String,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();
    let month = month_name(month);

    rsx! {
        div {
            class: "w-full flex flex-row gap-[19px] items-center justify-start px-[24px] py-[20px] rounded-[8px] bg-[{theme_data.primary07}]",
            div {
                class: " flex flex-col gap-[4px] items-center justify-start",
                span {
                    class: "text-[24px] leading-[25px] font-extrabold text-center",
                    "{day}"
                }
                span {
                    class: "text-[12px] leading-[15px] font-extrabold text-center",
                    "{month}"
                }
            }

            img {
                class: "w-[60px] h-[60px] rounded-[9.57px]",
                src: image,
            }

            div {
                class: "w-full flex flex-col gap-[9px] items-start justify-start",
                span {
                    class: "text-[16px] leading-[23px] font-extrabold",
                    "{title}"
                }
                div {
                    class: "rounded-[4px] px-[6px] py-[4px] bg-[{theme_data.primary03}] text-[10px] leading-[12px] font-extrabold text-[{theme_data.primary05}]",
                    "{date}"
                }
            }
        }
    }
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Invalid month",
    }
}
