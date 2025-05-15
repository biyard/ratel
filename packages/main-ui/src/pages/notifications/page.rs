use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn NotificationsPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: NotificationsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "notifications", "{tr.title} PAGE" } // end of this page
    }
}
