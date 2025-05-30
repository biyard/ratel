use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ExplorePage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ExploreTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "explore", "{tr.title} PAGE" } // end of this page
    }
}
