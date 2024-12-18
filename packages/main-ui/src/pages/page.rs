#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::Topic;

use super::highlighted_topic_component::HighlightedTopic;
use crate::{route::Language, theme::Theme};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new()?;
    let tr = super::i18n::translate_pages(&lang);

    rsx! {
        div {
            class: "flex flex-col",
            HighlightedTopics {
                topics: ctrl.main_topic(),
                onselect: |_| {},
            }
        }
        div { "{tr.text}" }
    }
}

#[component]
pub fn HighlightedTopics(topics: Vec<Topic>, onselect: EventHandler<usize>) -> Element {
    let mut selected = Signal::new(0);
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div {
            class: "flex flex-col",
            for (i, topic) in topics.iter().enumerate() {
                if i == selected() {
                    HighlightedTopic {
                    id: topic.id.clone(),
                    // image: topic.image.clone(),
                    // title: topic.title.clone(),
                    // description: topic.description.clone(),
                    // period: topic.period.clone(),
                    // donations: topic.donations,
                    // replies: topic.replies,
                    // yes: topic.yes,
                    // no: topic.no,
                }
                }
            }

            div {
                class: "flex flex-row w-full items-center justify-center gap-[10px] p-[10px]",
                for i in 0..topics.len() {
                    div {
                        class: format!(
                            "h-[8px] transition-all rounded-full cursor-pointer {} bg-[{}] hover:bg-white",
                            if i == selected() {
                                "w-[90px]"
                            } else {
                                "w-[52px] hover:w-[70px]"
                            },
                            theme_data.primary06
                        ),
                        onclick: move |_| {
                            selected.set(i);
                            onselect(i);
                        },
                    }
                }
            }
        }
    }
}
