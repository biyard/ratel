use crate::pages::landing::{SearchBox, components::SectionHeader};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn PresidentialElectionPageForLanding(
    #[props(default = Default::default())] lang: Language,
) -> Element {
    rsx! {
        PresidentialElectionPage { lang }
    }
}

#[component]
pub fn PresidentialElectionPage(#[props(default = Default::default())] lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: PresidentialElectionTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div {
            id: "presidential-election",
            class: "w-full max-w-desktop flex flex-col gap-72 py-150 max-tablet:!px-30 max-tablet:!overflow-y-scroll max-tablet:!pt-40 px-10",
            SectionHeader {
                section_name: tr.section_name,
                title: tr.title,
                description: tr.description,
            }


            div { class: "flex flex-col gap-24 w-full",
                div { class: "w-full min-w-200 max-w-500 max-tablet:!max-w-full",
                    SearchBox {
                        placeholder: tr.search_placeholder,
                        value: ctrl.keyword(),
                        onsearch: move |e| ctrl.keyword.set(e),
                    }
                }

                div { class: "w-full flex flex-row justify-end font-normal text-[15px]/22 text-c-w-g-30",
                    "Total {ctrl.candidates()?.len()}"
                }

                for candidate in ctrl.candidates()? {
                    CandidateCard { lang, candidate }
                }
            } // end of candidates

        } // end of page
    }
}
