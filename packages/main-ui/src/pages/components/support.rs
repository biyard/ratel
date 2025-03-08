#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use super::*;

#[component]
pub fn Support(lang: Language) -> Element {
    let tr: SupportTranslate = translate(&lang);

    rsx! {
        div {
            id: "support",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
                with_line: false,
            }
        }
    }
}

translate! {
    SupportTranslate;

    title: {
        ko: "Support",
        en: "Support",
    },

    mission: {
        ko: "지원이 필요하신가요?",
        en: "Need Support?",
    },

    description: {
        ko: "문의하기 위해 양식을 작성하세요.",
        en: "Fill in the form to get in touch.",
    }
}
