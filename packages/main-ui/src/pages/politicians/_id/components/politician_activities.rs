#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::{BillSorter, BillSummary};

use super::*;
use crate::components::dropdown::Dropdown;

#[component]
pub fn PoliticianActivities(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = vec![])] bills: Vec<Bill>,
    name: String,
    children: Element,
) -> Element {
    let tr: PoliticianActivitiesTranslate = translate(&lang);
    let description = match lang {
        Language::En => format!(
            "Here are some key legislative proposals related to cryptocurrency that {name} has been involved with."
        ),
        Language::Ko => format!("{name}이(가) 관련된 암호화폐와 관련된 주요 입법 제안 목록입니다."),
    };

    rsx! {
        div { class: "w-full flex flex-col items-center py-100 gap-35",
            div { class: "flex flex-col gap-8 items-center",
                h2 { class: "text-xl/22 font-bold text-text-primary", {tr.title} }
                p { class: "text-[15px]/22 font-normal text-text-secondary", {description} }
            }

            div { class: "w-full flex flex-col items-start justify-start gap-10",
                Dropdown {
                    items: BillSorter::variants(&lang),
                    onselect: |item: String| {
                        let sort: BillSorter = item.parse().unwrap_or_default();
                        tracing::debug!("Selected item: {:?}", item);
                    },
                }

                div {
                    id: "politician-bills",
                    class: "w-full flex flex-col gap-24",
                    for bill in bills {
                        BillCard { lang, bill }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BillCard(
    lang: Language,
    bill: BillSummary,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut popup: PopupService = use_context();
    let tr: BillCardTranslate = translate(&lang);
    let (yes, no) = bill.votes_percent();

    rsx! {
        div {
            class: "w-full p-30 flex flex-col gap-20 bg-bg rounded-[20px]",
            id: "bill-card-{bill.id}",
            div {
                id: "bill-card-header-{bill.id}",
                class: "w-full flex flex-col gap-10 items-start justify-start",
                h2 { class: "text-text-primary text-[20px]/25 font-medium", {bill.title(lang)} }
            }

            div {
                id: "bill-card-body-{bill.id}",
                class: "w-full flex flex-col items-start justify-start whitespace-pre-line",
                p { class: "text-[15px]/24 font-normal text-text-secondary",
                    {bill.summary.unwrap_or_default()}
                }
                div { class: "w-full flex flex-row justify-end",
                    a {
                        target: "_blank",
                        href: bill.site_url,
                        class: "text-[15px]/24 font-normal text-primary text-text-primary underline",
                        {tr.see_more}
                    }
                }
            }

            div {
                id: "bill-card-vote-result-{bill.id}",
                class: "w-full flex flex-col items-start justify-start gap-10",
                div { class: "w-full flex flex-row items-center justify-start",
                    div { class: "flex flex-row gap-4",
                        VoteIcon {}
                        p { class: "text-text-primary font-semibold text-xl/25 whitespace-pre-line",
                            "{bill.votes.len()} "
                            span { class: "text-text-secondary", {tr.votes} }
                        }
                    }

                // FIXME: No information yet: https://www.figma.com/design/YaLSz7dzRingD7CipyaC47/Ratel?node-id=183-9407&t=nGj9h0tjcpZm2O54-1
                // div { class: "flex flex-row gap-4",
                //     RewardCoin {}
                //     p { class: "text-text-primary font-semibold text-xl/25 whitespace-pre-line",
                //         "{bill.votes.len()} "
                //         span { class: "text-text-secondary", {tr.votes} }
                //     }
                // }
                }

                div { class: "w-full h-16 rounded-full overflow-hidden gap-5 flex flex-row",
                    div { class: "h-full", width: "{yes}%",
                        div { class: "animate-grow h-full bg-supportive/50" }
                    }
                    div { class: "relative h-full", width: "{no}%",
                        div { class: "absolute right-0 animate-grow h-full bg-against/50" }

                    }
                }

                div { class: "flex flex-row gap-20 py-5 items-center",
                    div { class: "flex flex-row gap-4 items-center",
                        div { class: "w-10 h-10 bg-supportive rounded-full" }
                        span { class: "text-text-secondary font-semibold text-sm/20",
                            "{yes as i32}%"
                        }
                    }

                    div { class: "flex flex-row gap-4 items-center",
                        div { class: "w-10 h-10 bg-against rounded-full" }
                        span { class: "text-text-secondary font-semibold text-sm/20",
                            "{no as i32}%"
                        }
                    }
                }
            }

            // FIXME: reflect my voted and finalized result
            // Refer to Figma(https://www.figma.com/design/YaLSz7dzRingD7CipyaC47/Ratel?node-id=183-9407&t=ntliyRgUTCrimYsj-1)
            div { id: "bill-card-vote-{bill.id}", class: "flex flex-row gap-10",
                button {
                    class: "w-full border border-supportive/25 hover:border-supportive hover:bg-supportive/25 rounded-[10px] flex items-center justify-center cursor-pointer px-16 py-8 text-sm font-bold",
                    onclick: move |_| {
                        tracing::debug!("Vote supportive clicked");
                        popup.open(rsx! {
                            VoteConfirm { vote: dto::VoteOption::Supportive, lang }
                        });
                    },
                    span { {tr.btn_supportive} }
                }
                button {
                    class: "w-full border border-against/25 hover:border-against hover:bg-against/25 rounded-[10px] flex items-center justify-center cursor-pointer px-16 py-8 text-sm font-bold",
                    onclick: move |_| {
                        tracing::debug!("Vote against clicked");
                        popup.open(rsx! {
                            VoteConfirm { vote: dto::VoteOption::Against, lang }
                        });
                    },
                    span { {tr.btn_against} }
                }
            }
        }
    }
}

#[component]
pub fn RewardCoin(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "21",
            view_box: "0 0 21 21",
            width: "21",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: "10.2314",
                cy: "10.0708",
                r: "7.5",
                stroke: "#BAB175",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M13.1483 12.9875C13.1483 10.4875 11.0852 10.0708 10.2446 10.0708C8.37685 10.0708 7.31494 10.0708 7.31494 10.0708V12.9875",
                stroke: "#BAB175",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M7.31494 7.15405H11.6899C12.4953 7.15405 13.1483 7.80698 13.1483 8.61241V8.61241C13.1483 9.41785 12.4953 10.0708 11.6899 10.0708H7.31494V7.15405Z",
                stroke: "#BAB175",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}

#[component]
pub fn VoteIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "21",
            view_box: "0 0 21 21",
            width: "21",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M3.3042 10.5056H17.1584V15.1358C17.1584 16.7927 15.8152 18.1358 14.1584 18.1358H6.3042C4.64735 18.1358 3.3042 16.7927 3.3042 15.1358V10.5056Z",
                stroke: "#777677",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M8.56494 14.3208H11.8983",
                stroke: "#777677",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M6.92399 8.00562H4.97087L3.3042 10.5056",
                stroke: "#777677",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M14.3722 8.00562H15.492L17.1587 10.5056",
                stroke: "#777677",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M6.80682 10.4877L8.94282 3.85433C9.1121 3.32863 9.67549 3.03969 10.2012 3.20897L13.85 4.38393C14.3757 4.55321 14.6647 5.11661 14.4954 5.64231L13.1975 9.67287",
                stroke: "#777677",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}

translate! {
    BillCardTranslate;

    see_more: {
        ko: "더보기",
        en: "See more",
    },

    votes: {
        ko: "투표",
        en: "Votes",
    },

    btn_supportive: {
        ko: "찬성",
        en: "Supportive",
    },

    btn_against: {
        ko: "반대",
        en: "Against",
    },
}

// #[derive(Debug, Clone, Copy, DioxusController)]
// pub struct Controller {
//     #[allow(dead_code)]
//     lang: Language,
// }

// impl Controller {
//     pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
//     let ctrl = Self { lang  };

//     Ok(ctrl)
//     }
// }

translate! {
    PoliticianActivitiesTranslate;

    title: {
        ko: "입법활동",
        en: "Legislative Activities",
    },

    description: {
        ko: "{politician name}이(가) 관련된 암호화폐와 관련된 주요 입법 제안 목록입니다.",
        en: "Here are some key legislative proposals related to cryptocurrency that {politician name} has been involved with.",
    },
}
