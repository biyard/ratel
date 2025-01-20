#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::TopicCreateRequest;

#[component]
pub fn WriteTopic(lang: Language, onclick: EventHandler<TopicCreateRequest>) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: WriteTopicTranslate = translate(&lang);

    rsx! {
        div { id: "new", "{tr.title} PAGE" }
    }
}
