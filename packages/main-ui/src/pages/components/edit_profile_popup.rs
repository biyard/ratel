#![allow(unused)]
use crate::{dioxus_elements::FileEngine, services::backend_api::BackendApi};
use bdk::prelude::*;
use dto::by_components::rich_texts::RichText;
use std::sync::Arc;

use crate::components::upload_image::UploadImage;

#[cfg(feature = "web")]
use crate::utils::file::handle_file_upload;

#[component]
pub fn EditProfilePopup(
    lang: Language,
    profile: String,
    nickname: String,
    description: String,

    onedit: EventHandler<(String, String, String)>,
) -> Element {
    let api: BackendApi = use_context();
    let tr: EditProfilePopupTranslate = translate(&lang);

    let mut profile = use_signal(|| profile);
    let mut nickname = use_signal(|| nickname);
    let mut description = use_signal(|| description);

    let onclick = move |_| {
        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;
            let input = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("edit-profile")
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
                        id: "edit-profile",
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
                                id: "edit-profile",
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
                    div { class: "font-semibold text-white text-sm/16", {tr.nickname} }
                    input {
                        class: "bg-black text-neutral-400 placeholder-neutral-600 focus:outline-none w-full font-medium text-sm/16 p-10 rounded-lg",
                        r#type: "text",
                        placeholder: tr.input_nickname_hint,
                        value: nickname(),
                        onchange: move |e| {
                            nickname.set(e.value());
                        },
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-white text-sm/16", {tr.description} }
                    div { class: "rounded-lg w-full h-fit justify-start items-start border border-neutral-600",
                        RichText {
                            id: "edit profile description",
                            content: description(),
                            onchange: move |v| {
                                description.set(v);
                            },
                            change_location: true,
                            remove_border: true,
                            placeholder: tr.input_description_hint,
                        }
                    }
                }

                div {
                    class: "cursor-pointer flex flex-row w-full justify-center items-center py-15 bg-primary rounded-[10px] font-bold text-[#000203] text-sm/19",
                    onclick: move |_| {
                        onedit.call((profile(), nickname(), description()));
                    },
                    {tr.edit_profile}
                }
            }
        }
    }
}

translate! {
    EditProfilePopupTranslate;

    nickname: {
        ko: "Nickname",
        en: "Nickname"
    }
    input_nickname_hint: {
        ko: "Input nickname",
        en: "Input nickname"
    }
    description: {
        ko: "Description",
        en: "Description"
    }
    input_description_hint: {
        ko: "Input description",
        en: "Input description"
    }
    edit_profile: {
        ko: "Edit Profile",
        en: "Edit Profile"
    }
}
