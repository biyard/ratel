use crate::common::types::{File, FileExtension};
use crate::features::spaces::pages::apps::apps::file::{
    CreateFileLinkRequest, DeleteFileLinkRequest, FileLinkTarget, components::FileUploadZone,
    create_file_link, delete_file_link, get_overview_files,
};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::pages::overview::controllers::{
    UpdateContentRequest, update_space_content,
};
use crate::features::spaces::space_common::controllers::SpaceResponse;
use std::collections::HashSet;

const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn OverviewPanel(
    open: bool,
    on_close: EventHandler<()>,
    space: SpaceResponse,
    participants: String,
    remaining: String,
    rewards: String,
    #[props(default)] is_admin: bool,
    space_id: ReadSignal<SpacePartition>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut space_ctx = crate::features::spaces::space_common::providers::use_space_context();
    let mut toast = use_toast();

    let author_profile = if space.author_profile_url.is_empty() {
        DEFAULT_PROFILE.to_string()
    } else {
        space.author_profile_url.clone()
    };

    let content_init = space.content.clone();
    let mut editing = use_signal(|| false);
    let mut about_value = use_signal(|| content_init.clone());
    let mut about_display = use_signal(|| content_init.clone());
    let mut is_saving = use_signal(|| false);

    // Files: load from server, track original vs current for diff on Save.
    // Loader runs unconditionally (hooks can't be conditional); we only surface
    // the Files section when `is_admin`.
    let mut file_loader =
        use_loader(move || async move { get_overview_files(space_id()).await })?;
    let mut files = use_signal(Vec::<File>::new);
    let mut original_files = use_signal(Vec::<File>::new);

    use_effect(move || {
        if !editing() {
            files.set(file_loader());
        }
    });

    rsx! {
        div {
            class: "overview-panel",
            "data-testid": "overview-panel",
            "data-open": open,
            "data-editing": editing(),
            div { class: "overview-panel__header",
                div { class: "overview-panel__title-wrap",
                    span { class: "overview-panel__title", "{tr.overview}" }
                    if editing() {
                        span { class: "overview-panel__mode-badge",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
                            }
                            "{tr.editing}"
                        }
                    }
                }
                div { class: "overview-panel__actions",
                    if is_admin && !editing() {
                        button {
                            aria_label: "{tr.edit}",
                            class: "overview-icon-btn overview-edit-btn",
                            "data-testid": "overview-edit-btn",
                            onclick: move |_| {
                                about_value.set(about_display());
                                original_files.set(files());
                                editing.set(true);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
                            }
                        }
                    }
                    if is_admin && editing() {
                        button {
                            aria_label: "{tr.cancel}",
                            class: "overview-icon-btn overview-icon-btn--danger",
                            "data-testid": "overview-cancel-btn",
                            disabled: is_saving(),
                            onclick: move |_| {
                                files.set(original_files());
                                editing.set(false);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                line {
                                    x1: "18",
                                    x2: "6",
                                    y1: "6",
                                    y2: "18",
                                }
                                line {
                                    x1: "6",
                                    x2: "18",
                                    y1: "6",
                                    y2: "18",
                                }
                            }
                        }
                        button {
                            class: "overview-save",
                            "data-testid": "overview-save-btn",
                            disabled: is_saving(),
                            onclick: move |_| async move {
                                if is_saving() {
                                    return;
                                }
                                is_saving.set(true);
                                let space_pk = space_id();
                                let html = about_value();
                                let current_files = files();
                                let orig_files = original_files();
                                let result = update_space_content(
                                        space_pk.clone(),
                                        UpdateContentRequest {
                                            content: html.clone(),
                                        },
                                    )
                                    .await;
                                match result {
                                    Ok(_) => {
                                        let orig_urls: HashSet<String> = orig_files
                                            .iter()
                                            .filter_map(|f| f.url.clone())
                                            .collect();
                                        let current_urls: HashSet<String> = current_files
                                            .iter()
                                            .filter_map(|f| f.url.clone())
                                            .collect();
                                        for file in &current_files {
                                            if let Some(url) = &file.url {
                                                if !orig_urls.contains(url) {
                                                    let _ = create_file_link(
                                                            space_pk.clone(),
                                                            CreateFileLinkRequest {
                                                                file_url: url.clone(),
                                                                file_name: Some(file.name.clone()),
                                                                link_target: FileLinkTarget::Overview,
                                                            },
                                                        )
                                                        .await;
                                                }
                                            }
                                        }
                                        for file in &orig_files {
                                            if let Some(url) = &file.url {
                                                if !current_urls.contains(url) {
                                                    let _ = delete_file_link(
                                                            space_pk.clone(),
                                                            DeleteFileLinkRequest {
                                                                file_url: url.clone(),
                                                                link_target: FileLinkTarget::Overview,
                                                            },
                                                        )
                                                        .await;
                                                }
                                            }
                                        }
                                        about_display.set(html);
                                        editing.set(false);
                                        file_loader.restart();
                                        space_ctx.space.restart();
                                    }
                                    Err(err) => {
                                        toast.error(err);
                                    }
                                }
                                is_saving.set(false);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" }
                                polyline { points: "17 21 17 13 7 13 7 21" }
                                polyline { points: "7 3 7 8 15 8" }
                            }
                            "{tr.save}"
                        }
                    }
                    button {
                        aria_label: "Close overview",
                        class: "overview-icon-btn overview-panel__close",
                        onclick: move |_| {
                            on_close.call(());
                        },
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            line {
                                x1: "18",
                                x2: "6",
                                y1: "6",
                                y2: "18",
                            }
                            line {
                                x1: "6",
                                x2: "18",
                                y1: "6",
                                y2: "18",
                            }
                        }
                    }
                }
            }
            div { class: "overview-panel__body",
                div { class: "overview-section",
                    div { class: "overview-section__label",
                        "{tr.about}"
                        span { class: "overview-section__label-hint", "{tr.about_edit_hint}" }
                    }
                    if !editing() {
                        div {
                            class: "overview-section__content ProseMirror",
                            "data-editable": "about",
                            dangerous_inner_html: "{about_display()}",
                        }
                    } else {
                        textarea {
                            class: "overview-section__input",
                            "data-testid": "overview-about-input",
                            placeholder: "{tr.quest_desc_placeholder}",
                            rows: "5",
                            value: "{about_value()}",
                            oninput: move |e| about_value.set(e.value()),
                        }
                    }
                }
                div { class: "overview-section",
                    span { class: "overview-section__label", "{tr.key_metrics}" }
                    div { class: "overview-info-grid",
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{participants}" }
                            span { class: "overview-info-card__label", "{tr.participants}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{remaining}" }
                            span { class: "overview-info-card__label", "{tr.spots_left}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{rewards}" }
                            span { class: "overview-info-card__label", "{tr.reward_pool}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{space.likes}" }
                            span { class: "overview-info-card__label", "{tr.likes}" }
                        }
                    }
                }
                if is_admin {
                    div { class: "overview-section overview-section--files",
                        div { class: "overview-section__label",
                            "{tr.files}"
                            span { class: "overview-section__label-hint", "{tr.files_hint}" }
                        }
                        div { class: "file-list",
                            for file in files().iter().cloned() {
                                {
                                    let kind_cls = match file.ext {
                                        FileExtension::PDF => "file-item__icon--pdf",
                                        FileExtension::JPG | FileExtension::PNG => "file-item__icon--img",
                                        _ => "file-item__icon--doc",
                                    };
                                    let url = file.url.clone();
                                    let key = url.clone().unwrap_or_else(|| file.name.clone());
                                    rsx! {
                                        div { class: "file-item", key: "{key}",
                                            div { class: "file-item__icon {kind_cls}",
                                                svg {
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    view_box: "0 0 24 24",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                                    polyline { points: "14 2 14 8 20 8" }
                                                }
                                            }
                                            div { class: "file-item__info",
                                                div { class: "file-item__name", "{file.name}" }
                                                div { class: "file-item__meta", "{file.size}" }
                                            }
                                            if editing() {
                                                div { class: "file-item__actions",
                                                    button {
                                                        aria_label: "Remove file",
                                                        class: "file-item__remove",
                                                        onclick: move |_| {
                                                            let key_url = url.clone();
                                                            files
                                                                .with_mut(|v| {
                                                                    v.retain(|f| f.url != key_url);
                                                                });
                                                        },
                                                        svg {
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if editing() {
                            FileUploadZone {
                                on_upload: move |file: File| {
                                    files.with_mut(|v| v.push(file));
                                },
                            }
                        }
                    }
                }
                div { class: "overview-section",
                    span { class: "overview-section__label", "{tr.created_by}" }
                    div { class: "overview-author-row",
                        img {
                            alt: "Author",
                            class: "overview-author-avatar",
                            src: "{author_profile}",
                        }
                        div {
                            div { class: "overview-author-name", "{space.author_display_name}" }
                            div { class: "overview-author-username", "@{space.author_username}" }
                        }
                    }
                }
            }
        }
    }
}
