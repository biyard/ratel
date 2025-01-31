#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use crate::components::icons::CloseBlank;
use crate::components::icons::CloseBlankSmall;
use crate::components::icons::FileUpload;
use crate::components::icons::LeftArrow;
use crate::components::icons::LogoWithBackground;
use crate::components::icons::PPTXFile;

use dioxus::prelude::*;
use dioxus_popup::PopupService;
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
            DiscussionPoint { lang }
            AdditionalResourcesBox { lang }
            UploadedFileBox { lang }
            CreateAndCancelButton { lang }
        }
    }
}

#[component]
pub fn MoveBackButton(lang: Language) -> Element {
    let tr: ButtonTextTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-start items-center w-full align-middle pb-[50px]",
            button { onclick: move |_| {},
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
pub fn DiscussionPointInputBox(lang: Language, onadd: EventHandler<String>) -> Element {
    let tr: DiscussionPointTextInputBox = translate(&lang);
    let mut contents = use_signal(|| "".to_string());

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
                    value: contents(),
                    onchange: move |e| {
                        contents.set(e.value());
                    },
                    r#type: "text",
                    style: " background-color: #212231; color: white",
                    placeholder: "{tr.discussion_point_text}",
                }
                // Button
                button {
                    class: "h-10 w-[60px] text-2xl font-bold flex flex-col justify-center items-center rounded-lg bg-[#B5AB65] text-white",
                    onclick: move |_| {
                        println!("More button clicked!");
                        onadd.call(contents());
                    },
                    "+"
                }
            }
        }
    }
}

#[component]
pub fn DiscussionPointBox(
    lang: Language,
    contents: Vec<String>,
    onremove: EventHandler<usize>,
) -> Element {
    rsx! {
        for (index , content) in contents.iter().enumerate() {
            div {
                class: "w-full flex justify-between items-center px-[20px] py-[10px] text-s font-bold p-[10px] rounded-md h-[64px] mt-[5px]",
                style: "background-color: #404761",
                div { class: "flex", "{content}" }
                button {
                    class: "rounded-full flex items-center",
                    style: "background-color: #212231 h-[30px] w-[30px]",
                    onclick: move |_| {
                        onremove.call(index);
                    },
                    CloseBlank {}
                }
            }
        }
    }
}

#[component]
pub fn DiscussionPoint(lang: Language) -> Element {
    let mut contents = use_signal(|| vec![]);

    rsx! {
        div {
            DiscussionPointInputBox {
                lang,
                onadd: move |value: String| {
                    let mut c = contents();
                    c.push(value);
                    contents.set(c);
                    tracing::debug!("contents: {:?}", contents);
                },
            }

            DiscussionPointBox {
                lang,
                contents: contents(),
                onremove: move |index: usize| {
                    contents.remove(index);
                },
            }
        }
    }
}

#[component]
pub fn AdditionalResourcesBox(lang: Language) -> Element {
    let tr: AdditionalResourcesBoxText = translate(&lang);

    rsx! {
        div { class: "h-[136px] rounded-[12px] w-full mt-[10px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1 text-[16px]",
                "{tr.title_text}"
            }
            div {
                class: "w-full flex flex-col justify-between items-center rounded-md  h-[136px] mt-[5px]",
                style: "padding: 8px; border: 1px dotted #424563;",

                //Icon
                FileUpload {}
                div { class: "font-[14px]", style: "color: #424563", "{tr.box_text}" }
                //Line
                div {
                    class: "text-[12px] w-full flex flex-row justify-center items-center gap-[10px]",
                    style: "color: #6D6D6D",
                    hr {
                        class: "w-[80px] mb-1",
                        style: "border-color: ##E7E7E7;",
                    }
                    div { "OR" }
                    hr {
                        class: "w-[80px] mb-1",
                        style: "border-color: ##E7E7E7;",
                    }
                }

                //button
                div {
                    class: "flex justify-center items-center w-[173px] h-[30px] rounded-[4px] align-middle",
                    style: "background-color: #74789E",
                    button { onclick: move |_event| { print!("button clicked") },
                        div { class: " text-[12px]", style: "color: #212231", "{tr.button_text}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UploadedFileBox(lang: Language) -> Element {
    rsx! {
        div { class: "h-[146px] rounded-[12px] w-full mt-[40px] flex justify-start items-center",
            div { class: "flex flex-col justify-start items-center gap-[10px]",
                div { class: "w-full flex flex- center justify-start items-center gap-[5px]",
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                }
                div { class: "w-full flex justify-start items-center gap-[5px]",
                    UploadedFile {}
                    UploadedFile {}
                    UploadedFile {}
                }
            }
        }
    }
}

#[component]
pub fn UploadedFile() -> Element {
    rsx! {
        div {
            class: "flex justify-center items-center w-[170px] h-[68px] rounded-[4px] p-4",
            style: "background-color: #404761",
            PPTXFile {}
            div { class: "w-full flex flex-col justify-center items-center text-[12px]",
                div { "assets.pdf" }
                div { style: "color: #6D6D6D", "5.3MB" }
            }
            button {
                class: "flex justify-center items-center",
                onclick: move |_event| { print!("button clicked") },
                CloseBlankSmall {}
            }
        }
    }
}

#[component]
pub fn CreateAndCancelButton(lang: Language) -> Element {
    let tr: CreateAndCancelButtonTextTranslate = translate(&lang);
    let mut list = use_signal(Vec::new);
    let mut is_open = use_signal(|| true);
    let mut is_loading = use_signal(|| true);
    let mut popup: PopupService = use_context();

    if is_open() && is_loading() {
        popup //loading popup
            .open(rsx! {
                div { class: "mb-[10px] flex flex-col justify-between items-center",
                    LogoWithBackground {}
                    div { class: "mt-[35px] w-[400px] text-center text-[16px] tracking-wide",
                        span { "{tr.popup_text_part1}" }
                        span { class: "text-[#B5AB65] font-bold tracking-wide",
                            "{tr.popup_text_highlight}\n"
                        }
                        span { "{tr.popup_text_part2}" }
                    }
                }
            })
            .with_id("loading_popup")
            .with_title(tr.popup_title)
            .without_close();
    } else if is_open() && !is_loading() {
        popup //open popup
            .open(rsx! {
                div { class: "flex flex-col justify-between items-center",
                    LogoWithBackground {}
                    div { class: "mt-[35px] mb-[35px] w-[400px] text-center text-[16px] tracking-wide",
                        span { "{tr.created_popup_text}" }
                    }
                    button { onclick: move |_event| { print!("button clicked") },
                        div {
                            class: "flex justify-center items-center rounded-[12px] w-[400px] h-[57px] font-extrabold text-[18px]",
                            style: "background-color: #74789E; color: #212231; ",
                            "{tr.created_button_text}"
                        }
                    }
                }
            })
            .with_id("created_popup")
            .with_title(tr.created_popup_title);
    }

    rsx! {
        div { class: "flex justify-center gap-[30px] mt-[50px]",
            div {
                class: "flex justify-center items-center w-[400px] h-[57px] rounded-[12px] align-middle",
                style: "background-color: #74789E",
                button {
                    onclick: move |_event| {
                        let list_len = list.len();
                        list.push(list_len);
                        list.push(list_len);
                    },
                    div {
                        class: "font-bold text-[18px]",
                        style: "color: #212231;",
                        "{tr.cancel_button_text}"
                    }
                }
            }
            div {
                class: "flex justify-center items-center w-[400px] h-[57px] rounded-[12px] align-middle",
                style: "background-color: #B5AB65",
                button {
                    div {
                        class: "font-bold text-[18px]",
                        onclick: move |_| {
                            is_open.set(true);
                            is_loading.set(true);
                        },
                        "{tr.create_button_text}"
                    }
                }
            }
        }
    }
}
