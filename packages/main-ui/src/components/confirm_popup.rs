#![allow(non_snake_case)]
use crate::{
    components::{button::primary_button::PrimaryButton, icons::CharacterWithCircle},
    route::Route,
};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn ConfirmPopup(
    #[props(default ="welcome_popup".to_string())] id: String,
    lang: Language,
    title: String,
    description: String,
    btn_label: String,
) -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class: "max-w-390 w-full",
            div { class: "w-full flex flex-col gap-35",
                WelcomeHeader { lang, title, description }

                PrimaryButton {
                    width: "100%",
                    onclick: move |_| {
                        popup.close();
                    },
                    {btn_label}
                }
            }

            SigninPopupFooter { lang }
        }
    }
}

#[component]
pub fn WelcomeHeader(lang: Language, title: String, description: String) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-24 items-center justify-center mt-35",
            p { class: "text-white font-bold text-2xl", {title} }
            CharacterWithCircle { size: 100 }
            p { class: "text-neutral-400 text-center text-base font-medium", {description} }
        }
    }
}

#[component]
pub fn SigninPopupFooter(lang: Language) -> Element {
    let tr = translate::<SigninPopupFooterTranslate>(&lang);
    let mut popup: PopupService = use_context();
    rsx! {
        Link {
            to: Route::PrivacyPolicyPage { lang },
            class: "flex flex-row gap-10 mt-35 justify-center w-full",
            onclick: move |_| {
                tracing::debug!("Privacy policy clicked");
                popup.close();
            },
            div { class: "cursor-pointer",
                span { class: "text-neutral-400 text-xs/14 font-medium", {tr.privacy_policy} }
            }
            div { class: "cursor-pointer",
                span { class: "text-neutral-400 text-xs/14 font-medium", {tr.term_of_service} }
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
