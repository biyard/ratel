#![allow(non_snake_case)]
use super::EmailInput;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Footer(lang: Language) -> Element {
    let _tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full items-start", EmailInput {} }
    }
}

translate! {
    FooterTranslate;

    title_text: {
        ko: "Mine the Future, Cast Your Predictions.",
        en: "Mine the Future, Cast Your Predictions.",
    },

    privacy_policy_button_text: {
        ko: "개인 정보 보호 정책",
        en: "Privacy Policy",
    },
    terms_of_service_button_text: {
        ko: "서비스 약관",
        en: "Terms of Service",
    },
}
