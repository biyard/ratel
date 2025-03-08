#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Community(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: CommunityTranslate = translate(&lang);

    rsx! {
        div {
            id: "community",
            class: "w-screen h-screen flex flex-col items-center justify-center",
            ..attributes,
            {tr.title}
            {children}
        }
    }
}

translate! {
    CommunityTranslate;

    title: {
        ko: "Community",
        en: "Community",
    },
}
