#![allow(non_snake_case)]
use crate::components::{dropdown::Dropdown, party::PartyIcon};
use crate::pages::components::FooterWithSocial;
use crate::pages::components::SectionHeader;

use super::{controller::*, i18n::*};
use bdk::prelude::by_components::icons::edit::Search;
use bdk::prelude::*;
use by_components::hooks::use_scroll::use_scroll;
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

    //TODO(web): have to change header style
    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div { class: "w-full h-[calc(100vh-52px)] max-w-1177 flex flex-col gap-50 pt-150 overflow-y-hidden max-tablet:!px-30 max-tablet:!overflow-y-scroll max-tablet:!pt-40 px-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            div { class: "w-full flex flex-row justify-between items-start gap-10 max-tablet:!flex-col",
                div { class: "flex flex-row gap-10 max-tablet:!w-full order-1 max-tablet:!order-2",
                    div { class: "max-tablet:!w-full",
                        Dropdown {
                            items: CryptoStance::variants(&lang),
                            onselect: move |value| ctrl.set_stance(value),
                        }
                    }

                    div { class: "max-tablet:!w-full",
                        Dropdown {
                            items: Party::variants(&lang),
                            onselect: move |value| ctrl.set_party(value),
                        }
                    }
                }

                div { class: "w-full min-w-300 max-w-500 max-tablet:!max-w-full order-1 max-tablet:!order-1",
                    SearchBox {
                        placeholder: "Search for a  Politcian",
                        value: "",
                        onsearch: move |e| {},
                    }
                }
            }

            //TODO(web): have to make scroll w-full and check chart range
            div { class: "w-full grow overflow-x-scroll mb-52 flex flex-col",
                div { class: "min-w-1100 flex flex-col",
                    div { class: "grid grid-cols-5 items-center bg-bg rounded-t-[8px] w-full",
                        div {
                            class: "px-20 py-14 text-left col-span-1 font-bold text-c-wg-30",
                            onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Name),
                            {tr.th_name}
                        }

                        div {
                            class: "px-20 py-14 text-left col-span-2 font-bold text-c-wg-30",
                            onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Stance),
                            {tr.th_stance}
                        }

                        div {
                            class: "px-20 py-14 text-left col-span-1",
                            onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Party),
                            {tr.th_party}
                        }

                        div {
                            class: "px-20 py-14 text-left col-span-1",
                            onclick: move |_| ctrl.set_sort(dto::AssemblyMemberSorter::Bills),
                            {tr.th_key_actions}
                        }
                    }

                    div { class: "w-full h-full overflow-hidden flex flex-col",
                        div { class: "grow flex flex-col overflow-y-scroll w-full h-screen",
                            for politician in ctrl.politicians()?.items {
                                div {
                                    class: "grid grid-cols-5 items-center border-b border-b-c-wg-80 cursor-pointer min-w-1100 w-full",
                                    onclick: move |_| {
                                        ctrl.go_to_politician_by_id(politician.id);
                                    },

                                    div { class: "px-20 py-14 col-span-1",
                                        div { class: "flex flex-row items-center gap-4",
                                            img {
                                                src: "{politician.image_url}",
                                                class: "w-18 h-18 rounded-[4px] object-cover",
                                            }
                                            div { class: "flex max-tablet:!text-[15px] max-tablet:!flex justify-start items-center min-h-50",
                                                p { class: "font-md text-[15px] max-tablet:!text-[15px] max-tablet:!font-md",
                                                    {politician.name(&lang)}
                                                }
                                            }
                                        }
                                    }

                                    div { class: "px-20 py-14 col-span-2 inline-flex flex-row items-center gap-10",
                                        div { class: "w-8 h-8 rounded-full {politician.stance_color()}" }
                                        div { class: "text-[15px] font-md flex justify-start",
                                            {politician.stance.translate(&lang)}
                                        }
                                    }

                                    div { class: "px-20 py-14 col-span-1",
                                        div { class: "flex flex-row items-center gap-4",
                                            PartyIcon { party: politician.party_enum() }
                                            span { class: "text-white font-medium text-[15px]",
                                                {politician.party(&lang)}
                                            }
                                        }
                                    }

                                    div { class: "px-20 py-14 col-span-1",
                                        {politician.no_of_bills.to_string()}
                                    }
                                }
                            }
                        }
                    } // tbody
                }
            } // table
        } // div

        if !ctrl.is_end() {
            div {
                class: "fixed bottom-52 left-0 w-full h-283 z-10 pointer-events-none",
                style: "background: linear-gradient(180deg, rgba(30, 30, 30, 0) -36.75%, rgba(30, 30, 30, 0.4) 52.94%, #1E1E1E 88.39%);",
            }
        }

        div { class: "fixed bottom-0 left-0 w-full flex items-center justify-center bg-bg z-10",
            div { class: "max-w-1177 w-full flex items-center justify-center mx-10",
                FooterWithSocial { lang }
            }
        }
    }
}

#[component]
pub fn SearchBox(
    #[props(default = "flex flex-row w-full placeholder-[#bebebe] bg-[#white] text-[#222222] focus:outline-none".to_string())]
    class: String,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: String,
    value: String,
    onsearch: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full h-44 justify-start items-center border border-c-wg-70 rounded-full max-tablet:rounded-[8px] px-20",
            input {
                class,
                width,
                height,
                placeholder,
                value,
                onchange: move |e| {
                    onsearch.call(e.value());
                },
            }
            Search {
                width: "24",
                height: "24",
                class: "[&>path]:stroke-[#979797] [&>circle]:stroke-[#979797]",
            }
        }
    }
}
