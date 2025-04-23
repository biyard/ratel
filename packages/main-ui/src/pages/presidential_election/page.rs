use crate::pages::{SearchBox, components::SectionHeader};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn PresidentialElectionPage(#[props(default = Default::default())] lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: PresidentialElectionTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div {
            id: "presidential-election",
            class: "w-full max-w-desktop flex flex-col gap-50 pt-150 max-tablet:!px-30 max-tablet:!overflow-y-scroll max-tablet:!pt-40 px-10",
            SectionHeader {
                section_name: tr.section_name,
                title: tr.title,
                description: tr.description,
            }

            div { class: "w-full min-w-200 max-w-500 max-tablet:!max-w-full order-1 max-tablet:!order-1",
                SearchBox {
                    placeholder: tr.search_placeholder,
                    value: ctrl.keyword(),
                    onsearch: move |e| ctrl.keyword.set(e),
                }
            }

            for c in ctrl.candidates()? {
                div { class: "col gap-20",
                    div { class: "row justify-between",
                        h2 { class: "text-2xl", {c.name} }
                    }
                    div { class: "row gap-20",
                        img { class: "w-300 h-500", src: c.image }
                        div { class: "col gap-20",
                            p { class: "text-sm", {c.party.translate(&lang)} }
                            p { class: "text-sm", {c.crypto_stance.translate(&lang)} }
                            for p in c.election_pledges {
                                article { dangerous_inner_html: p.promise }
                            }
                        }
                    }
                } // end of candidate
            } // end of candidates

        } // end of page
    }
}
