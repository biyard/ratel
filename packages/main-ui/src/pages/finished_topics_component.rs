#![allow(non_snake_case)]
use super::i18n::FinishedTopicTranslate;
use crate::{
    components::{
        button::{RoundedNoButton, RoundedYesButton},
        icon_text::IconText,
        icons,
    },
    theme::Theme,
};
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn FinishedTopics(
    #[props(default ="finished_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    topics: Vec<TopicSummary>,
    lang: Language,
) -> Element {
    let len = if topics.len() > 2 { 2 } else { topics.len() };
    let tr = translate::<FinishedTopicTranslate>(&lang);
    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full justify-start items-start gap-[18px]",
                div { class: "text-[18px] font-semibold", "{tr.finished_topic}" }

                div { class: "w-full grid grid-cols-2 max-[635px]:grid-cols-1 gap-[20px] rounded-[8px] flex items-center justify-center",
                    for topic in topics.iter().take(len) {
                        FinishedTopic {
                            id: "finished-topic-{topic.id}",
                            class: "col-span-1 h-[209px]",
                            image: topic.images.get(0).unwrap_or(&"".to_string()),
                            title: topic.title.to_string(),
                            accepted: match topic.result {
                                Some(TopicResult::Accepted) => true,
                                _ => false,
                            },
                            donations: topic.donations(),
                            replies: topic.replies,
                            lang: lang.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FinishedTopic(
    #[props(default ="finished_topic".to_string())] id: String,
    class: String,
    image: String,
    title: String,
    accepted: bool,
    donations: u64,
    replies: u64,
    lang: Language,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();
    let tr = translate::<FinishedTopicTranslate>(&lang);
    rsx! {
        div { id, class,
            div { class: "border-[{theme_data.primary11}] rounded-[8px] w-full cursor-pointer",
                div { class: "w-full flex flex-col gap-[6px] px-[20px] py-[10px] items-start justify-start cursor-pointer bg-[{theme_data.primary06}] rounded-t-[8px] border-[{theme_data.primary11}] border-opacity-50 hover:border-opacity-100",
                    div { class: "py-[4px] px-[6px] text-[10px] font-extrabold text-[{theme_data.primary05}] bg-[{theme_data.primary03}] rounded-[4px]",
                        "{tr.closed_voting}"
                    }
                    div { class: "flex flex-col gap-[40px] items-start justify-start w-full",
                        div { class: "flex flex-row items-center max-w-full justify-start gap-[10px]",
                            img {
                                class: "w-[42px] h-[42px] rounded-[6px]",
                                src: image,
                            }
                            span { class: "text-[16px] line-clamp-1 font-extrabold",
                                "{title}"
                            }
                        }
                        if accepted {
                            RoundedYesButton { rounded: 6, class: "w-full", disabled: true }
                        } else {
                            RoundedNoButton { rounded: 6, class: "w-full", disabled: true }
                        }
                    }
                }

                div { class: "flex flex-row rounded-b-[8px] bg-[#393B54] border-top-[1px] border-[{theme_data.primary11}] gap-[10px] px-[20px] py-[12px] items-center justify-between",
                    div { class: "flex flex-row items-center gap-[12px]",
                        div { class: "flex flex-row items-center gap-[4px]",
                            icons::Money {}
                            span { class: "text-[14px] font-bold text-[{theme_data.primary00}]",
                                "{tr.cumulative_donations}"
                            }
                        }
                        span { class: "text-[14px] font-bold text-[{theme_data.grey00}] ",
                            {format!("{}₩", donations.to_formatted_string(&Locale::en))}
                        }
                    }
                    IconText { class: "", text: "{replies}", icons::ChatBubble {} }
                }
            }
        }
    }
}
