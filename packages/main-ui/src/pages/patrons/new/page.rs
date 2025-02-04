#![allow(non_snake_case)]

use crate::components::icons::BigRightArrow;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
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
                div { class: "text-[#414462] text-sm font-normal font-['Inter']", "{tr.input_box_text}" }
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
            div { class: "w-full h-[100px] mt-[10px] rounded-xl border border-dotted border-[#414462] flex flex-col justify-center items-center overflow-hidden",
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
