#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn TopicsByIdPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: TopicsByIdTranslate = translate(&lang);

    rsx! {
        div { id: "topics-by-id", "{tr.title} PAGE" }
    }
}
