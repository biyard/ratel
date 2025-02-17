#![allow(non_snake_case)]

use crate::{components::icons::BigRightArrow, pages::patrons::_id::controller::Controller};

use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronsLoginPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;

    rsx! {
        div {
            LoginSupportUs { lang }
            LoginProposeANewFeature { lang }
            LoginRankingBoard { lang }
        }
    }
}

#[component]
pub fn LoginSupportUs(lang: Language) -> Element {
    let tr: LoginSupportUsTranslate = translate(&lang);
    // let mut popup: PopupService = use_context();
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "w-full min-h-[30px] flex flex-col justify-start items-start",
            div { class: "grow shrink basis-0 text-white text-xl font-semibold font-['Inter']",
                "{tr.title}"
            }
            div { class: "self-stretch border-b border-[#414462] justify-center items-center gap-2.5" }
            div { class: "h-[15px] self-stretch text-[#414462] text-xs font-normal font-['Inter']",
                "{tr.sub_text}"
            }
            div { class: "w-full flex justify-center min-w-md pt-[20px] text-[16px] leading-[24px]",
                // Input Box
                input {
                    class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                    r#type: "text",
                    style: " background-color: #212231; color: white",
                    placeholder: "{tr.box_text}",
                }
                //Drop Box
                div { class: "ml-[5px] min-w-[103px] h-[44px] bg-[#212231] text-[#414462] rounded-md p-2",
                    select {
                        class: "w-full h-full bg-[#212231] border-gray-500 rounded-md",
                        onchange: move |_| {},
                        option { value: "TOKEN", "TOKEN" }
                        option { value: "ETH", "ETH" }
                        option { value: "BTC", "BTC" }
                        option { value: "USDT", "USDT" }
                    }
                }
                //Button
                div {
                    class: "ml-[30px] flex justify-center items-center min-w-[400px] h-[44px] rounded-[8px] align-middle",
                    style: "background-color: #B5AB65",
                    button { onclick: move |_event| { print!("button clicked") },
                        div {
                            class: " text-[18px] font-extrabold",
                            style: "color: #ffffff80;",
                            "{tr.button_text}"
                        }
                    }
                }
            }
        }
        //Check Box
        label {
            div { class: "text-[16px] py-[10px] flex flex-row gap-[6px]",
                input {
                    class: "w-[21px] h-[21px] border-[#212231] rounded-md",
                    style: "rounded-md",
                    r#type: "checkbox",
                    checked: "{checked}",
                    onchange: move |_| checked.set(!checked()),
                }
                " If you agree to the above terms and conditions, click \"Agree and Continue\" to finalize your support."
                div {
                    button {
                        onclick: move |_event| { print!("button clicked") },
                        class: "w-[84px] h-[28px] font-semibold text-[14px]",
                        style: "background-color: #292B3C",
                        "{tr.check_box_text}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn LoginProposeANewFeature(lang: Language) -> Element {
    let tr: LoginProposeANewFeatureTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-h-[30px] flex flex-col justify-start items-start py-[30px]",
            div { class: "grow shrink basis-0 text-white text-xl font-semibold font-['Inter']",
                "{tr.title}"
            }
            div { class: "self-stretch border-b border-[#414462] justify-center items-center gap-2.5" }
            div { class: "min-h-[30px] self-stretch text-[#414462] text-xs font-normal font-['Inter']",
                "{tr.sub_text}"
            }
            button {
                class: "w-full h-[100px] mt-[10px] rounded-xl border border-dotted border-[#414462] flex flex-col justify-center items-center overflow-hidden",
                onclick: move |_| {
                    tracing::debug!("Propose a New Feature button clicked");
                },
                // TODO: If Propose a New Feature button clicked, send to new feature page.
                div { class: "text-[#414462] text-sm font-normal font-['Inter']", "{tr.input_box_text}" }
            }
        }
    }
}

#[component]
pub fn LoginRankingBoard(lang: Language) -> Element {
    let tr: LoginRankingBoardTranslate = translate(&lang);

    rsx! {
        button {
            class: "flex w-full py-[10px]",
            onclick: move |_| {
                tracing::debug!(
                    "w-full h-[100px] px-7 flex justify-between items-center bg-[#404760] rounded-xl border border-[#414462]"
                );
            },
            // TODO: If button clicked, go to Ranking board function need.
            div { class: "w-full h-[100px] px-7 flex justify-between items-center bg-[#404760] rounded-xl border border-[#414462]",
                div { class: "gap-[10px] flex flex-col justify-start items-start",
                    div { class: "text-white text-lg font-bold font-['Inter']", "{tr.title}" }
                    div { class: "text-[#b8b8b8] text-xs font-normal font-['Inter']",
                        "{tr.sub_text}"
                    }
                }
                BigRightArrow {}
            }
        }
    }
}
