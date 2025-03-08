#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn About(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: AboutTranslate = translate(&lang);

    rsx! {
        div {
            id: "about",
            class: "w-screen h-screen flex flex-col items-center justify-center",
            ..attributes,
            {tr.title}
            {children}
        }
    }
}

translate! {
    AboutTranslate;

    title: {
        ko: "About",
        en: "About",
    },
}
