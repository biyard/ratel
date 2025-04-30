mod i18n;

use bdk::prelude::{by_components::icons::arrows::ArrowRight, *};
use dto::*;
use i18n::*;

use crate::{
    components::{candidate_card::CandidateCard, coming_soon::ComingSoon},
    pages::components::SectionHeader,
    route::Route,
};

#[component]
pub fn PresidentialElection(
    lang: Language,
    candidates: Vec<PresidentialCandidateSummary>,
) -> Element {
    let tr: PresidentialElectionTranslate = translate(&lang);
    let l = candidates.len();

    rsx! {
        section {
            id: "presidential-election",
            class: "w-full max-w-desktop min-h-screen flex flex-col items-start justify-start pt-120 gap-50 max-tablet:pt-68 max-tablet:!h-auto overflow-hidden",
            SectionHeader {
                section_name: tr.section_name,
                title: tr.title,
                description: tr.description,
            }

            p {
                class: "text-primary text-[32px]/22 font-bold flex flex-row items-center gap-20 aria-hidden: hidden",
                "aria-hidden": l == 0,
                "{l}"
                span { class: "text-white text-xl/22", {tr.total_candidates} }
            }

            div {
                class: "w-full flex flex-col gap-30 items-center aria-hidden:hidden overflow-x-scroll items-start",
                "aria-hidden": l == 0,
                div { class: "w-full grid grid-cols-2 gap-24 max-tablet:min-w-624 order-1 max-tablet:!order-2",
                    for candidate in candidates.into_iter().take(2) {
                        CandidateCard { lang, candidate }
                    }
                }

                div { class: "w-full flex flex-row justify-center order-2 max-tablet:!order-1",
                    Link {
                        class: "btn secondary sm",
                        to: Route::PresidentialElectionPage {},
                        "View All"
                        ArrowRight {
                            class: "[&>path]:stroke-3",
                            width: "15",
                            height: "15",
                        }
                    }
                }
            }

            ComingSoon {
                class: "w-full h-430 hidden aria-show:block",
                "aria-show": l == 0,
            }
        }
    }
}
