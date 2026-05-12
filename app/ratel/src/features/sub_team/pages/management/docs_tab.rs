//! "Documents / 문서" tab — consumes `UseSubTeamDocs`.
//!
//! Ports `assets/design/sub-team/subteam-management-page.html` 1:1 —
//! `notice` banner + `doc-list` of `doc-item` rows (head with input
//! title, updated date, required toggle pill, edit pill, up/down/del
//! actions; preview body; foot with stats + hint) + `add-doc-btn`.
//! Editing the body still routes to the dedicated compose page.

use crate::features::sub_team::{
    use_sub_team_docs, SubTeamDocumentResponse, SubTeamTranslate, UpdateSubTeamDocumentRequest,
    UseSubTeamDocs,
};
use crate::route::Route;
use crate::*;

fn format_updated_date(updated_at: i64) -> String {
    if updated_at <= 0 {
        return String::new();
    }
    chrono::DateTime::<chrono::Utc>::from_timestamp_millis(updated_at)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

#[component]
pub fn DocsTab(
    username: String,
    // Default category for newly-added docs. Computed by the parent
    // management component from `team_data.parent_team_id`:
    // - team without parent (=상위팀) → "Bylaws"
    // - team with parent (=하위팀)  → "ClubBylaws"
    // Empty string disables auto-category and the existing
    // non-bylaws compose route is used.
    #[props(default)] category: String,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamDocs {
        docs,
        mut handle_update,
        mut handle_delete,
        mut handle_reorder,
        ..
    } = use_sub_team_docs()?;

    let items: Vec<SubTeamDocumentResponse> = docs().items.clone();
    let item_count = items.len();
    let required_count = items.iter().filter(|d| d.required).count();

    let nav = use_navigator();
    let username_for_new = username.clone();

    let username_for_bylaws = username.clone();
    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.docs_card_title}" }
                span { class: "card__dash" }
                span { class: "card__meta",
                    "{item_count} documents · {required_count} {tr.required_reading}"
                }
                // "View bylaws" — public reader page that mirrors the
                // documents list with the bylaws-section visual treatment.
                // Anchor href instead of `nav.push` for the same dioxus
                // 0.7 reconciler workaround used by deregister/leave.
                a {
                    class: "card-head__link",
                    "data-testid": "sub-team-docs-view-bylaws",
                    href: "/{username_for_bylaws}/bylaws",
                    lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                    "{tr.docs_view_bylaws}"
                }
            }

            div { class: "notice", style: "margin-bottom: 10px",
                div { class: "notice__icon",
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
                div { class: "notice__body",
                    span { class: "notice__title", "{tr.docs_banner_title}" }
                    span { class: "notice__text", "{tr.docs_banner_text}" }
                }
            }

            div { class: "doc-list", id: "doc-list",
                for (idx, doc) in items.iter().enumerate() {
                    DocItem {
                        key: "{doc.id}",
                        doc: doc.clone(),
                        is_first: idx == 0,
                        is_last: idx + 1 == item_count,
                        rows_snapshot: items.clone(),
                        idx,
                        username: username.clone(),
                        on_update: move |(id, req)| handle_update.call(id, req),
                        on_delete: move |id| handle_delete.call(id),
                        on_reorder: move |ids| handle_reorder.call(ids),
                    }
                }
            }

            button {
                class: "add-doc-btn",
                id: "add-doc",
                "data-testid": "sub-team-doc-add-btn",
                style: "margin-top: 12px",
                onclick: {
                    let category = category.clone();
                    move |_| {
                        // Every doc gets a category (parent's "Bylaws"
                        // or sub-team's "ClubBylaws") so the dual-write
                        // (SubTeamDocument + backing Post) always fires
                        // and the doc shows up on the bylaws page.
                        nav.push(Route::TeamSubTeamBylawsComposePage {
                            username: username_for_new.clone(),
                            category: category.clone(),
                        });
                    }
                },
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line {
                        x1: "12",
                        y1: "5",
                        x2: "12",
                        y2: "19",
                    }
                    line {
                        x1: "5",
                        y1: "12",
                        x2: "19",
                        y2: "12",
                    }
                }
                "{tr.docs_add_btn}"
            }
        }
    }
}

#[component]
fn DocItem(
    doc: SubTeamDocumentResponse,
    is_first: bool,
    is_last: bool,
    rows_snapshot: Vec<SubTeamDocumentResponse>,
    idx: usize,
    username: String,
    on_update: EventHandler<(String, UpdateSubTeamDocumentRequest)>,
    on_delete: EventHandler<String>,
    on_reorder: EventHandler<Vec<String>>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let doc_id = doc.id.clone();
    let doc_id_for_required = doc_id.clone();
    let doc_id_for_delete = doc_id.clone();
    let title = doc.title.clone();
    let body_preview = doc.body.chars().take(180).collect::<String>();
    let required = doc.required;

    // Foot stats — `{chars}자 · {N} 파일 · {size}` mirroring mockup.
    let char_count = doc.body.chars().count();
    let attachment_count = doc.attachments.len();
    let total_bytes: u64 = doc
        .attachments
        .iter()
        .map(|f| crate::features::sub_team::pages::doc_compose::parse_size_to_bytes(&f.size))
        .sum();
    let size_label =
        crate::features::sub_team::pages::doc_compose::pretty_size(total_bytes);
    let updated_label = format_updated_date(doc.updated_at);

    let nav = use_navigator();
    let edit_username = username.clone();
    let edit_doc_id = doc_id.clone();

    let move_up = {
        let rows_snapshot = rows_snapshot.clone();
        move |_| {
            if is_first {
                return;
            }
            let mut ids: Vec<String> = rows_snapshot.iter().map(|d| d.id.clone()).collect();
            ids.swap(idx, idx - 1);
            on_reorder.call(ids);
        }
    };
    let move_down = {
        let rows_snapshot = rows_snapshot.clone();
        move |_| {
            if is_last {
                return;
            }
            let mut ids: Vec<String> = rows_snapshot.iter().map(|d| d.id.clone()).collect();
            ids.swap(idx, idx + 1);
            on_reorder.call(ids);
        }
    };

    rsx! {
        div {
            class: "doc-item",
            "data-required": "{required}",
            "data-testid": "sub-team-doc-item",

            div { class: "doc-item__head",
                span { class: "doc-item__icon",
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
                input {
                    class: "doc-item__title",
                    r#type: "text",
                    "data-testid": "sub-team-doc-title",
                    value: "{title}",
                    readonly: "true",
                }
                if !updated_label.is_empty() {
                    span { class: "doc-item__updated", "{updated_label} {tr.docs_updated_suffix}" }
                }
                label { class: "doc-item__req",
                    input {
                        r#type: "checkbox",
                        "data-testid": "sub-team-doc-required-check",
                        checked: required,
                        onchange: move |e| {
                            on_update
                                .call((
                                    doc_id_for_required.clone(),
                                    UpdateSubTeamDocumentRequest {
                                        required: Some(e.checked()),
                                        ..Default::default()
                                    },
                                ));
                        },
                    }
                    span { class: "doc-item__req-pill", "{tr.required_reading}" }
                }
                a {
                    class: "doc-item__edit",
                    "data-testid": "sub-team-doc-edit-btn",
                    onclick: move |_| {
                        nav.push(Route::TeamSubTeamDocEditPage {
                            username: edit_username.clone(),
                            doc_id: edit_doc_id.clone(),
                        });
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 20h9" }
                        path { d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4z" }
                    }
                    "{tr.docs_edit_btn}"
                }
                div { class: "doc-item__actions",
                    button {
                        class: "doc-item__btn",
                        "aria-label": "Move up",
                        disabled: is_first,
                        onclick: move_up,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "18 15 12 9 6 15" }
                        }
                    }
                    button {
                        class: "doc-item__btn",
                        "aria-label": "Move down",
                        disabled: is_last,
                        onclick: move_down,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    button {
                        class: "doc-item__btn doc-item__btn--danger",
                        "aria-label": "Delete",
                        onclick: move |_| on_delete.call(doc_id_for_delete.clone()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
                        }
                    }
                }
            }

            div { class: "doc-item__preview", "{body_preview}" }

            div { class: "doc-item__foot",
                span { class: "doc-item__stats",
                    span {
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
                        "{char_count}{tr.doc_compose_stats_chars}"
                    }
                    span {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                        }
                        "{attachment_count} {tr.doc_compose_attach_files_unit}"
                        if attachment_count > 0 {
                            " · {size_label}"
                        }
                    }
                }
                span { class: "doc-item__hint",
                    if required {
                        "{tr.doc_compose_required_on} · {tr.doc_compose_required_desc_short}"
                    } else {
                        "{tr.doc_compose_required_off} · {tr.doc_compose_reference_only}"
                    }
                }
            }
        }
    }
}
