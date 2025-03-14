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

            table { class: "rounded-[8px] w-full overflow-hidden",
                thead { class: "bg-bg",
                    tr { class: "bg-bg rounded-t-[8px] w-full overflow-hidden",
                        th { class: "px-20 py-14 text-left w-250", {tr.th_name} }
                        th { class: "px-20 py-14 text-left w-250", {tr.th_stance} }
                        th { class: "px-20 py-14 text-left w-250", {tr.th_party} }
                        th { class: "px-20 py-14 text-left w-427", {tr.th_key_actions} }
                    }
                }

                tbody {
                    for politician in ctrl.politicians()?.items.iter() {
                        tr { class: "border-b border-b-c-wg-80",
                            td { class: "px-20 py-14",
                                div { class: "flex flex-row items-center gap-4",
                                    img {
                                        src: "{politician.image_url}",
                                        class: "w-18 h-18 rounded-[4px] object-cover",
                                    }
                                    {politician.name(&lang)}
                                }
                            }
                            td { class: "px-20 py-14 inline-flex flex-row items-center gap-10",
                                div { class: "w-8 h-8 rounded-full {politician.stance_color()}" }
                                {politician.stance.translate(&lang)}
                            }
                            td { class: "px-20 py-14 ",
                                span { class: "bg-blue-600 text-white px-2 py-1 rounded-full",
                                    "{politician.party}"
                                }
                            }
                            td { class: "px-20 py-14", "" }
                        }
                    }
                }
            }
        }
    }
}
