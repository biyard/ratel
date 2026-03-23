use super::*;
use crate::common::components::SpaceCard;
use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::common::types::extract_filename_from_url;
use crate::common::utils::time::time_ago;
use crate::features::spaces::hooks::use_user;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

const DEFAULT_PROFILE_URL: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

fn file_icon(ext: &FileExtension) -> Element {
    match ext {
        FileExtension::JPG => rsx! {
            icons::files::Jpg { width: "36", height: "36" }
        },
        FileExtension::PNG => rsx! {
            icons::files::Png { width: "36", height: "36" }
        },
        FileExtension::PDF => rsx! {
            icons::files::Pdf { width: "36", height: "36" }
        },
        FileExtension::ZIP => rsx! {
            icons::files::Zip { width: "36", height: "36" }
        },
        FileExtension::WORD => rsx! {
            icons::files::Docx { width: "36", height: "36" }
        },
        FileExtension::PPTX => rsx! {
            icons::files::Pptx { width: "36", height: "36" }
        },
        FileExtension::EXCEL => rsx! {
            icons::files::Xlsx { width: "36", height: "36" }
        },
        FileExtension::MP4 => rsx! {
            icons::files::Mp4 { width: "36", height: "36" }
        },
        FileExtension::MOV => rsx! {
            icons::files::Mov { width: "36", height: "36" }
        },
        FileExtension::MKV => rsx! {
            icons::file::File {
                width: "36",
                height: "36",
                class: "[&>path]:stroke-current text-card-meta",
            }
        },
    }
}

#[component]
pub fn UploadTab(can_edit: bool) -> Element {
    let ctx = use_space_quiz_context();
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let user = use_user()?;
    let mut files = use_signal(|| ctx.quiz.read().files.clone());
    let mut opened_menu = use_signal(|| Option::<String>::None);
    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let uploader_name = user
        .read()
        .as_ref()
        .map(|user| user.display_name.clone())
        .unwrap_or_else(|| "".to_string());
    let uploader_profile_url = user
        .read()
        .as_ref()
        .map(|user| {
            if user.profile_url.trim().is_empty() {
                DEFAULT_PROFILE_URL.to_string()
            } else {
                user.profile_url.clone()
            }
        })
        .unwrap_or_else(|| DEFAULT_PROFILE_URL.to_string());
    let upload_uploader_name = uploader_name.clone();
    let upload_uploader_profile_url = uploader_profile_url.clone();
    let mut query = use_query_store();

    let save_files = move |next_files: Vec<File>| {
        let mut toast = toast;
        spawn(async move {
            let req = UpdateQuizRequest {
                files: Some(next_files),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update quiz files: {:?}", err);
                toast.error(err);
            } else {
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                query.invalidate(&keys);
            }
        });
    };

    rsx! {
        div { class: "flex w-full flex-col gap-4",
            if can_edit {
                FileUploader {
                    accept: ".pdf,.docx,.pptx,.xlsx,.png,.jpg,.jpeg,.mp4,.mov",
                    on_upload_success: move |_url: String| {},
                    on_upload_meta: move |uploaded: UploadedFileMeta| {
                        let UploadedFileMeta { url, name, size } = uploaded;
                        let uploaded_name = if name.trim().is_empty() {
                            extract_filename_from_url(&url)
                        } else {
                            name
                        };
                        let ext = FileExtension::from_name_or_url(&uploaded_name, &url);
                        let mut next = files();
                        next.push(File {
                            id: url.clone(),
                            name: uploaded_name,
                            size,
                            ext,
                            url: Some(url),
                            uploader_name: Some(upload_uploader_name.clone()),
                            uploader_profile_url: Some(upload_uploader_profile_url.clone()),
                            uploaded_at: Some(crate::common::utils::time::now()),
                        });
                        files.set(next.clone());
                        save_files(next);
                    },
                    div { class: "flex px-4 py-2.5 w-full gap-5 flex-col items-center justify-center rounded-[12px] border border-dashed border-quiz-upload-zone-border bg-quiz-upload-zone-bg text-center transition-colors hover:border-primary",
                        div { class: "flex flex-col w-full justify-center items-center gap-1",
                            icons::ratel::Cloud {
                                width: "64",
                                height: "64",
                                class: "text-quiz-upload-meta [&>path]:stroke-current",
                            }
                            div { class: "text-[15px]/[18px] font-bold text-text-primary",
                                {tr.upload_drop_title}
                            }
                        }
                        div { class: "flex flex-col w-full justify-center items-center gap-2.5",
                            div { class: "inline-flex h-11 min-w-[118px] items-center justify-center gap-2 rounded-full border border-white bg-white px-5 text-quiz-upload-cta-text transition-colors hover:bg-white/90",
                                icons::upload_download::Upload2 {
                                    width: "20",
                                    height: "20",
                                    class: "shrink-0 [&>path]:stroke-quiz-upload-cta-icon",
                                }
                                span { class: "text-[14px]/[16px] font-bold text-quiz-upload-cta-text",
                                    {tr.upload_cta}
                                }
                            }
                            p { class: "text-[13px]/[20px] font-medium text-quiz-upload-helper",
                                {tr.upload_supported_types}
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-2.5",
                if files().is_empty() {
                    div { class: "flex min-h-[96px] items-center justify-center rounded-[12px] border border-quiz-upload-card-border bg-quiz-upload-card-bg px-6 text-center",
                        p { class: "text-[15px]/[22px] font-medium text-quiz-upload-meta",
                            {tr.upload_empty}
                        }
                    }
                }
                for file in files().iter() {
                    {
                        let file = file.clone();
                        let file_id = file.id.clone();
                        let menu_file_id = file_id.clone();
                        let delete_file_id = file_id.clone();
                        let is_menu_open = opened_menu().as_ref() == Some(&file_id);
                        let profile_url = file
                            .uploader_profile_url
                            .clone()
                            .unwrap_or_else(|| uploader_profile_url.clone());
                        let uploader_name = file
                            .uploader_name
                            .clone()
                            .unwrap_or_else(|| uploader_name.clone());
                        let uploaded_at = file
                            .uploaded_at
                            .map(time_ago)
                            .unwrap_or_else(|| "just now".to_string());
                        rsx! {
                            SpaceCard {
                                key: "{file_id}",
                                class: "relative !h-auto !rounded-[12px] !border !border-quiz-upload-card-border !bg-quiz-upload-card-bg !px-5 !py-4 overflow-visible",
                                div { class: "flex items-center justify-between gap-4",
                                    div { class: "flex min-w-0 items-center gap-5",
                                        div { class: "shrink-0 [&>svg]:size-10", {file_icon(&file.ext)} }
                                        div { class: "flex min-w-0 flex-1 flex-col gap-1",
                                            p { class: "truncate text-[15px]/[20px] font-bold tracking-[0.5px] text-white light:text-text-primary",
                                                "{file.name}"
                                            }
                                            div { class: "flex min-w-0 items-center gap-2.5 font-medium text-quiz-upload-meta",
                                                img {
                                                    class: "size-6 rounded-full object-cover shrink-0",
                                                    src: "{profile_url}",
                                                    alt: "Profile",
                                                }
                                                span { class: "truncate text-[13px]/[20px] text-white light:text-text-primary",
                                                    "{uploader_name}"
                                                }
                                                span { class: "shrink-0 text-[12px]/[16px] text-quiz-upload-meta",
                                                    "{uploaded_at}"
                                                }
                                            }
                                        }
                                    }

                                    div { class: "flex items-center gap-2 shrink-0",
                                        if file.url.is_some() {
                                            Button {
                                                style: ButtonStyle::Outline,
                                                shape: ButtonShape::Rounded,
                                                class: "px-4 py-2 rounded-full border-white bg-quiz-upload-view-bg !text-quiz-upload-view-text hover:!bg-quiz-upload-view-bg/90",
                                                onclick: move |_| {
                                                    #[cfg(not(feature = "server"))]
                                                    if let Some(url) = &file.url {
                                                        let _ = crate::common::web_sys::window()
                                                            .and_then(|w| w.open_with_url_and_target(url, "_blank").ok());
                                                    }
                                                },
                                                span { class: "text-quiz-upload-view-text", {tr.upload_view} }
                                            }
                                        }
                                        if can_edit {
                                            Button {
                                                size: ButtonSize::Icon,
                                                style: ButtonStyle::Text,
                                                class: "p-1 rounded-full transition-colors focus:outline-none hover:bg-hover"
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    .to_string(),
                                                onclick: move |_| {
                                                    if opened_menu().as_ref() == Some(&menu_file_id) {
                                                        opened_menu.set(None);
                                                    } else {
                                                        opened_menu.set(Some(menu_file_id.clone()));
                                                    }
                                                },
                                                icons::validations::Extra { class: "size-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent [&>circle]:fill-icon-primary" }
                                            }
                                        }
                                    }
                                }

                                if can_edit && is_menu_open {
                                    div { class: "absolute right-0 top-full z-50 mt-2 w-40 rounded-md border border-divider bg-background light:bg-input-box-bg",
                                        Button {
                                            size: ButtonSize::Inline,
                                            style: ButtonStyle::Text,
                                            class: "flex items-center py-2 px-4 w-full text-sm text-red-400 cursor-pointer hover:bg-hover"
                                                .to_string(),
                                            onclick: move |_| {
                                                let mut next = files();
                                                next.retain(|f| f.id != delete_file_id);
                                                files.set(next.clone());
                                                opened_menu.set(None);
                                                save_files(next);
                                            },
                                            span { class: "inline-flex items-center text-red-400", {tr.upload_delete} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
