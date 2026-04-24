use crate::common::components::editor::Editor as RichEditor;
use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::common::types::extract_filename_from_url;
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::actions::quiz::views::main::creator::QuizCreatorTranslate;

fn icon_class(ext: &FileExtension) -> &'static str {
    match ext {
        FileExtension::PDF => "file-row__icon file-row__icon--pdf",
        FileExtension::JPG | FileExtension::PNG => "file-row__icon file-row__icon--img",
        FileExtension::WORD | FileExtension::EXCEL | FileExtension::PPTX => {
            "file-row__icon file-row__icon--doc"
        }
        _ => "file-row__icon",
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SaveStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
}

#[component]
pub fn ContentCard() -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut ctx = use_space_quiz_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let mut pass_score = ctx.pass_score;
    let mut retry_count = ctx.retry_count;
    let questions = ctx.questions;

    let initial_title = ctx.quiz.read().title.clone();
    let initial_description = ctx.quiz.read().description.clone();
    let initial_files = ctx.quiz.read().files.clone();
    let mut title = use_signal(|| initial_title.clone());
    let mut last_saved_title = use_signal(|| initial_title);
    let mut description = use_signal(|| initial_description.clone());
    let mut last_saved_description = use_signal(|| initial_description);
    let mut title_version = use_signal(|| 0u64);
    let mut description_version = use_signal(|| 0u64);
    let mut title_status = use_signal(|| SaveStatus::Idle);
    let mut description_status = use_signal(|| SaveStatus::Idle);
    let mut files = use_signal(|| initial_files);

    let total_questions = questions.read().len();

    let save_files_after_upload = move |next_files: Vec<File>| {
        spawn(async move {
            let req = UpdateQuizRequest {
                files: Some(next_files),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to save quiz files: {:?}", err);
                toast.error(err);
            } else {
                ctx.quiz.restart();
            }
        });
    };

    let save_scoring = move || {
        spawn(async move {
            let req = UpdateQuizRequest {
                pass_score: Some(pass_score()),
                retry_count: Some(retry_count()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to save scoring: {:?}", err);
                toast.error(err);
            } else {
                ctx.quiz.restart();
            }
        });
    };

    let mut save_title = move || {
        let current = title();
        if current == last_saved_title() {
            return;
        }
        title_status.set(SaveStatus::Saving);
        spawn(async move {
            let req = UpdateQuizRequest {
                title: Some(current.clone()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to save title: {:?}", err);
                title_status.set(SaveStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved_title.set(current);
                title_status.set(SaveStatus::Saved);
                ctx.quiz.restart();
            }
        });
    };

    let mut save_description = move || {
        let current = description();
        if current == last_saved_description() {
            return;
        }
        description_status.set(SaveStatus::Saving);
        spawn(async move {
            let req = UpdateQuizRequest {
                description: Some(current.clone()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to save description: {:?}", err);
                description_status.set(SaveStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved_description.set(current);
                description_status.set(SaveStatus::Saved);
                ctx.quiz.restart();
            }
        });
    };

    // Autosave title — 3-second debounce.
    use_effect(move || {
        let version = title_version();
        if version == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
            if title_version() != version {
                return;
            }
            if title() == last_saved_title() {
                return;
            }
            save_title();
        });
    });

    // Autosave description — 3-second debounce.
    use_effect(move || {
        let version = description_version();
        if version == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
            if description_version() != version {
                return;
            }
            if description() == last_saved_description() {
                return;
            }
            save_description();
        });
    });

    rsx! {
        section { class: "pager__page", "data-page": "0",
            article { class: "page-card", "data-testid": "page-card-content",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_1}" }
                        div {
                            h1 { class: "page-card__title", "{tr.card_content_title}" }
                            div { class: "page-card__subtitle", "{tr.card_content_subtitle}" }
                        }
                    }
                }

                section { class: "section", "data-testid": "section-content",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_content_label}" }
                        span { class: "section__hint", "{tr.section_content_hint}" }
                    }
                    div { class: "field",
                        div { style: "display:flex;align-items:center;justify-content:space-between;gap:8px",
                            label { class: "field__label", "{tr.title_label}" }
                            AutosaveStatusBadge { status: title_status() }
                        }
                        input {
                            class: "input",
                            "data-testid": "quiz-title",
                            placeholder: "{tr.title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| {
                                title.set(e.value());
                                title_status.set(SaveStatus::Unsaved);
                                title_version.set(title_version() + 1);
                            },
                            onblur: move |_| save_title(),
                        }
                    }
                    div { class: "editor",
                        RichEditor {
                            class: "[&_.re-toolbar]:border-b [&_.re-toolbar]:border-[rgba(255,255,255,0.06)] [&_.re-content]:min-h-[180px] [&_.re-content]:px-[22px] [&_.re-content]:py-[20px] [&_.re-content]:outline-none",
                            content: description(),
                            editable: true,
                            placeholder: "",
                            on_content_change: move |html: String| {
                                description.set(html.clone());
                                if html != last_saved_description() {
                                    description_status.set(SaveStatus::Unsaved);
                                    description_version.set(description_version() + 1);
                                }
                            },
                        }
                        div { class: "editor__footer",
                            AutosaveStatusBadge { status: description_status() }
                        }
                    }
                }

                // ── Attachments ─────
                section { class: "section", "data-testid": "section-attachments",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_attachments_label}" }
                        span { class: "section__hint", "{tr.section_attachments_hint}" }
                    }

                    div { "data-testid": "attachment-list",
                        for (idx, file) in files().iter().enumerate() {
                            {
                                let file_id = file.id.clone();
                                let file_name = file.name.clone();
                                let file_size = file.size.clone();
                                let file_ext = file.ext.clone();
                                let ext_label = format!("{:?}", file_ext).to_uppercase();
                                let icon_cls = icon_class(&file_ext);
                                rsx! {
                                    div { key: "file-{idx}", class: "file-row",
                                        div { class: "{icon_cls}",
                                            FileExtensionIcon { ext: file_ext.clone(), size: 14 }
                                        }
                                        div { class: "file-row__info",
                                            div { class: "file-row__name", "{file_name}" }
                                            div { class: "file-row__meta", "{ext_label} \u{00B7} {file_size}" }
                                        }
                                        button {
                                            class: "icon-btn",
                                            r#type: "button",
                                            aria_label: "{tr.remove_file}",
                                            onclick: move |_| {
                                                let file_url = file_id.clone();
                                                async move {
                                                    let prev = files();
                                                    files.write().retain(|f| f.id != file_url);
                                                    match remove_quiz_file(
                                                            space_id(),
                                                            quiz_id(),
                                                            RemoveQuizFileRequest { file_url },
                                                        )
                                                        .await
                                                    {
                                                        Ok(_) => ctx.quiz.restart(),
                                                        Err(e) => {
                                                            error!("Failed to remove quiz file: {:?}", e);
                                                            files.set(prev);
                                                            toast.error(e);
                                                        }
                                                    }
                                                }
                                            },
                                            svg {
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                line {
                                                    x1: "18",
                                                    y1: "6",
                                                    x2: "6",
                                                    y2: "18",
                                                }
                                                line {
                                                    x1: "6",
                                                    y1: "6",
                                                    x2: "18",
                                                    y2: "18",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    FileUploader {
                        accept: ".pdf,.docx,.pptx,.xlsx,.png,.jpg,.jpeg,.mp4,.mov",
                        on_upload_success: move |_url: String| {},
                        on_upload_meta: move |uploaded: UploadedFileMeta| {
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
                                        uploader_name: None,
                                        uploader_profile_url: None,
                                        uploaded_at: Some(crate::common::utils::time::now()),
                                    });
                                save_files_after_upload(files());
                            }
                        },
                        div {
                            class: "dropzone",
                            "data-testid": "attachment-dropzone",
                            tabindex: "0",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "17 8 12 3 7 8" }
                                line {
                                    x1: "12",
                                    y1: "3",
                                    x2: "12",
                                    y2: "15",
                                }
                            }
                            div { class: "dropzone__text",
                                span { class: "dropzone__title", "{tr.dropzone_title}" }
                                span { class: "dropzone__sub", "{tr.dropzone_sub}" }
                            }
                        }
                    }
                }

                section { class: "section", "data-testid": "section-scoring",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_scoring_label}" }
                    }
                    div { class: "grid-2",
                        div { class: "field",
                            label { class: "field__label", "{tr.pass_score_label}" }
                            div { class: "input-group",
                                input {
                                    class: "input input--num",
                                    r#type: "number",
                                    min: "0",
                                    "data-testid": "quiz-pass-score",
                                    value: "{pass_score()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            pass_score.set(v);
                                        }
                                    },
                                    onblur: move |_| save_scoring(),
                                }
                                span { class: "input-suffix",
                                    "/ {total_questions} {tr.questions_suffix}"
                                }
                            }
                        }
                        div { class: "field",
                            label { class: "field__label", "{tr.retry_count_label}" }
                            div { class: "input-group",
                                input {
                                    class: "input input--num",
                                    r#type: "number",
                                    min: "0",
                                    "data-testid": "quiz-retry-count",
                                    value: "{retry_count()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            retry_count.set(v);
                                        }
                                    },
                                    onblur: move |_| save_scoring(),
                                }
                                span { class: "input-suffix", "{tr.retries_suffix}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AutosaveStatusBadge(status: SaveStatus) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let (label, modifier) = match status {
        SaveStatus::Idle => return rsx! {},
        SaveStatus::Saving => (tr.autosave_saving.to_string(), "autosave--saving"),
        SaveStatus::Saved => (tr.autosave_saved.to_string(), "autosave--saved"),
        SaveStatus::Unsaved => (tr.autosave_unsaved.to_string(), "autosave--unsaved"),
    };
    rsx! {
        span { class: "autosave {modifier}",
            span { class: "autosave__dot" }
            "{label}"
        }
    }
}
