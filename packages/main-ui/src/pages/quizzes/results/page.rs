use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ResultsPage(
    #[props(default = Language::En)] lang: Language,
    id: ReadOnlySignal<String>,
) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let _tr: ResultsTranslate = translate(&lang);
    let (_result, candidate) = ctrl.result()?;

    rsx! {
        by_components::meta::MetaPage { title: "{candidate.name}", image: "{candidate.image}" }

        div { id: "results", class: "flex flex-col",
            img {
                src: candidate.image,
                alt: candidate.name,
                class: "w-full max-w-200",
            }

        }
    }
}
