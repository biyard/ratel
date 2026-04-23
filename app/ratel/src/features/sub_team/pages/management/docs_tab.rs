//! "Documents / 문서" tab — consumes `UseSubTeamDocs`.
//!
//! Shows all parent-team documents (bylaws, policies) with an
//! "Add document" CTA that routes to `TeamSubTeamDocComposePage`, and
//! per-row up/down/delete buttons. Editing the body happens on the
//! dedicated compose page — this tab only supports toggling `required`,
//! reorder, delete, and add.

use crate::features::sub_team::{
    use_sub_team_docs, SubTeamDocumentResponse, SubTeamTranslate, UpdateSubTeamDocumentRequest,
    UseSubTeamDocs,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn DocsTab(username: String) -> Element {
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

    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_documents}" }
                span { class: "card__dash" }
                span { class: "card__meta",
                    "{item_count} documents · {required_count} "
                    "{tr.required_reading}"
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
                onclick: move |_| {
                    nav.push(Route::TeamSubTeamDocComposePage {
                        username: username_for_new.clone(),
                    });
                },
                lucide_dioxus::Plus { class: "w-3 h-3 [&>path]:stroke-current" }
                "{tr.add_document}"
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
    let body_preview = doc
        .body
        .chars()
        .take(180)
        .collect::<String>();
    let required = doc.required;

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
                    lucide_dioxus::FileText { class: "w-4 h-4 [&>path]:stroke-current" }
                }
                span { class: "doc-item__title", "{title}" }
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
                    onclick: move |_| {
                        nav.push(Route::TeamSubTeamDocEditPage {
                            username: edit_username.clone(),
                            doc_id: edit_doc_id.clone(),
                        });
                    },
                    lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                    "{tr.edit_document}"
                }
                div { class: "doc-item__actions",
                    button {
                        class: "doc-item__btn",
                        "aria-label": "Move up",
                        disabled: is_first,
                        onclick: move_up,
                        lucide_dioxus::ChevronUp { class: "w-3 h-3 [&>path]:stroke-current" }
                    }
                    button {
                        class: "doc-item__btn",
                        "aria-label": "Move down",
                        disabled: is_last,
                        onclick: move_down,
                        lucide_dioxus::ChevronDown { class: "w-3 h-3 [&>path]:stroke-current" }
                    }
                    button {
                        class: "doc-item__btn doc-item__btn--danger",
                        "aria-label": "Delete",
                        onclick: move |_| on_delete.call(doc_id_for_delete.clone()),
                        lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
                    }
                }
            }
            div { class: "doc-item__preview", "{body_preview}" }
        }
    }
}
