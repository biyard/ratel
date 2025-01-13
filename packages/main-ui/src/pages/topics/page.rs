#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn TopicsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: TopicsTranslate = translate(&lang);

    rsx! {
        div { id: "topics", "{tr.title} PAGE" }
    }
}
