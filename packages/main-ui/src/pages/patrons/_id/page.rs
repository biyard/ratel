#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronsByIdPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: PatronsByIdTranslate = translate(&lang);

    rsx! {
        div { id: "patrons-by-id", "{tr.title} PAGE" }
    }
}
