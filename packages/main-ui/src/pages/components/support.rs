#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Support(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: SupportTranslate = translate(&lang);

    rsx! {
        div {
            id: "support",
            class: "w-screen h-screen flex flex-col items-center justify-center",
            ..attributes,
            {tr.title}
            {children}
        }
    }
}

translate! {
    SupportTranslate;

    title: {
        ko: "Support",
        en: "Support",
    },
}
