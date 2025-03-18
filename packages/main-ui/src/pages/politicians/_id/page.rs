#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn PoliticiansByIdPage(id: ReadOnlySignal<i64>, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: PoliticiansByIdTranslate = translate(&lang);
    let p = ctrl.politician();

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "politicians-by-id", "{p:?}" } // end of this page
    }
}
