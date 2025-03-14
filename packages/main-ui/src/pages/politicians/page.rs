#![allow(non_snake_case)]
use crate::components::dropdown::Dropdown;
use crate::pages::components::SectionHeader;

use super::controller::*;
use super::i18n::*;
use bdk::prelude::*;
use dto::CryptoStance;
use dto::Party;

#[component]
pub fn PoliticiansPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: PoliticiansTranslate = translate(&lang);
    let politicians = ctrl.politicians()?;

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div { class: "w-full max-w-1177 flex flex-col gap-50 w-full",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            div { class: "flex flex-row gap-10",
                Dropdown {
                    items: CryptoStance::variants(&lang),
                    onselect: move |value| {
                        tracing::debug!("selected: {}", value);
                    },
                }

                Dropdown {
                    items: Party::variants(&lang),
                    onselect: move |value| {
                        tracing::debug!("selected: {}", value);
                    },
                }
            }

            for p in politicians.items.iter() {
                "{p:?}"
            }
        }
    }
}
