#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use crate::{
    theme::Theme,
    components::dropdown::Dropdown,
};
use dto::ServiceError;
use super::i18n::PoliticianStanceTranslate;

#[component]
pub fn FilterPopup(
    #[props(default = "politician_status_filter_popup".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    lang: Language,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr = translate::<PoliticianStanceTranslate>(&lang);

    let mut name_signal: Signal<String> = use_signal(|| "".to_string());
    let mut stance_signal: Signal<String> = use_signal(|| "".to_string());
    let mut party_signal: Signal<String> = use_signal(|| "".to_string());
    let mut city_signal: Signal<String> = use_signal(|| "".to_string());
    let mut district_signal: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[10px] pt-[10px]",

                // NAME
                div { class: "flex flex-col w-full gap-[2px]",
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

                // STANCE ON CRYPTO
                div { class: "flex flex-col w-full gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.stance_on_crypto}" }
                    }
                    Dropdown {
                        items: vec!["찬성".to_string(), "반대".to_string(), "중립".to_string(), "무단".to_string()],
                        placeholder: "{tr.stance_placeholder}",
                        value: None,
                        onclick: move |value| {
                            stance_signal.set(value);
                        },
                        bg_color: theme.background.clone(),
                    }
                }

                // PARTY
                div { class: "flex flex-col w-full gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.party}" }
                    }
                    Dropdown {
                        items: vec!["더불어민주당".to_string(), "미래통합당".to_string(), "정의당".to_string(), "국민의당".to_string(), "기타".to_string()],
                        placeholder: "{tr.party_placeholder}",
                        value: None,
                        onclick: move |value| {
                            party_signal.set(value);
                        },
                        bg_color: theme.background.clone(),
                    }
                }

                // DISTRICT
                div { class: "flex flex-col w-full items-start gap-[2px]",
                    span { class: "text-[14px] font-bold leading-[24px]", "{tr.district}" }
                    div { class: "flex flex-row w-full gap-[2px]",
                        Dropdown {
                            items: vec!["서울특별시".to_string(), "부산광역시".to_string(), "대구광역시".to_string(), "인천광역시".to_string(), "광주광역시".to_string(), "대전광역시".to_string(), "울산광역시".to_string(), "세종특별자치시".to_string(), "경기도".to_string(), "강원도".to_string(), "충청북도".to_string(), "충청남도".to_string(), "전라북도".to_string(), "전라남도".to_string(), "경상북도".to_string(), "경상남도".to_string(), "제주특별자치도".to_string()],
                            placeholder: "{tr.city_placeholder}",
                            value: None,
                            onclick: move |value| {
                                city_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                        Dropdown {
                            items: vec!["서울특별시".to_string(), "부산광역시".to_string(), "대구광역시".to_string(), "인천광역시".to_string(), "광주광역시".to_string(), "대전광역시".to_string(), "울산광역시".to_string(), "세종특별자치시".to_string(), "경기도".to_string(), "강원도".to_string(), "충청북도".to_string(), "충청남도".to_string(), "전라북도".to_string(), "전라남도".to_string(), "경상북도".to_string(), "경상남도".to_string(), "제주특별자치도".to_string()],
                            placeholder: "{tr.district_placeholder}",
                            value: None,
                            onclick: move |value| {
                                district_signal.set(value);
                            },
                            bg_color: theme.background.clone(),
                        }
                    }
                }
            }
        }
    }
}