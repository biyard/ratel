#![allow(non_snake_case)]
pub mod i18n;

use dioxus::prelude::*;
use dioxus_translate::*;
use i18n::FooterTranslate;

#[component]
pub fn Footer(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full items-start",
            div { class: "flex flex-col justify-start items-start pb-[50px] pt-[50px] text-[15px] font-semibold align-middle gap-3",
                // logo
                // text
                div {
                    class: "text-base flex flex-col",
                    style: "color: #8588AB;",
                    "{tr.title_text}"
                }
            }
            hr { class: "w-full", style: "border-color: #424563;" }

            div { class: "w-full flex justify-between align-middle text-[15] font-bold py-[45px]",

                div { style: "color: #8588AB;", "© 2025  Biyard Corp. All rights reserved." }

                div { class: " flex gap-5", style: "color: #8588AB;",
                    a {
                        href: "https://",
                        class: "hover:pointer",
                        target: "_blank",
                        "{tr.privacy_policy_button_text}"
                    }
                    a {
                        href: "https://",
                        class: "hover:pointer",
                        target: "_blank",
                        "{tr.terms_of_service_button_text}"
                    }
                }
            }
        }
    }
}
