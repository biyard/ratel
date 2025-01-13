#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: PatronsTranslate = translate(&lang);

    rsx! {
        div { id: "patrons", "{tr.title} PAGE" }
    }
}
