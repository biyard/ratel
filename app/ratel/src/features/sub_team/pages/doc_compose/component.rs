//! Sub-team document composer — new + edit. Consumes
//! `UseSubTeamDocCompose`. Mirrors
//! `assets/design/sub-team/subteam-doc-compose.html` (full layout:
//! topbar, edit banner, title + rich editor body + attachments,
//! side panel with 필독 toggle / 문서 정보 / Discard, and a bottom
//! stat bar).
//!
//! Backend fields populated on save: title / body / required /
//! attachments / version / editor_username. Version bumps once per
//! save on the existing-doc path. The composer treats `version == 0`
//! as v1 for display so legacy rows render the same as freshly
//! created ones.

use crate::common::components::editor::Editor as RichEditor;
use crate::common::components::file_uploader::{FileUploader, UploadedFileMeta};
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::controllers::{
    create_sub_team_doc_handler, update_sub_team_doc_handler,
};
use crate::features::sub_team::{
    use_sub_team_doc_compose, CreateSubTeamDocumentRequest, SubTeamTranslate,
    UpdateSubTeamDocumentRequest, UseSubTeamDocCompose,
};
use crate::route::Route;
use crate::*;

// `File.size` is stored as a human string (e.g. "248 KB", "1.2 MB").
// To aggregate totals across multiple attachments we parse back into
// bytes and re-format — keeps the wire format unchanged while letting
// the composer show `{n} files · {total}` in the side panel.
pub fn parse_size_to_bytes(s: &str) -> u64 {
    let upper = s.trim().to_ascii_uppercase();
    let (num, mul) = if let Some(n) = upper.strip_suffix("MB") {
        (n, 1024u64 * 1024)
    } else if let Some(n) = upper.strip_suffix("KB") {
        (n, 1024u64)
    } else if let Some(n) = upper.strip_suffix("B") {
        (n, 1u64)
    } else {
        (upper.as_str(), 1u64)
    };
    let v: f64 = num.trim().parse().unwrap_or(0.0);
    (v.max(0.0) * mul as f64) as u64
}

pub fn pretty_size(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb >= 1.0 {
        format!("{:.1} MB", mb)
    } else {
        let kb = bytes as f64 / 1024.0;
        format!("{:.0} KB", kb.max(1.0))
    }
}

fn file_icon_modifier(ext: &FileExtension) -> &'static str {
    match ext {
        FileExtension::PDF => "file-row__icon--pdf",
        FileExtension::JPG | FileExtension::PNG => "file-row__icon--img",
        _ => "",
    }
}

fn file_ext_label(ext: &FileExtension) -> &'static str {
    match ext {
        FileExtension::PDF => "PDF",
        FileExtension::PNG => "PNG",
        FileExtension::JPG => "JPG",
        FileExtension::WORD => "DOCX",
        FileExtension::PPTX => "PPTX",
        FileExtension::EXCEL => "XLSX",
        FileExtension::ZIP => "ZIP",
        FileExtension::MP4 => "MP4",
        FileExtension::MOV => "MOV",
        FileExtension::MKV => "MKV",
    }
}

/// Newtype seeded via `use_context_provider` from the bylaws-mode
/// route so `DocComposeForm` can pass it into
/// `CreateSubTeamDocumentRequest.category` on save. A bare `String`
/// would collide with the `doc_id` context the page already provides.
#[derive(Clone, Debug, Default)]
pub struct DocComposeCategory(pub String);

#[component]
pub fn TeamSubTeamDocEditPage(username: String, doc_id: String) -> Element {
    // Existing edits stay on the pre-category compose path — the
    // category is already on the row and update_sub_team_doc doesn't
    // touch it.
    render_compose(username, Some(doc_id), String::new())
}

/// Single entry point for NEW documents. Always takes a category
/// (`"Bylaws"` / `"ClubBylaws"`) — Documents tab passes the team's
/// own category, bylaws page passes the section's category, and the
/// save handler dual-writes a backing Post.
#[component]
pub fn TeamSubTeamBylawsComposePage(username: String, category: String) -> Element {
    render_compose(username, None, category)
}

fn render_compose(
    username: String,
    doc_id: Option<String>,
    category: String,
) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);

    let doc_id_for_ctx = doc_id.clone();
    use_context_provider(move || doc_id_for_ctx.clone());

    // Bylaws-mode preset — empty string means "no category" (regular
    // doc) and the save handler skips the dual-write.
    let category_for_ctx = category.clone();
    use_context_provider(move || DocComposeCategory(category_for_ctx.clone()));

    rsx! {
        SeoMeta { title: if doc_id.is_some() { "{tr.doc_compose_title_edit}" } else { "{tr.doc_compose_title_new}" } }
        DocComposeForm { username: username.clone() }
    }
}

#[component]
fn DocComposeForm(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamDocCompose {
        team_id,
        doc_id,
        doc,
        ..
    } = use_sub_team_doc_compose()?;

    let existing = doc();
    let initial_title = existing
        .as_ref()
        .map(|d| d.title.clone())
        .unwrap_or_default();
    let initial_body = existing
        .as_ref()
        .map(|d| d.body.clone())
        .unwrap_or_default();
    let initial_required = existing.as_ref().map(|d| d.required).unwrap_or(false);
    let initial_doc_id = doc_id();
    let initial_updated_at = existing.as_ref().map(|d| d.updated_at).unwrap_or(0);
    let initial_version = existing.as_ref().map(|d| d.version.max(1)).unwrap_or(0);
    let initial_editor = existing
        .as_ref()
        .map(|d| d.editor_username.clone())
        .unwrap_or_default();
    let initial_attachments: Vec<File> = existing
        .as_ref()
        .map(|d| d.attachments.clone())
        .unwrap_or_default();

    let mut title: Signal<String> = use_signal(|| initial_title);
    let mut body: Signal<String> = use_signal(|| initial_body);
    let mut required: Signal<bool> = use_signal(|| initial_required);
    let mut attachments: Signal<Vec<File>> = use_signal(|| initial_attachments);
    // Mobile drawer state — same pattern as post_edit. Click "옵션" in the
    // bottom-bar to slide the side-panel up from below.
    let mut drawer_open = use_signal(|| false);

    let current_id: Signal<Option<String>> = use_signal(move || initial_doc_id.clone());

    let username_for_back = username.clone();
    let username_for_after = username.clone();
    let username_for_discard = username.clone();

    // Save handler — call the server functions directly (not via the
    // hook's `use_action` wrappers) and only navigate after the await
    // resolves. Using fire-and-forget `Action::call(...)` followed by
    // a synchronous `nav.push(...)` would unmount this component
    // mid-await and silently drop the future, so the doc would never
    // hit the server (see CLAUDE.md anti-patterns § "Async Event
    // Handlers" / hooks-and-actions guidance).
    // Bylaws-mode category seeded via `DocComposeCategory` context. Empty
    // string = regular doc; non-empty = bylaws/club-bylaws.
    let category_ctx: DocComposeCategory =
        try_consume_context().unwrap_or_default();
    let category_value: Option<String> = if category_ctx.0.trim().is_empty() {
        None
    } else {
        Some(category_ctx.0.clone())
    };
    let save_action = use_callback(move |_| {
        let username_for_after = username_for_after.clone();
        let category_value = category_value.clone();
        spawn(async move {
            let t = title();
            let b = body();
            let r = required();
            let files = attachments();
            let team = team_id();

            let result: crate::common::Result<()> = if let Some(id) = current_id() {
                update_sub_team_doc_handler(
                        team,
                        id,
                        UpdateSubTeamDocumentRequest {
                            title: Some(t),
                            body: Some(b),
                            required: Some(r),
                            attachments: Some(files),
                            ..Default::default()
                        },
                    )
                    .await
                    .map(|_| ())
            } else {
                create_sub_team_doc_handler(
                        team,
                        CreateSubTeamDocumentRequest {
                            title: t,
                            body: b,
                            required: r,
                            attachments: Some(files),
                            category: category_value.clone(),
                            ..Default::default()
                        },
                    )
                    .await
                    .map(|_| ())
            };

            if let Err(e) = result {
                crate::error!("sub-team doc save failed: {e}");
                return;
            }
            nav.push(Route::TeamSubTeamManagementPage {
                username: username_for_after,
            });
        });
    });

    let discard_action = move |_| {
        nav.push(Route::TeamSubTeamManagementPage {
            username: username_for_discard.clone(),
        });
    };

    // Body stats — derived from current `body` signal.
    let body_text_for_count = body();
    let char_count = body_text_for_count.chars().count();
    let word_count = body_text_for_count
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .count();

    let updated_at_display = if initial_updated_at > 0 {
        crate::common::utils::time::time_ago(initial_updated_at)
    } else {
        String::from("—")
    };

    // Document-info fields derived from the response.
    let version_label = if current_id().is_some() {
        format!("v{}", initial_version.max(1))
    } else {
        String::from("draft v1")
    };
    let editor_label = if initial_editor.is_empty() {
        String::from("—")
    } else {
        initial_editor.clone()
    };

    // Attachment aggregate label: `{n} {파일} · {total}` — empty
    // when there are no attachments yet.
    let attachments_now = attachments();
    let attachment_count = attachments_now.len();
    let attachments_size_label = if attachment_count == 0 {
        tr.doc_compose_attach_none.to_string()
    } else {
        let total_bytes: u64 = attachments_now
            .iter()
            .map(|f| parse_size_to_bytes(&f.size))
            .sum();
        format!(
            "{} {} · {}",
            attachment_count,
            tr.doc_compose_attach_files_unit,
            pretty_size(total_bytes)
        )
    };

    let title_for_topbar = title();
    let topbar_main = if !title_for_topbar.is_empty() {
        title_for_topbar.clone()
    } else if current_id().is_some() {
        tr.doc_compose_title_edit.to_string()
    } else {
        tr.doc_compose_title_new.to_string()
    };

    rsx! {
        // Standalone composer — route lives OUTSIDE TeamArenaLayout
        // (see route.rs). `.arena` class intentionally omitted: the
        // page-scoped style provides `min-height: 100vh + flex column`
        // and we handle our own topbar / banner / footer.
        div { class: "sub-team-doc-compose",

            // ── 1. TOP BAR ─────────────────────────────────────
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.push(Route::TeamSubTeamManagementPage {
                                username: username_for_back.clone(),
                            });
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.doc_compose_eyebrow}" }
                        span { class: "topbar-title__main", "{topbar_main}" }
                    }
                }
                div { class: "arena-topbar__right",
                    span { class: "autosave",
                        if current_id().is_some() {
                            "{tr.doc_compose_autosaved}"
                        } else {
                            "{tr.doc_compose_draft_unsaved}"
                        }
                    }
                    button {
                        class: "topbar-btn",
                        "data-testid": "sub-team-doc-preview-btn",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                            circle { cx: "12", cy: "12", r: "3" }
                        }
                        "{tr.doc_compose_preview}"
                    }
                    button {
                        class: "topbar-btn topbar-btn--primary",
                        id: "publish-btn",
                        "data-testid": "sub-team-doc-save-btn",
                        onclick: save_action,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        "{tr.save}"
                    }
                }
            }

            // ── 2. EDIT BANNER ─────────────────────────────────
            // Full-bleed banner outside the composer so it stretches edge
            // to edge across the page, matching the user's preference.
            div { class: "edit-banner",
                div { class: "edit-banner__inner",
                    div { class: "edit-banner__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                        }
                    }
                    div { class: "edit-banner__body",
                        div { class: "edit-banner__label", "{tr.doc_compose_banner_label}" }
                        div { class: "edit-banner__text", "{tr.doc_compose_banner_text}" }
                    }
                }
            }

            // ── 3. COMPOSER PAGE (grid: composer + side-panel) ─
            div { class: "composer-page",
                main { class: "composer",
                    // Title
                    div {
                        input {
                            class: "title-input",
                            r#type: "text",
                            "data-testid": "sub-team-doc-title-input",
                            placeholder: "{tr.doc_compose_title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| title.set(e.value()),
                        }
                        div { class: "title-divider" }
                    }

                    // Body — Ratel rich editor (toolbar + contenteditable)
                    RichEditor {
                        class: "w-full",
                        content: body(),
                        editable: true,
                        placeholder: tr.doc_compose_body_placeholder.to_string(),
                        on_content_change: move |html: String| body.set(html),
                    }
                }

                aside {
                    class: "side-panel",
                    "data-open": drawer_open(),
                    // Mobile drawer head — handle + title + close button.
                    // Hidden on desktop via base `.side-panel__head { display: none }`.
                    div { class: "side-panel__head",
                        span { class: "side-panel__handle" }
                        span { class: "side-panel__title", "{tr.sub_team_options_drawer_title}" }
                        button {
                            class: "side-panel__close",
                            "aria-label": tr.sub_team_options_close,
                            onclick: move |_| drawer_open.set(false),
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

                    // 필독 toggle
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                            "{tr.doc_compose_required_card_title}"
                        }
                        label { class: "required-row", "data-on": "{required()}",
                            div { class: "required-row__body",
                                div { class: "required-row__title",
                                    if required() {
                                        "{tr.doc_compose_required_on}"
                                    } else {
                                        "{tr.doc_compose_required_off}"
                                    }
                                }
                                div { class: "required-row__desc", "{tr.doc_compose_required_desc}" }
                            }
                            span {
                                class: "switch",
                                "aria-checked": "{required()}",
                                input {
                                    r#type: "checkbox",
                                    "data-testid": "sub-team-doc-required-toggle",
                                    checked: required(),
                                    onchange: move |e| required.set(e.checked()),
                                }
                            }
                        }
                    }

                    // 문서 정보
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "10" }
                                polyline { points: "12 6 12 12 16 14" }
                            }
                            "{tr.doc_compose_info_title}"
                        }
                        div { class: "meta-list",
                            div { class: "meta-row",
                                span { class: "meta-row__k", "{tr.doc_compose_info_version}" }
                                span { class: "meta-row__v meta-row__v--accent", "{version_label}" }
                            }
                            // Last updated / Editor only make sense once the
                            // doc has been saved at least once (v1+). For a
                            // brand-new unsaved doc there's no timestamp /
                            // editor yet, so we hide both rows in that case.
                            if current_id().is_some() {
                                div { class: "meta-row",
                                    span { class: "meta-row__k", "{tr.doc_compose_info_updated}" }
                                    span { class: "meta-row__v", "{updated_at_display}" }
                                }
                                div { class: "meta-row",
                                    span { class: "meta-row__k", "{tr.doc_compose_info_editor}" }
                                    span { class: "meta-row__v", "{editor_label}" }
                                }
                            }
                            div { class: "meta-row",
                                span { class: "meta-row__k", "{tr.doc_compose_info_attachments}" }
                                span { class: "meta-row__v", "{attachments_size_label}" }
                            }
                            div { class: "meta-row",
                                span { class: "meta-row__k", "{tr.doc_compose_info_required}" }
                                span { class: "meta-row__v meta-row__v--accent",
                                    if required() {
                                        "{tr.yes}"
                                    } else {
                                        "{tr.no}"
                                    }
                                }
                            }
                        }
                    }

                    // Attachments — moved from inside the composer to the
                    // sidebar (above the discard button) so the body area
                    // stays clean and the upload affordance lives next to
                    // 문서 정보 / Required toggle.
                    div { class: "side-card",
                        div { class: "attachments",
                            div { class: "attachments__title",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                                }
                                "{tr.doc_compose_attachments_title}"
                                span { class: "attachments__count", "{attachment_count}" }
                            }

                            div { class: "attachment-list", id: "attach-list",
                                for (idx, f) in attachments_now.iter().enumerate() {
                                    AttachmentRow {
                                        key: "{f.id}-{idx}",
                                        file: f.clone(),
                                        on_remove: move |_| {
                                            attachments
                                                .with_mut(|v| {
                                                    if idx < v.len() {
                                                        v.remove(idx);
                                                    }
                                                });
                                        },
                                    }
                                }
                            }

                            FileUploader {
                                class: "file-dropzone".to_string(),
                                accept: ".pdf,.docx,.pptx,.xlsx,.png,.jpg,.jpeg".to_string(),
                                on_upload_success: move |_: String| {},
                                on_upload_meta: move |uploaded: UploadedFileMeta| {
                                    let UploadedFileMeta { url, name, size } = uploaded;
                                    let uploaded_name = if name.trim().is_empty() {
                                        url.split('/').next_back().unwrap_or("file").to_string()
                                    } else {
                                        name
                                    };
                                    let ext = FileExtension::from_name_or_url(&uploaded_name, &url);
                                    attachments
                                        .with_mut(|v| {
                                            v.push(File {
                                                id: url.clone(),
                                                name: uploaded_name,
                                                size,
                                                ext,
                                                url: Some(url),
                                                uploader_name: None,
                                                uploader_profile_url: None,
                                                uploaded_at: Some(
                                                    crate::common::utils::time::get_now_timestamp_millis(),
                                                ),
                                            });
                                        });
                                },
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
                                span { "data-testid": "sub-team-doc-attach-btn",
                                    "{tr.doc_compose_upload_title}"
                                    small { "{tr.doc_compose_upload_hint}" }
                                }
                            }
                        }
                    }

                    // Discard
                    button {
                        class: "danger-row",
                        r#type: "button",
                        onclick: discard_action,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" }
                            path { d: "M10 11v6" }
                            path { d: "M14 11v6" }
                        }
                        "{tr.doc_compose_discard}"
                    }

                    // Delete moved out of the composer — admins delete docs
                    // from the management page's docs tab instead, which keeps
                    // the destructive action one click away from the edit
                    // surface.
                }
            }

            // ── 4. DRAWER BACKDROP (mobile only) ───────────────
            div {
                class: "drawer-backdrop",
                "data-open": drawer_open(),
                onclick: move |_| drawer_open.set(false),
            }

            // ── 5. BOTTOM BAR ──────────────────────────────────
            div { class: "bottom-bar",
                div { class: "bottom-bar__left",
                    span { class: "bottom-bar__stat",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                        strong { "{word_count}" }
                        " {tr.doc_compose_stats_words} · "
                        strong { "{char_count}" }
                        " {tr.doc_compose_stats_chars}"
                    }
                }
                div { class: "bottom-bar__right",
                    button {
                        class: "bottom-bar__btn bottom-bar__btn--mobile",
                        onclick: move |_| drawer_open.set(true),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "4", y1: "21", x2: "4", y2: "14" }
                            line { x1: "4", y1: "10", x2: "4", y2: "3" }
                            line { x1: "12", y1: "21", x2: "12", y2: "12" }
                            line { x1: "12", y1: "8", x2: "12", y2: "3" }
                            line { x1: "20", y1: "21", x2: "20", y2: "16" }
                            line { x1: "20", y1: "12", x2: "20", y2: "3" }
                            line { x1: "1", y1: "14", x2: "7", y2: "14" }
                            line { x1: "9", y1: "8", x2: "15", y2: "8" }
                            line { x1: "17", y1: "16", x2: "23", y2: "16" }
                        }
                        "{tr.sub_team_options}"
                    }
                    button {
                        class: "bottom-bar__btn bottom-bar__btn--mobile bottom-bar__btn--primary",
                        "data-testid": "sub-team-doc-save-btn-mobile",
                        onclick: save_action,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        span { "{tr.save}" }
                    }
                }
            }
        }
    }
}

#[component]
fn AttachmentRow(file: File, on_remove: EventHandler<()>) -> Element {
    let icon_modifier = file_icon_modifier(&file.ext);
    let icon_class = format!("file-row__icon {icon_modifier}");
    let ext_label = file_ext_label(&file.ext);
    let download_href = file.url.clone().unwrap_or_default();
    let download_target = if download_href.is_empty() {
        None
    } else {
        Some("_blank")
    };

    rsx! {
        div { class: "file-row", "data-testid": "sub-team-doc-attach-row",
            div { class: "{icon_class}",
                match file.ext {
                    FileExtension::JPG | FileExtension::PNG => rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "3",
                                width: "18",
                                height: "18",
                                rx: "2",
                                ry: "2",
                            }
                            circle { cx: "8.5", cy: "8.5", r: "1.5" }
                            polyline { points: "21 15 16 10 5 21" }
                        }
                    },
                    _ => rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                            line {
                                x1: "9",
                                y1: "15",
                                x2: "15",
                                y2: "15",
                            }
                        }
                    },
                }
            }
            a {
                class: "file-row__body",
                href: "{download_href}",
                target: download_target,
                rel: "noopener noreferrer",
                div { class: "file-row__name", "{file.name}" }
                div { class: "file-row__meta",
                    "{ext_label} · "
                    strong { "{file.size}" }
                }
            }
            span { class: "file-row__progress",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "20 6 9 17 4 12" }
                }
                "Uploaded"
            }
            button {
                class: "file-row__del",
                "aria-label": "Remove file",
                r#type: "button",
                onclick: move |_| on_remove.call(()),
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
