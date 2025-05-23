use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn MessagesPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: MessagesTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "messages", "{tr.title} PAGE" } // end of this page
    }
}
