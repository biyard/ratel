#![allow(non_snake_case)]
use super::feature_section::FeatureSection;
use super::patron_section::PatronSection;
use crate::components::page_title::PageHeader;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NewPatronPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: NewPatronTranslate = translate(&lang);

    rsx! {
        div { id: "new-patron",
            PageHeader { title: "{tr.title}" }
            PatronSection { lang }
            PageHeader { title: "{tr.title}" }
            FeatureSection { lang }
        }
    }
}
