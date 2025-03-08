#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PoliticianStance(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: PoliticianStanceTranslate = translate(&lang);

    rsx! {
        div {
            id: "politician-stance",
            class: "w-screen h-screen flex flex-col items-center justify-center",
            ..attributes,
            {tr.title}
            {children}
        }
    }
}

translate! {
    PoliticianStanceTranslate;

    title: {
        ko: "PoliticianStance",
        en: "PoliticianStance",
    },
}
