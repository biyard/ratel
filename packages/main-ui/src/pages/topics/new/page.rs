#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use super::legislation_selector::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NewTopicPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: NewTopicTranslate = translate(&lang);

    rsx! {
        div { id: "creation",
            LegislationSelector { lang }
        }
    }
}
