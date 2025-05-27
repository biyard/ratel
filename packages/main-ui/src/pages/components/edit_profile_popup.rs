#![allow(unused)]
use crate::dioxus_elements::FileEngine;
use crate::services::backend_api::BackendApi;
use bdk::prelude::*;
use dto::by_components::rich_texts::RichText;
use dto::{AssetPresignedUrisReadAction, File, FileExtension, FileType};
use std::str::FromStr;
use std::sync::Arc;

use crate::components::upload_image::UploadImage;

#[allow(dead_code)]
fn human_readable_size(bytes: usize) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < sizes.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, sizes[index])
}

#[cfg(feature = "web")]
pub async fn handle_file_upload(file_engine: Arc<dyn FileEngine>, api: BackendApi) -> Vec<File> {
    let mut result: Vec<File> = vec![];
    let files = file_engine.files();

    for f in files {
        match file_engine.read_file(f.as_str()).await {
            Some(bytes) => {
                let file_name: String = f.into();
                let file_name_copy = file_name.clone();
                let ext = file_name.rsplitn(2, '.').nth(0).unwrap_or("").to_string();

                let file_type = FileType::from_str(&ext.clone());
                let file_type = if file_type.is_ok() {
                    Some(file_type.unwrap())
                } else {
                    None
                };

                let req = AssetPresignedUrisReadAction {
                    action: None,
                    total_count: None,
                    file_type,
                };

                let extension = FileExtension::from_str(&ext);

                match extension {
                    Ok(ext) => {
                        let url = match api.upload_metadata(bytes.clone(), req).await {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        };

                        result.push(File {
                            name: file_name,
                            size: human_readable_size(bytes.len()),
                            ext,
                            url,
                        });
                    }
                    Err(_) => {
                        tracing::error!("Not Allowed file extension {}", ext);
                        continue;
                    }
                }
            }
            None => {
                tracing::error!("Error reading file");
                continue;
            }
        };
    }
    result
}

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
