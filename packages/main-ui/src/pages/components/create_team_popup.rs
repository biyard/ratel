#![allow(unused)]
use crate::{dioxus_elements::FileEngine, services::backend_api::BackendApi};
use bdk::prelude::*;

use crate::components::upload_image::UploadImage;

#[cfg(feature = "web")]
use crate::utils::file::handle_file_upload;

#[component]
pub fn CreateTeamPopup(lang: Language, oncreate: EventHandler<(String, String)>) -> Element {
    let api: BackendApi = use_context();
    let mut profile = use_signal(|| "".to_string());
    let mut username = use_signal(|| "".to_string());

    let onclick = move |_| {
        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;
            let input = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("create-team")
                .unwrap();
            input
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .click();
        }
    };

    rsx! {
        div { class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "flex flex-col w-400 max-mobile:!w-full gap-20",
                div { class: "flex flex-row w-full justify-center items-center",
                    UploadImage {
                        id: "create-team",
                        onupload: move |ev: FormEvent| async move {
                            spawn(async move {
                                #[cfg(feature = "web")]
                                if let Some(file_engine) = ev.files() {
                                    let result = handle_file_upload(file_engine, api).await;
                                    tracing::debug!("file upload results: {:?}", result);
                                    if !result.is_empty() && result[0].clone().url.is_some() {
                                        profile.set(result[0].clone().url.unwrap());
                                    }
                                }
                            });
                        },
                        if !profile().is_empty() {
                            img {
                                id: "create-team",
                                onclick,
                                class: "cursor-pointer w-50 h-50 object-cover rounded-full",
                                src: profile(),
                            }
                        } else {
                            div {
                                id: "edit-profile",
                                onclick,
                                class: "cursor-pointer w-50 h-50 rounded-full bg-neutral-400",
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-white text-sm/16", "Username" }
                    input {
                        class: "bg-black text-neutral-400 placeholder-neutral-600 focus:outline-none w-full font-medium text-sm/16 p-10 rounded-lg",
                        r#type: "text",
                        placeholder: "Input username",
                        value: username(),
                        onchange: move |e| {
                            username.set(e.value());
                        },
                    }
                }

                div {
                    class: "cursor-pointer flex flex-row w-full justify-center items-center py-15 bg-primary rounded-[10px] font-bold text-[#000203] text-sm/19",
                    onclick: move |_| {
                        oncreate.call((profile(), username()));
                    },
                    "Create Team"
                }
            }
        }
    }
}
