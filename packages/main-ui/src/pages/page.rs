#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::route::Language;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new()?;
    let tr = super::i18n::translate_pages(&lang);

    rsx! {
        div { "{tr.text}" }
    }
}
