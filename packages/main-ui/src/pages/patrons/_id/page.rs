#![allow(non_snake_case)]
use crate::components::icons::CloseBlankSmall;
use crate::components::icons::FileUpload;
use crate::components::icons::PPTXFile;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

// TODO: Check this page is new_feature page.
#[component]
pub fn PatronsByIdPage(id: String, lang: Language) -> Element {
    let mut _ctrl: Controller = Controller::new()?;

    rsx! {
        div {
            ProposeANewFeatureInputBox { lang }
            FeatureNameInputBox { lang }
            ReferenceWebsitesInputBox { lang }
            DetailedFeatureDescriptionInputBox { lang }
            AdditionalReferenceMaterialsBox { lang }
            UploadedFileBox { lang }
            SubmitAndCancelButton { lang }
        }
    }
}

#[component]
pub fn ProposeANewFeatureInputBox(lang: Language) -> Element {
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
        }
    }
}

#[component]
pub fn FeatureNameInputBox(lang: Language) -> Element {
    let tr: FeatureNameTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[20px] text-[16px] leading-[24px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.title}"
                div { class: "text-red-600", "*" }
            }
            // Input Box
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "{tr.box_text}",
            }
        }
    }
}

#[component]
pub fn ReferenceWebsitesInputBox(lang: Language) -> Element {
    let tr: ReferenceWebsitesTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[20px] text-[16px] leading-[24px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.title}"
            }
            // Input Box
            input {
                class: "w-full px-[20px] py-[10px] rounded-lg focus:outline-none focus:ring-2 gap-[10px] placeholder-[#404761]",
                r#type: "text",
                style: " background-color: #212231; color: white",
                placeholder: "{tr.box_text}",
            }
        }
    }
}

#[component]
pub fn DetailedFeatureDescriptionInputBox(lang: Language) -> Element {
    let tr: DetailedFeatureDescriptionTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-w-md pt-[20px] text-[16px] leading-[24px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1",
                "{tr.title}"
            }
            // Input Box
            textarea {
                class: "w-full h-[325px] flex justify-start items-start px-[20px] py-[10px] text-[16px] p-3 rounded-md focus:outline-none focus:ring-2 placeholder-[#404761]",
                style: " background-color: #212231; color: white",
                placeholder: "{tr.box_text}",
            }
        }
    }
}

#[component]
pub fn AdditionalReferenceMaterialsBox(lang: Language) -> Element {
    let tr: AdditionalReferenceMaterialsTranslate = translate(&lang);

    rsx! {
        div { class: "h-[136px] rounded-[12px] w-full mt-[10px]",
            // Label
            label { class: "w-full font-bold text-white mb-1 flex items-center gap-1 text-[16px]",
                "{tr.title}"
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
                onclick: move |_event| {
                    tracing::debug!("Cancel button");
                },
                // TODO: If 'SELECT FILE FROM PC' button clicked, file upload browser open function need.
                CloseBlankSmall {}
            }
        }
    }
}

#[component]
pub fn SubmitAndCancelButton(lang: Language) -> Element {
    let tr: SubmitAndCancelButtonTextTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-center gap-[30px] mt-[50px]",
            div {
                class: "flex justify-center items-center w-[400px] h-[57px] rounded-[12px] align-middle",
                style: "background-color: #74789E",
                button {
                    onclick: move |_event| {
                        tracing::debug!("Cancel button");
                    },
                    // TODO: If 'CANCEL' button clicked, move to Patronage page function need.
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
                    onclick: move |_| {
                        tracing::debug!("Submit button");
                    },
                    // TODO: If 'SUBMIT' button clicked, move to Patronage page function need.
                    div { class: "font-bold text-[18px]", "{tr.submit_button_text}" }
                }
            }
        }
    }
}
