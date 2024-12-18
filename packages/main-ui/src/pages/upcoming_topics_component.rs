#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::Topic;

#[component]
pub fn UpcomingTopics(
    #[props(default ="upcoming_topics".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    _topics: Vec<Topic>,
) -> Element {
    rsx! {
        div { id, class, "UpcomingTopics page"}
    }
}
