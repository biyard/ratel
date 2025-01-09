#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_elements::iframe::name;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use crate::theme::Theme;
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
    let popup: PopupService = use_context();
    let tr = translate::<PoliticianStanceTranslate>(&lang);

    let mut name_signal: Signal<String> = use_signal(|| "".to_string());
    let mut stance_signal: Signal<String> = use_signal(|| "".to_string());
    let mut party_signal: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut district_signal: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { id, class,
            div { class: "flex flex-col w-full items-start justify-start gap-[10px] pt-[10px]",

                // NAME
                div { class: "flex flex-col gap-[2px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "{tr.name}" }
                    }
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
        }
    }
}