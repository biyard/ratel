#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn UpcomingTopics(
    #[props(default ="upcoming_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
) -> Element {
    rsx! {
        div { id, class, "UpcomingTopics page"}
    }
}
