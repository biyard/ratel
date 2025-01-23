#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use crate::components::icons::LeftArrow;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn LegislationSelector(lang: Language, onclick: EventHandler<Option<String>>) -> Element {
    let mut _ctrl = Controller::new()?;
    let _tr: LegislationSelectorTranslate = translate(&lang);

    rsx! {
        div {
            MoveBackButton { lang }
        }

        div { Title {} }

        div { NameInputBox {} }

        div { SummaryInputBox {} }

        div { LegislationInputBox {} }

        div { ProposedSolutionInputBox {} }

        div { DiscussionPointInputBox {} }
    }
}

#[component]
pub fn MoveBackButton(lang: Language) -> Element {
    let tr: ButtonTextTranslate = translate(&lang);

    //버튼 속 글자 중앙 정렬 다시 봐야함
    rsx! {
        div { class: "flex justify-start items-center w-full align-middle pb-[50px]",
            button {
                onclick: move |_| {
                    println!("button clicked");
                },
                div { class: "h-5 flex items-center w-full" }
                div {
                    class: "flex items-center w-full gap-2",
                    style: "color: #404761",
                    LeftArrow {}
                    "{tr.button_text}"
                }
            }
        }
    }
}

#[component]
pub fn Title() -> Element {
    rsx! {
        div { class: "text-xl font-bold", style: "color: white",
            "Topic creation (2/2) - Create a new topic"
        }
        hr { class: "w-full mb-1", style: "border-color: #424563;" }
        div { class: "text-xs", style: "color: #424563;",
            "Create a new topic to discuss how and to what extent the legislative proposal impacts the crypto industry."
        }
    }
}

#[component]
pub fn NameInputBox() -> Element {
    rsx! {
        div { class: "w-full min-w-md pt-[20px] text-[16px] leading-[24px]",
            // 라벨
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "PROPOSAL NAME"
                div { class: "text-red-600", "*" }
            }
            // 입력 박스
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "Provide a concise and descriptive title for your proposal to highlight the focus of your suggested improvements.",
            }
        }
    }
}

#[component]
pub fn SummaryInputBox() -> Element {
    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // 라벨
            label { class: "w-full text-lg font-bold text-white mb-1 flex items-center gap-1",
                "PROPOSAL SUMMARY"
                div { class: "text-red-600", "*" }
            }
            // 입력 박스
            textarea {
                class: "w-full text-s font-bold p-3 rounded-md focus:outline-none focus:ring-2 h-[325px] placeholder-[#404761]",
                style: " background-color: #212231; border-color: #404761; color:white",
                placeholder: "Provide a brief summary of the changes or improvements you want to propose for the selected legislation. Focus on the key points and how they address current issues or enhance the bill.\n ex) \"This proposal aims to simplify the regulatory framework for crypto exchanges and reduce compliance burdens for startups.\"",
            }
        }
    }
}

#[component]
pub fn LegislationInputBox() -> Element {
    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // 라벨
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "LEGISLATION"
            }
            // 입력 박스
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "Provide related link. ex) https://google.com/...",
            }
        }
    }
}

#[component]
pub fn ProposedSolutionInputBox() -> Element {
    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // 라벨
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "PROPOSED SOLUTION(S)"
            }
            // 입력 박스
            textarea {
                class: "w-full text-s font-bold p-3 rounded-md focus:outline-none focus:ring-2 h-[325px] placeholder-[#404761]",
                style: " background-color: #212231; border-color: #404761; color:white",
                placeholder: "Outline your suggested changes or amendments. Be as specific and actionable as possible, detailing steps or measures for implementation.\n Example: \n 1.Define clear criteria for exchange registration.\n 2.Introduce a streamlined application process for startups with less than $1M in annual revenue.",
            }
        }
    }
}

#[component]
pub fn DiscussionPointInputBox() -> Element {
    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // 라벨
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "DISCUSSION POINTS"
            }
            div { class: "flex justify-between w-full gap-3",
                // 입력 박스
                input {
                    class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                    r#type: "text",
                    style: " background-color: #212231; color: white",
                    placeholder: "Identify specific areas or questions for discussion to foster collaboration and diverse input from participants.",
                }
                //버튼
                div { class: "h-10 w-[60px] text-2xl font-bold flex flex-col justify-center items-center rounded-lg bg-[#B5AB65]",
                    button {
                        onclick: move |_| {
                            println!("button clicked");
                        },
                        style: "color: white",
                        "+"
                    }
                }
            }
        }
    }
}
