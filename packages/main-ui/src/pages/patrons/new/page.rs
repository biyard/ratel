#![allow(non_snake_case)]

use crate::components::icons::BigRightArrow;
#[allow(unused_imports)]
use crate::pages::wallet_popup::WalletPopup;

use super::controller::*;
use super::i18n::*;
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

#[component]
pub fn NewPatronPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;

    rsx! {
        div {
            id: "new-patron",
            class: "w-full flex flex-col justify-center gap-[30px]",
            SupportUs { lang }
            ProposeANewFeature { lang }
            RankingBoard { lang }
        }
    }
}

#[component]
pub fn SupportUs(lang: Language) -> Element {
    let tr: SupportUsTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let phantom = crate::utils::phantom::PhantomAuth::new();

    rsx! {
        div { class: "w-full min-h-[30px] flex flex-col justify-start items-start",
            div { class: "grow shrink basis-0 text-white text-xl font-semibold font-['Inter']",
                "{tr.title}"
            }
            div { class: "self-stretch border-b border-[#414462] justify-center items-center gap-2.5" }
            div { class: "h-[15px] self-stretch text-[#414462] text-xs font-normal font-['Inter']",
                "{tr.sub_text}"
            }
            div { class: "w-full h-[100px] mt-[10px] rounded-xl border border-dotted border-[#414462] flex flex-col justify-center items-center overflow-hidden",
                if let Ok(_account) = phantom.get_account() {
                    div { class: "text-[#414462] text-sm font-normal font-['Inter']" }
                } else {
                    button {
                        onclick: move |_| {
                            popup.open(rsx! {
                                WalletPopup {
                                    class: "w-[400px] h-[82px] flex flex-row my-[10px] p-[8px] bg-[#5B5E80] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer hover:bg-[#5C6BFF]",
                                    id: "wallet_popup".to_string(),
                                    lang,
                                }
                            }).with_title(tr.popup_title);
                        },
                        // TODO: If wallet login success, send to Donate Details page.
                        div { class: "text-[#414462] text-sm font-normal font-['Inter']",
                            "{tr.input_box_text}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProposeANewFeature(lang: Language) -> Element {
    let tr: ProposeANewFeatureTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-h-[30px] flex flex-col justify-start items-start",
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
pub fn RankingBoard(lang: Language) -> Element {
    let tr: RankingBoardTranslate = translate(&lang);

    rsx! {
        button {
            onclick: move |_| {
                tracing::debug!("Ranking board button");
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
