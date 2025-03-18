#![allow(non_snake_case)]
use crate::components::dropdown::Dropdown;
use crate::pages::components::SectionHeader;

use super::{controller::*, i18n::*, *};
use bdk::prelude::*;
use by_components::hooks::use_scroll::use_scroll;
use components::party::PartyIcon;
use dto::CryptoStance;
use dto::Party;

#[component]
pub fn PoliticiansPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: PoliticiansTranslate = translate(&lang);
    use_scroll(move |_, y| {
        tracing::debug!("scrolling: {}", y);
    });

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div { class: "relative w-full max-w-1177 flex flex-col gap-50 w-full mt-150",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            div { class: "flex flex-row gap-10",
                Dropdown {
                    items: CryptoStance::variants(&lang),
                    onselect: move |value| ctrl.set_stance(value),
                }

                Dropdown {
                    items: Party::variants(&lang),
                    onselect: move |value| ctrl.set_party(value),
                }
            }

            div { class: "w-full overflow-x-scroll",
                table { class: "rounded-[8px] w-full min-w-1000 max-h-[calc(100vh-100px)]",
                    thead {
                        tr { class: "bg-bg rounded-t-[8px] w-full overflow-hidden",
                            th {
                                class: "px-20 py-14 text-left w-250",
                                onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Name),
                                {tr.th_name}
                            }
                            th {
                                class: "px-20 py-14 text-left w-250",
                                onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Stance),
                                {tr.th_stance}
                            }
                            th {
                                class: "px-20 py-14 text-left w-250",
                                onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Party),
                                {tr.th_party}
                            }
                            th {
                                class: "px-20 py-14 text-left w-427",
                                onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Bills),
                                {tr.th_key_actions}
                            }
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
                                    div { class: "flex flex-row items-center gap-4",
                                        PartyIcon { party: politician.party_enum() }
                                        span { class: "text-white font-medium text-[15px]",
                                            {politician.party(&lang)}
                                        }
                                    }
                                }
                                td { class: "px-20 py-14", {politician.no_of_bills.to_string()} }
                            }
                        }
                    }
                }
            }
        }
    }
}
