#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Top(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: TopTranslate = translate(&lang);

    rsx! {
        div {
            id: "top",
            class: "w-screen h-screen flex flex-col items-center justify-center",
            ..attributes,
            {tr.title}
            {children}
        }
    }
}

translate! {
    TopTranslate;

    title: {
        ko: "Top",
        en: "Top",
    },
}
