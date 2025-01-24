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
            Title { lang }
            NameInputBox { lang }
            SummaryInputBox { lang }
            LegislationInputBox { lang }
            ProposedSolutionInputBox { lang }
            DiscussionPointInputBox { lang }
        }
    }
}

#[component]
pub fn MoveBackButton(lang: Language) -> Element {
    let tr: ButtonTextTranslate = translate(&lang);

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
pub fn Title(lang: Language) -> Element {
    let tr: TitleTranslate = translate(&lang);

    rsx! {
        div { class: "text-xl font-bold", style: "color: white", "{tr.title}" }
        hr { class: "w-full mb-1", style: "border-color: #424563;" }
        div { class: "text-xs", style: "color: #424563;", "{tr.title_text}" }
    }
}

#[component]
pub fn NameInputBox(lang: Language) -> Element {
    let tr: TitleNameInputBox = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[20px] text-[16px] leading-[24px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.title_name}"
                div { class: "text-red-600", "*" }
            }
            // Input Box
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "{tr.name_text}",
            }
        }
    }
}

#[component]
pub fn SummaryInputBox(lang: Language) -> Element {
    let tr: SummaryTextInputBox = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // Label
            label { class: "w-full text-lg font-bold text-white mb-1 flex items-center gap-1",
                "{tr.summary_title_text}"
                div { class: "text-red-600", "*" }
            }
            // Input Box
            textarea {
                class: "w-full px-[20px] py-[10px] text-s font-bold p-3 rounded-md focus:outline-none focus:ring-2 h-[325px] placeholder-[#404761]",
                style: " background-color: #212231; border-color: #404761; color:white",
                placeholder: "{tr.summary_text}",
            }
        }
    }
}

#[component]
pub fn LegislationInputBox(lang: Language) -> Element {
    let tr: LegislationTextInputBox = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.legislation_title}"
            }
            // Input Box
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "{tr.legislation_text}",
            }
        }
    }
}

#[component]
pub fn ProposedSolutionInputBox(lang: Language) -> Element {
    let tr: ProposedSolutionTextInputBox = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.proposedsolution_title}"
            }
            // Input Box
            textarea {
                class: "w-full px-[20px] py-[10px] text-s font-bold p-3 rounded-md focus:outline-none focus:ring-2 h-[325px] placeholder-[#404761]",
                style: " background-color: #212231; border-color: #404761; color:white",
                placeholder: "{tr.proposedsolution_text}",
            }
        }
    }
}

#[component]
pub fn DiscussionPointInputBox(lang: Language) -> Element {
    let tr: DiscussionPointTextInputBox = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[10px] text-[16px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.discussion_point_title}"
            }
            // Input Box
            div { class: "flex justify-between w-full gap-3",
                input {
                    class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                    r#type: "text",
                    style: " background-color: #212231; color: white",
                    placeholder: "{tr.discussion_point_text}",
                }
                // Button
                button {
                    class: "h-10 w-[60px] text-2xl font-bold flex flex-col justify-center items-center rounded-lg bg-[#B5AB65] text-white",
                    onclick: move |_| {
                        println!("More button clicked!");
                    },
                    "+"
                }
            }
        }
    }
}
