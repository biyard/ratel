use super::*;
use bdk::prelude::{by_components::icons::arrows::ShapeArrowDown, *};
use dto::*;

use crate::components::{crypto_stance::CryptoStanceIcon, party::PartyIcon};

#[component]
pub fn CandidateCard(lang: Language, candidate: PresidentialCandidateSummary) -> Element {
    rsx! {
        div {
            id: "presidential-candidate-{candidate.id}",
            class: "w-full flex flex-col gap-24 p-24 rounded-[20px] bg-component-bg",
            div { class: "w-full flex flex-row gap-24 max-mobile:flex-col max-mobile:gap-10 ",
                img {
                    class: "w-121 h-121 object-cover rounded-[10px]",
                    src: candidate.image,
                    alt: "{candidate.name}",
                }

                div { class: "flex flex-col gap-10",
                    div { class: "text-20/28 font-bold text-c-w-g-100",
                        h2 { class: "text-2xl/40 font-bold", "{candidate.name}" }
                        div { class: "flex flex-row items-center gap-8 text-[15px]/22 font-medium",
                            PartyIcon { party: candidate.party, size: 20 }
                            {candidate.party.translate(&lang)}
                        }
                    }

                    div { class: "flex flex-row gap-4 py-8 px-16 rounded-full bg-background font-bold text-sm/22 text-tag items-center text-center",
                        CryptoStanceIcon { stance: candidate.crypto_stance, size: 18 }
                        {candidate.crypto_stance.translate(&lang)}
                    }
                }
            } // candidate header

            div { class: "flex flex-col gap-8",
                for promise in candidate.election_pledges.iter().take(3) {
                    ElectionPledgeCard { promise: promise.clone() }
                }
                button {
                    class: "w-full rounded-[10px] bg-neutral-800 py-6 flex flex-row items-center gap-4 text-sm/22 font-semibold text-white justify-center hover:bg-neutral-700 cursor-pointer transition-all duration-200 ease-in-out group aria-hidden:hidden",
                    "aria-hidden": candidate.election_pledges.len() < 4,
                    "See More"
                    ShapeArrowDown { class: "[&>path]:stroke-neutral-600 [&>path]:fill-neutral-600 group-hover:[&>path]:stroke-neutral-400 group-hover:[&>path]:fill-neutral-400 transition-all duration-200 ease-in-out" }
                }
            } // end of candidate body

        }
    }
}
