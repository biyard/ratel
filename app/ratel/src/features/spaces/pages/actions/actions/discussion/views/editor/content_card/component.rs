use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::common::types::extract_filename_from_url;
use crate::features::spaces::pages::actions::actions::discussion::views::editor::DiscussionEditorTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;

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
    let tr: DiscussionEditorTranslate = use_translate();
    let mut ctx = use_discussion_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let discussion_id = ctx.discussion_id;

    let initial_post = ctx.discussion().post.clone();
    let initial_html = initial_post.html_contents.clone();
    let initial_title = initial_post.title.clone();
    let initial_files = initial_post.files.clone();

    let mut html_contents = use_signal(|| initial_html.clone());
    let mut last_saved_html = use_signal(|| initial_html);
    let mut title = use_signal(|| initial_title.clone());
    let mut last_saved_title = use_signal(|| initial_title);
    let mut title_version = use_signal(|| 0u64);
    let mut html_version = use_signal(|| 0u64);
    let mut title_status = use_signal(|| SaveStatus::Idle);
    let mut html_status = use_signal(|| SaveStatus::Idle);
    let mut files = use_signal(|| initial_files);

    let mut save_html = move || {
        let current = html_contents();
        if current == last_saved_html() {
            return;
        }
        html_status.set(SaveStatus::Saving);
        spawn(async move {
            let req = UpdateDiscussionRequest {
                title: None,
                html_contents: Some(current.clone()),
                category_name: None,
                started_at: None,
                ended_at: None,
                files: None,
            };
            if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
                error!("Failed to save discussion content: {:?}", err);
                html_status.set(SaveStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved_html.set(current);
                html_status.set(SaveStatus::Saved);
                ctx.discussion.restart();
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
            let req = UpdateDiscussionRequest {
                title: Some(current.clone()),
                html_contents: None,
                category_name: None,
                started_at: None,
                ended_at: None,
                files: None,
            };
            if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
                error!("Failed to save discussion title: {:?}", err);
                title_status.set(SaveStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved_title.set(current);
                title_status.set(SaveStatus::Saved);
                ctx.discussion.restart();
            }
        });
    };

    // Autosave title — 3-second debounce
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

    // Autosave html_contents — 3-second debounce
    use_effect(move || {
        let version = html_version();
        if version == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
            if html_version() != version {
                return;
            }
            if html_contents() == last_saved_html() {
                return;
            }
            save_html();
        });
    });

    let save_files_after_upload = move |next_files: Vec<File>| {
        spawn(async move {
            let req = UpdateDiscussionRequest {
                title: None,
                html_contents: None,
                category_name: None,
                started_at: None,
                ended_at: None,
                files: Some(next_files),
            };
            if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
                error!("Failed to save discussion files: {:?}", err);
                toast.error(err);
            } else {
                ctx.discussion.restart();
            }
        });
    };

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
                        div {
                            style: "display:flex;align-items:center;justify-content:space-between;gap:8px",
                            label { class: "field__label", "{tr.title_label}" }
                            AutosaveStatusBadge { status: title_status() }
                        }
                        input {
                            class: "input",
                            "data-testid": "discussion-title",
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
                        crate::common::components::TiptapEditor {
                            class: "[&_[data-tiptap-toolbar]]:border-b [&_[data-tiptap-toolbar]]:border-[rgba(255,255,255,0.06)] [&_[contenteditable='true']]:min-h-[220px] [&_[contenteditable='true']]:px-[22px] [&_[contenteditable='true']]:py-[20px] [&_[contenteditable='true']]:outline-none",
                            content: html_contents(),
                            editable: true,
                            placeholder: "",
                            on_content_change: move |html: String| {
                                html_contents.set(html.clone());
                                if html != last_saved_html() {
                                    html_status.set(SaveStatus::Unsaved);
                                    html_version.set(html_version() + 1);
                                }
                            },
                        }
                        div { class: "editor__footer",
                            AutosaveStatusBadge { status: html_status() }
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
                        for (idx , file) in files().iter().enumerate() {
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
                                            div { class: "file-row__meta",
                                                "{ext_label} \u{00B7} {file_size}"
                                            }
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
                                                    match remove_discussion_file(
                                                            space_id(),
                                                            discussion_id(),
                                                            RemoveDiscussionFileRequest {
                                                                file_url,
                                                            },
                                                        )
                                                        .await
                                                    {
                                                        Ok(_) => ctx.discussion.restart(),
                                                        Err(e) => {
                                                            error!("Failed to remove discussion file: {:?}", e);
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
                                            link_target: crate::features::spaces::pages::apps::apps::file::FileLinkTarget::Board(
                                                discussion_id().to_string(),
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
            }
        }
    }
}

#[component]
fn AutosaveStatusBadge(status: SaveStatus) -> Element {
    let tr: DiscussionEditorTranslate = use_translate();
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
