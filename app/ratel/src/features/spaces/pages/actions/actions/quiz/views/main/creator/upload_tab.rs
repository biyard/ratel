use super::*;
use crate::common::components::SpaceCard;
use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::common::types::extract_filename_from_url;
use crate::common::utils::time::time_ago;
use crate::features::spaces::hooks::use_user;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

const DEFAULT_PROFILE_URL: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

fn file_icon(ext: &FileExtension) -> Element {
    rsx! {
        FileExtensionIcon { ext: ext.clone(), size: 36 }
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
    let uploader_name = user().unwrap_or_default().display_name;
    let uploader_profile_url = match user()
        .map(|u| u.profile_url)
        .filter(|s| !s.trim().is_empty())
    {
        Some(url) => url,
        None => DEFAULT_PROFILE_URL.to_string(),
    };
    let upload_uploader_name = uploader_name.clone();
    let upload_uploader_profile_url = uploader_profile_url.clone();
    let mut query = use_query_store();

    let save_files = move |next_files: Vec<File>| async move {
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
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            if can_edit {
                FileUploader {
                    accept: ".pdf,.docx,.pptx,.xlsx,.png,.jpg,.jpeg,.mp4,.mov",
                    on_upload_success: move |_url: String| {},
                    on_upload_meta: move |uploaded: UploadedFileMeta| {
                        let upload_uploader_name = upload_uploader_name.clone();
                        let upload_uploader_profile_url = upload_uploader_profile_url.clone();
                        async move {
                            let UploadedFileMeta { url, name, size } = uploaded;
                            let uploaded_name = if name.trim().is_empty() {
                                extract_filename_from_url(&url)
                            } else {
                                name
                            };
                            let ext = FileExtension::from_name_or_url(&uploaded_name, &url);
                            if let Err(e) = crate::features::spaces::pages::apps::apps::file::create_file_link(
                                    space_id(),
                                    crate::features::spaces::pages::apps::apps::file::CreateFileLinkRequest {
                                        file_url: url.clone(),
                                        file_name: Some(uploaded_name.clone()),
                                        link_target: crate::features::spaces::pages::apps::apps::file::FileLinkTarget::Quiz(
                                            quiz_id().to_string(),
                                        ),
                                    },
                                )
                                .await
                            {
                                error!("Failed to create file link: {:?}", e);
                                toast.error(e);
                                return;
                            }
                            files
                                .write()
                                .push(File {
                                    id: url.clone(),
                                    name: uploaded_name,
                                    size,
                                    ext,
                                    url: Some(url),
                                    uploader_name: Some(upload_uploader_name.clone()),
                                    uploader_profile_url: Some(upload_uploader_profile_url.clone()),
                                    uploaded_at: Some(crate::common::utils::time::now()),
                                });
                            save_files(files()).await;
                        }
                    },
                    div { class: "flex flex-col gap-5 justify-center items-center py-2.5 px-4 w-full text-center border border-dashed transition-colors rounded-[12px] border-quiz-upload-zone-border bg-quiz-upload-zone-bg hover:border-primary",
                        div { class: "flex flex-col gap-1 justify-center items-center w-full",
                            icons::ratel::Cloud {
                                width: "64",
                                height: "64",
                                class: "text-quiz-upload-meta [&>path]:stroke-current [&>path]:fill-none",
                            }
                            div { class: "font-bold text-[15px]/[18px] text-text-primary",
                                {tr.upload_drop_title}
                            }
                        }
                        div { class: "flex flex-col gap-2.5 justify-center items-center w-full",
                            div { class: "inline-flex gap-2 justify-center items-center px-5 h-11 rounded-full border transition-opacity hover:opacity-90 min-w-[118px] border-quiz-upload-cta-bg bg-quiz-upload-cta-bg text-quiz-upload-cta-text",
                                icons::upload_download::Upload2 {
                                    width: "20",
                                    height: "20",
                                    class: "shrink-0 [&>path]:stroke-quiz-upload-cta-icon",
                                }
                                span { class: "font-bold text-[14px]/[16px] text-quiz-upload-cta-text",
                                    {tr.upload_cta}
                                }
                            }
                            p { class: "font-medium text-[13px]/[20px] text-quiz-upload-helper",
                                {tr.upload_supported_types}
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-2.5",
                if files().is_empty() {
                    div { class: "flex justify-center items-center px-6 text-center border min-h-[96px] rounded-[12px] border-quiz-upload-card-border bg-quiz-upload-card-bg",
                        p { class: "font-medium text-[15px]/[22px] text-quiz-upload-meta",
                            {tr.upload_empty}
                        }
                    }
                }
                for file in files().iter() {
                    FileItem {
                        key: "{file.id}",
                        file: file.clone(),
                        can_edit,
                        opened_menu,
                        files,
                        space_id,
                        quiz_id,
                        fallback_profile_url: uploader_profile_url.clone(),
                        fallback_uploader_name: uploader_name.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn FileItem(
    file: ReadSignal<File>,
    can_edit: bool,
    mut opened_menu: Signal<Option<String>>,
    mut files: Signal<Vec<File>>,
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    fallback_profile_url: String,
    fallback_uploader_name: String,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();

    let (file_id, file_name, file_ext, has_url, profile_url, display_name, uploaded_at) = {
        let f = file.read();
        (
            f.id.clone(),
            f.name.clone(),
            f.ext.clone(),
            f.url.is_some(),
            f.uploader_profile_url
                .clone()
                .unwrap_or(fallback_profile_url),
            f.uploader_name.clone().unwrap_or(fallback_uploader_name),
            f.uploaded_at
                .map(time_ago)
                .unwrap_or_else(|| "just now".to_string()),
        )
    };
    let is_menu_open = opened_menu().as_ref() == Some(&file_id);

    rsx! {
        SpaceCard { class: "overflow-visible relative !h-auto !rounded-[12px] !border !border-quiz-upload-card-border !bg-quiz-upload-card-bg !px-5 !py-4",
            div { class: "flex gap-4 justify-between items-center",
                div { class: "flex gap-5 items-center min-w-0",
                    div { class: "shrink-0 [&>svg]:size-10", {file_icon(&file_ext)} }
                    div { class: "flex flex-col flex-1 gap-1 min-w-0",
                        p { class: "font-bold text-white truncate text-[15px]/[20px] tracking-[0.5px] light:text-text-primary",
                            "{file_name}"
                        }
                        div { class: "flex gap-2.5 items-center min-w-0 font-medium text-quiz-upload-meta",
                            img {
                                class: "object-cover rounded-full size-6 shrink-0",
                                src: "{profile_url}",
                                alt: "Profile",
                            }
                            span { class: "text-white truncate text-[13px]/[20px] light:text-text-primary",
                                "{display_name}"
                            }
                            span { class: "shrink-0 text-[12px]/[16px] text-quiz-upload-meta",
                                "{uploaded_at}"
                            }
                        }
                    }
                }

                div { class: "flex gap-2 items-center shrink-0",
                    if has_url {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Rounded,
                            class: "py-2 px-4 rounded-full border-white bg-quiz-upload-view-bg !text-quiz-upload-view-text hover:!bg-quiz-upload-view-bg/90",
                            onclick: move |_| {
                                #[cfg(not(feature = "server"))]
                                if let Some(url) = file.read().url.as_ref() {
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
                            class: "p-1 rounded-full transition-colors focus:outline-none hover:bg-hover".to_string(),
                            onclick: move |_| {
                                let id = file.read().id.clone();
                                if opened_menu().as_ref() == Some(&id) {
                                    opened_menu.set(None);
                                } else {
                                    opened_menu.set(Some(id));
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
                            let file_url = file.read().id.clone();
                            async move {
                                let prev = files();
                                files.write().retain(|f| f.id != file_url);
                                opened_menu.set(None);
                                if let Err(e) = remove_quiz_file(
                                        space_id(),
                                        quiz_id(),
                                        RemoveQuizFileRequest { file_url },
                                    )
                                    .await
                                {
                                    error!("Failed to remove quiz file: {:?}", e);
                                    files.set(prev);
                                    toast.error(e);
                                }
                            }
                        },
                        span { class: "inline-flex items-center text-red-400", {tr.upload_delete} }
                    }
                }
            }
        }
    }
}
