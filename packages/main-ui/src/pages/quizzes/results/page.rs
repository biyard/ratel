use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ResultsPage(
    #[props(default = Language::En)] lang: Language,
    id: ReadOnlySignal<String>,
) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ResultsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "results", "{tr.title} PAGE" } // end of this page
    }
}
