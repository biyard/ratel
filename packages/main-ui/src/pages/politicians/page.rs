#![allow(non_snake_case)]
use crate::components::dropdown::Dropdown;
use crate::pages::components::FooterWithSocial;
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
    use_scroll(move |_, y, is_end, _| {
        tracing::debug!("scrolling: {}", y);
        if is_end {
            ctrl.is_end.set(true);
        } else if !is_end && ctrl.is_end() {
            ctrl.is_end.set(false);
        }
    });

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div { class: "w-full h-[calc(100vh-52px)] max-w-1177 flex flex-col gap-50 pt-150 overflow-y-hidden",
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

            div { class: "w-full grow overflow-x-scroll mb-52 flex flex-col",
                div { class: "flex flex-row items-center bg-bg rounded-t-[8px] w-full",
                    div {
                        class: "px-20 py-14 text-left w-250",
                        onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Name),
                        {tr.th_name}
                    }
                    div {
                        class: "px-20 py-14 text-left w-250",
                        onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Stance),
                        {tr.th_stance}
                    }
                    div {
                        class: "px-20 py-14 text-left w-250",
                        onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Party),
                        {tr.th_party}
                    }
                    div {
                        class: "px-20 py-14 text-left w-427",
                        onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Bills),
                        {tr.th_key_actions}
                    }
                }

                div { class: "w-full h-full overflow-hidden flex flex-col",
                    div { class: "grow flex flex-col overflow-y-scroll w-full",
                        for politician in ctrl.politicians()?.items {
                            div {
                                class: "flex flex-row items-center border-b border-b-c-wg-80",
                                onclick: move |_| {
                                    ctrl.go_to_politician_by_id(politician.id);
                                },
                                div { class: "px-20 py-14 w-250",
                                    div { class: "flex flex-row items-center gap-4",
                                        img {
                                            src: "{politician.image_url}",
                                            class: "w-18 h-18 rounded-[4px] object-cover",
                                        }
                                        {politician.name(&lang)}
                                    }
                                }
                                div { class: "px-20 py-14 w-250 inline-flex flex-row items-center gap-10",
                                    div { class: "w-8 h-8 rounded-full {politician.stance_color()}" }
                                    {politician.stance.translate(&lang)}
                                }
                                div { class: "px-20 py-14 w-250",
                                    div { class: "flex flex-row items-center gap-4",
                                        PartyIcon { party: politician.party_enum() }
                                        span { class: "text-white font-medium text-[15px]",
                                            {politician.party(&lang)}
                                        }
                                    }
                                }
                                div { class: "px-20 py-14 w-427",
                                    {politician.no_of_bills.to_string()}
                                }
                            }
                        }
                    }
                } // tbody
            } // table
        } // div

        if !ctrl.is_end() {
            div {
                class: "fixed bottom-52 left-0 w-full h-283 z-10 pointer-events-none",
                style: "background: linear-gradient(180deg, rgba(30, 30, 30, 0) -36.75%, rgba(30, 30, 30, 0.4) 52.94%, #1E1E1E 88.39%);",
            }
        }

        div { class: "fixed bottom-0 left-0 w-full h-52 flex items-center justify-center bg-bg z-10",
            div { class: "max-w-1177 w-full flex items-center justify-center",
                FooterWithSocial { lang }
            }
        }
    }
}
