#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_elements::span;
use dioxus_translate::*;
use crate::theme::Theme;
use super::i18n::PoliticianStanceTranslate;

#[component]
pub fn EmailConfirmationPopup(
    #[props(default = "email_confirmation_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    email: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<PoliticianStanceTranslate>(&lang);

    let explain = tr.explanation_confirm_email1.replace("{email}", &email);

    rsx! {
        div { id, class,
            div { class: "flex flex flex-col w-full items-start justify-start gap-[35px] pt-[10px]",
                div { class: "flex flex-col items-center justify-center w-full gap-[30px]",
                    div { class: "rounded-full w-[85px] h-[85px] bg-[{theme.background}]",
                    },
                    p { class: "text-center text-[16px] leading-[24px]",
                        span { class: "font-normal", "{explain}" },
                        span { 
                            u { class: "font-bold text-[{theme.highlight}] cursor-pointer", " {tr.here}" 
                                // onclick
                            }
                        },
                        span { class: "font-normal", "{tr.explanation_confirm_email2}" },
                    }
                }
                div { class: "flex w-full",
                    button {
                        class: "w-full h-[57px] text-[{theme.primary05}] bg-[{theme.primary03}] text-[18px] font-extrabold leading-[24px] rounded-[12px] opacity-50",
                        onclick: move |_| { /* onclick */ },
                        "{tr.confirm_verification}"
                    }
                }
            }
        }
    }
}