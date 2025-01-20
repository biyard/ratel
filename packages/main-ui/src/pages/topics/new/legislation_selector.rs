#![allow(non_snake_case)]
use crate::components::page_title::PageHeader;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn LegislationSelector(lang: Language, onclick: EventHandler<Option<String>>) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: LegislationSelectorTranslate = translate(&lang);

    rsx! {
        div { id: "creation",
            PageHeader { title: "{tr.title}" }
        }
    }
}
