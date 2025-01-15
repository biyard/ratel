#![allow(non_snake_case)]
use super::i18n::UpcomingTopicsTranslate;
use crate::theme::Theme;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::TopicSummary;

#[component]
pub fn UpcomingTopics(
    #[props(default ="upcoming_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    topics: Vec<TopicSummary>,
    lang: Language,
) -> Element {
    let tr = translate::<UpcomingTopicsTranslate>(&lang);
    rsx! {
        div { id, class,
            div { class: "flex flex-col gap-[16px] items-start justify-start w-full",
                span { class: "text-[18px] font-semibold", "{tr.soon_voting}" }
                for topic in topics.iter().take(2) {
                    UpcomingTopic {
                        day: topic.day(),
                        month: topic.month(),
                        // FIXME: provide default image
                        image: "{topic.images[0]}",
                        title: "{topic.title}",
                        date: topic.date(),
                    }
                }
            }
        }
    }
}

#[component]
pub fn UpcomingTopic(
    day: String,
    month: String,
    image: String,
    title: String,
    #[props(default = "1월 19일".to_string())] date: String,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div { class: "w-full flex flex-row gap-[19px] items-center justify-start px-[24px] py-[20px] rounded-[8px] bg-[{theme_data.primary07}]",
            div { class: " flex flex-col gap-[4px] items-center justify-start",
                span { class: "text-[24px] leading-[25px] font-extrabold text-center",
                    "{day}"
                }
                span { class: "text-[12px] leading-[15px] font-extrabold text-center",
                    "{month}"
                }
            }

            img { class: "w-[60px] h-[60px] rounded-[9.57px]", src: image }

            div { class: "w-full flex flex-col gap-[9px] items-start justify-start",
                span { class: "text-[16px] leading-[23px] font-extrabold", "{title}" }
                div { class: "rounded-[4px] px-[6px] py-[4px] bg-[{theme_data.primary03}] text-[10px] leading-[12px] font-extrabold text-[{theme_data.primary05}]",
                    "{date}"
                }
            }
        }
    }
}
