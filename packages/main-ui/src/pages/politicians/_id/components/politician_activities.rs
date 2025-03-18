#![allow(non_snake_case)]
use bdk::prelude::*;
use dto::BillSummary;

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
        div { class: "w-full flex flex-col" }
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
