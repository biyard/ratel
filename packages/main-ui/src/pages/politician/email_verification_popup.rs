#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::{theme::Theme, components::checkbox::Checkbox,};
use super::i18n::PoliticianStanceTranslate;

#[component]
pub fn EmailVerificationPopup(
    #[props(default = "politician_email_verification".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    name: String,
    party: String,
    email: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<PoliticianStanceTranslate>(&lang);

    let mut agreed = use_signal(|| false);

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[35px] pt-[10px]",
                div { class: "flex flex-col w-full gap-[10px]",

                    // NAME
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.name}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: "name",
                            readonly: true,
                        }
                    }

                    // PARTY
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.party}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: party,
                            readonly: true,
                        }
                    }

                    // EMAIL
                    div { class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.email}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: email,
                            readonly: true,
                        }
                    }

                    div { class: "flex flex-row gap-[6px] items-center",
                        Checkbox {
                            class: "cursor-pointer",
                            title: "{tr.agree_email_verification}",
                            onchange: move |check| {
                                agreed.set(check);
                            },
                        }
                    }
                }

                div { class: "flex w-full",
                    button {
                        class: "w-full h-[57px] text-[{theme.primary05}] bg-[{theme.primary03}] text-[18px] font-extrabold leading-[24px] rounded-[12px]",
                        style: if agreed() {
                            "opacity: 1; cursor: pointer;"
                        } else {
                            "opacity: 0.5;"
                        },
                        disabled: !agreed(),
                        "{tr.verify_email}"
                    }
                }
            }
        }
    }
}