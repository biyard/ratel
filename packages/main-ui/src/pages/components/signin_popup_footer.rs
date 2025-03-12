#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SigninPopupFooter(lang: Language) -> Element {
    let tr = translate::<SigninPopupFooterTranslate>(&lang);
    rsx! {
        // TODO: applying policy and terms.
        div { class: "flex flex-row gap-10 mt-35 justify-center",
            button {
                class: "cursor-pointer",
                onclick: move |_| {
                    tracing::debug!("Privacy policy clicked");
                },
                span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.privacy_policy}" }
            }
            button {
                class: "cursor-pointer",
                onclick: move |_| {
                    tracing::debug!("Privacy policy clicked");
                },
                span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.term_of_service}" }
            }
        }
    }
}

translate! {
    SigninPopupFooterTranslate;

    privacy_policy: {
        ko: "개인정보 처리방침",
        en: "Privacy Policy",
    },

    term_of_service: {
        ko: "이용약관",
        en: "Term of Service",
    },
}
