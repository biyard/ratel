mod controller;
mod i18n;

use bdk::prelude::{by_components::icons::arrows::ArrowRight, *};
use controller::*;
use i18n::*;

use crate::{
    components::candidate_card::CandidateCard, pages::components::SectionHeader, route::Route,
};

#[component]
pub fn PresidentialElection(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: PresidentialElectionTranslate = translate(&lang);

    rsx! {
        section {
            id: "presidential-election",
            class: "w-full max-w-desktop min-h-screen flex flex-col items-start justify-start pt-120 gap-50 max-tablet:pt-68 max-tablet:!h-auto overflow-hidden",
            SectionHeader {
                section_name: tr.section_name,
                title: tr.title,
                description: tr.description,
            }

            p { class: "text-primary text-[32px]/22 font-bold flex flex-row items-center gap-20",
                if let Ok(candidates) = ctrl.candidates() {
                    "{candidates.len()}"
                } else {
                    "0"
                }
                span { class: "text-white text-xl/22", {tr.total_candidates} }
            }

            div { class: "w-full flex flex-col gap-30 items-center",
                div { class: "w-full grid grid-cols-2 gap-24",
                    for candidate in ctrl.candidates()?.into_iter().take(2) {
                        CandidateCard { lang, candidate }
                    }
                }

                Link {
                    class: "btn secondary sm",
                    to: Route::PresidentialElectionPage {
                        lang,
                    },
                    "View All"
                    ArrowRight {
                        class: "[&>path]:stroke-3",
                        width: "15",
                        height: "15",
                    }

                }
            }
        }
    }
}
