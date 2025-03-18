#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use components::*;
use controller::*;
use i18n::*;

#[component]
pub fn PoliticiansByIdPage(id: ReadOnlySignal<i64>, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: PoliticiansByIdTranslate = translate(&lang);
    let p = ctrl.politician()?;

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            id: "politicians-by-id",
            class: "w-full grow max-w-1177 mt-160 flex flex-col justify-start",
            PoliticianHeader {
                lang,
                image: p.image_url.clone(),
                name: p.name(&lang),
                party: p.party_enum(),
                stance: p.stance,
                email: p.email.clone().unwrap_or_default(),
            }
            PoliticianActivities {}
            "{p:?}"
        } // end of this page
    }
}
