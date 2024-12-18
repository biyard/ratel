#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    pages::{
        finished_topics_component::FinishedTopics, highlighted_topic_component::HighlightedTopics,
        upcoming_topics_component::UpcomingTopics,
    },
    route::Language,
};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new()?;
    let _tr = super::i18n::translate_pages(&lang);

    rsx! {
        div {
            class: "flex flex-col gap-[100px]",
            HighlightedTopics {
                topics: ctrl.ongoing_topics(),
                onselect: |_| {},
            }
            div {
                class: "w-full flex flex-row gap-[20px] grid-cols-1 lg:grid-cols-2",
                FinishedTopics {
                    class: "col-span-1",
                }
                UpcomingTopics {
                    class: "col-span-1",
                }
            }
        }
    }
}
