#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use crate::{
    theme::Theme,
    components::{
        icons,
        dropdown::Dropdown,
        checkbox::Checkbox,
        tooltip::Tooltip,
    },
};
use super::i18n::ContactUsPopupTranslate;
use dto::CryptoStance;

#[component]
pub fn ContactUsPopup(
    #[props(default = "contact_us_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    name: String,
    stance: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<ContactUsPopupTranslate>(&lang);

    let mut homepage_signal = use_signal(|| "".to_string());
    let mut contact_email_signal = use_signal(|| "".to_string());
    let mut stance_signal = use_signal(|| stance);
    let mut agreed = use_signal(|| false);

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[35px] pt-[10px]",
                div { class: "flex flex-col w-full gap-[10px]",

                    div {
                        id: "contact-us-popup-name",
                        class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.name}" }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            readonly: true,
                            value: name,
                        }
                    }

                    div {
                        id: "contact-us-popup-homepage",
                        class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "{tr.homepage}" }
                            Tooltip {
                                inner_class: "text-xs text-white font-bold bg-[#2C2E42] px-[15px] py-[10px] rounded-[8px] shadow-2xl w-[230px] h-[80px]"
                                    .to_string(),
                                text: "{tr.tooltip}",
                                bg_color: "#2C2E42".to_string(),
                                icons::Tooltip { color: "#ADBCD7" }
                            }
                        }
                        input {
                            class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: "https://",
                            value: homepage_signal(),
                            oninput: move |e| {
                                let mut value = e.value();
                                if !value.is_empty() && !value.starts_with("http://")
                                    && !value.starts_with("https://")
                                {
                                    value = format!("https://{}", value);
                                }
                                homepage_signal.set(value);
                            },
                        }
                    }

                    div {
                        id: "contact-us-popup-contact-email",
                        class: "flex flex-col w-full items-start gap-[2px]",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.contact_email}" }
                        div { class: "flex flex-row w-full gap-[2px]",
                            input {
                                class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                                placeholder: "{tr.district_placeholder}",
                                value: contact_email_signal(),
                                oninput: move |e| {
                                    let value = e.value();
                                    contact_email_signal.set(value);
                                },
                            }
                        }
                    }

                    div {
                        id: "contact-us-popup-stance",
                        class: "flex flex-col w-full gap-[2px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]",
                                "{tr.stance_on_crypto}"
                            }
                        }
                        Dropdown {
                            // TODO: replace this data to CryptoStance
                            items: CryptoStance::iter()
                                .map(|stance| match stance {
                                    CryptoStance::Supportive => tr.supportive.to_string(),
                                    CryptoStance::Against => tr.against.to_string(),
                                    CryptoStance::Neutral => tr.neutral.to_string(),
                                    CryptoStance::NoStance => tr.no_stance.to_string(),
                                })
                                .collect(),

                            value: stance_signal(),
                            placeholder: "{tr.stance_placeholder}",
                            onselect: move |value| {
                                stance_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                    }

                    div {
                        id: "contact-us-popup-agree",
                        class: "flex flex-row gap-[6px] items-center",
                        Checkbox {
                            class: "cursor-pointer",
                            title: "{tr.agree_contact_us}",
                            onchange: move |check| {
                                agreed.set(check);
                            },
                        }
                    }
                }

                div { id: "contact-us-popup-button", class: "flex w-full",
                    button {
                        class: "w-full h-[57px] text-[{theme.primary05}] bg-[{theme.primary03}] text-[18px] font-extrabold leading-[24px] rounded-[12px]",
                        style: if agreed() { "opacity: 0.5; cursor: pointer;" } else { "opacity: 0.2;" },
                        onclick: move |_| {
                            tracing::debug!("email verification clicked");
                            if !agreed() {
                                return;
                            }
                        },
                        disabled: !agreed(),
                        "{tr.verify_contact_email}"
                    }
                }
            }
        }
    }
}