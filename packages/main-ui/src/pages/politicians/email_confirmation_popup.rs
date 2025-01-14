#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::{
    theme::Theme,
    components::icons,
};
use super::{
    i18n::EmailConfirmPopupTranslate,
    contact_us_popup::ContactUsPopup,
    proclaim_popup::ProclaimPopup,
};

#[component]
pub fn EmailConfirmationPopup(
    #[props(default = "email_confirmation_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    email: String,
    name: String,
    party: String,
    stance: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<EmailConfirmPopupTranslate>(&lang);
    let mut popup: PopupService = use_context();
    let explain = tr.explanation_confirm_email1.replace("{email}", &email);

    let name_clone = name.clone();
    let stance_clone = stance.clone();
    let lang_clone = lang.clone();

    rsx! {
        div { id, class,
            div { class: "flex flex flex-col w-full items-start justify-start gap-[35px] pt-[10px]",
                div { class: "flex flex-col items-center justify-center w-full gap-[30px]",
                    div { class: "flex items-center justify-center rounded-full w-[85px] h-[85px] bg-[{theme.background}]",
                        icons::Logo { width: 36, height: 42}
                    },
                    p { class: "text-center text-[16px] leading-[24px]",
                        span { class: "font-normal", "{explain}" },
                        span { 
                            u { class: "font-bold text-[{theme.highlight}] cursor-pointer",
                                onclick: move |_| {
                                    tracing::debug!("Contact us clicked");
                                    popup
                                        .open(rsx! {
                                            ContactUsPopup {
                                                class: "w-[450px]",
                                                name: name_clone.clone(),
                                                stance: stance_clone.clone(),
                                                lang: lang_clone.clone(),
                                            }
                                        })
                                        .with_id("contact_us_popup")
                                        .with_title(tr.contact_us);
                                    },
                                " {tr.here}" 
                            }
                        },
                        span { class: "font-normal", "{tr.explanation_confirm_email2}" },
                    }
                }
                div { class: "flex w-full",
                    button {
                        class: "w-full h-[57px] text-[{theme.primary05}] bg-[{theme.primary03}] text-[18px] font-extrabold leading-[24px] rounded-[12px] disabled:opacity-50",
                        onclick: move |_| { 
                            tracing::debug!("confirm button clicked");
                            // TODO: check email verification
                            popup.
                                open(
                                    rsx! {
                                        ProclaimPopup {
                                            class: "w-[450px]",
                                            name: name.clone(),
                                            party: party.clone(),
                                            stance: stance.clone(),
                                            lang: lang.clone(),
                                        }
                                    })
                                    .with_id("proclaim_popup")
                                    .with_title(tr.proclaim);

                         },
                        "{tr.confirm_verification}"
                    }
                }
            }
        }
    }
}