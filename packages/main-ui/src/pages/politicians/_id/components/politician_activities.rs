#![allow(non_snake_case)]
use bdk::prelude::*;
use dto::{BillSorter, BillSummary};

use crate::components::dropdown::Dropdown;

#[component]
pub fn PoliticianActivities(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = vec![])] bills: Vec<BillSummary>,
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

            div { class: "w-full flex items-start justify-start gap-10",
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
    rsx! {
        div { ..attributes,{children} }
    }
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
