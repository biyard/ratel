#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::{
    theme::Theme,
    components::dropdown::Dropdown,
};
use super::i18n::FilterPopupTranslate;
use dto::CryptoStance;

#[component]
pub fn FilterPopup(
    #[props(default = "politician_status_filter_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<FilterPopupTranslate>(&lang);
    let mut popup: PopupService = use_context();

    let mut name_signal: Signal<String> = use_signal(|| "".to_string());
    let mut stance_signal: Signal<String> = use_signal(|| "".to_string());
    let mut party_signal: Signal<String> = use_signal(|| "".to_string());
    let mut city_signal: Signal<String> = use_signal(|| "".to_string());
    let mut district_signal: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[10px] pt-[10px]",

                div {
                    id: "filter-popup-name",
                    class: "flex flex-col w-full gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.name}" }
                    }
                    input {
                        class: "w-full h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] placeholder-[{theme.primary07}] rounded-[8px]",
                        placeholder: "{tr.name_placeholder}",
                        value: name_signal(),
                        oninput: move |e| {
                            let value = e.value();
                            name_signal.set(value);
                        },
                    }
                }

                div {
                    id: "filter-popup-stance",
                    class: "flex flex-col w-full gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.stance_on_crypto}" }
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
                    id: "filter-popup-party",
                    class: "flex flex-col w-full gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.party}" }
                    }
                    Dropdown {
                        // TODO: replace dummy data
                        items: vec!["DEMOCRATIC".to_string(), "POWER POWER".to_string(), "etc".to_string()],
                        value: party_signal(),
                        placeholder: "{tr.party_placeholder}",
                        onselect: move |value| {
                            party_signal.set(value);
                        },
                        bg_color: theme.background.clone(),
                    }
                }

                // DISTRICT
                div {
                    id: "filter-popup-district",
                    class: "flex flex-col w-full items-start gap-[2px]",
                    span { class: "text-[14px] font-bold leading-[24px]", "{tr.district}" }
                    div { class: "flex flex-row w-full gap-[2px]",
                        Dropdown {
                            // TODO: replace dummy data
                            items: vec!["Seoul".to_string(), "Busan".to_string(), "Incheon".to_string()],
                            value: city_signal(),
                            placeholder: "{tr.city_placeholder}",
                            onselect: move |value| {
                                city_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                        Dropdown {
                            // TODO: replace dummy data
                            items: vec![
                                "Gangnam-Gu".to_string(),
                                "Gangdong-Gu".to_string(),
                                "Gangbuk-Gu".to_string(),
                                "Gangseo-Gu".to_string(),
                            ],
                            value: district_signal(),
                            placeholder: "{tr.district_placeholder}",
                            onselect: move |value| {
                                district_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                    }
                }

                div {
                    id: "filter-popup-buttons",
                    class: "flex flex-row w-full gap-[30px] pt-[25px]",
                    button {
                        class: "w-full h-[57px] rounded-[12px] bg-[{theme.primary03}] text-[{theme.primary05}] font-extrabold text-[18px] leading-[24px] tracking-[0.005em]",
                        onclick: move |_| {
                            name_signal.set("".to_string());
                            stance_signal.set("".to_string());
                            party_signal.set("".to_string());
                            city_signal.set("".to_string());
                            district_signal.set("".to_string());
                        },
                        "{tr.clear}"
                    }
                    button {
                        class: "w-full h-[57px] rounded-[12px] bg-[{theme.primary100}] text-white font-extrabold text-[18px] leading-[24px] tracking-[0.005em]",
                        onclick: move |_| {
                            popup.close();
                        },
                        "{tr.search}"
                    }
                }
            }
        }
    }
}