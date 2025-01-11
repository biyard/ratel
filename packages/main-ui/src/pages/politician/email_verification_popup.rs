#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::theme::Theme;
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
                }
            }
        }
    }
}