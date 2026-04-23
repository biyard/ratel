//! "Application form / 신청폼" tab — consumes `UseSubTeamForm`.
//!
//! Renders the list of form fields, an "Add field" button, and per-row
//! delete. Reorder is collapsed to up/down arrows that push new field_id
//! orders to `handle_reorder` (matches the HTML mockup's simplified drag
//! handle semantics).

use crate::features::sub_team::models::SubTeamFormFieldType;
use crate::features::sub_team::{
    use_sub_team_form, CreateSubTeamFormFieldRequest, SubTeamFormFieldResponse, SubTeamTranslate,
    UpdateSubTeamFormFieldRequest, UseSubTeamForm,
};
use crate::*;

#[component]
pub fn FormTab() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamForm {
        fields,
        mut handle_create_field,
        mut handle_update_field,
        mut handle_delete_field,
        mut handle_reorder,
        ..
    } = use_sub_team_form()?;

    let rows: Vec<SubTeamFormFieldResponse> = fields().items.clone();

    let row_count = rows.len();

    rsx! {
        section {
            class: "card card--collapsible",
            id: "form",
            "data-collapsed": "false",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_form}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{row_count} fields" }
            }

            div { class: "card__body",
                div { class: "field-list", id: "field-list",
                    for (idx, field) in rows.iter().enumerate() {
                        FieldRow {
                            key: "{field.id}",
                            field: field.clone(),
                            is_first: idx == 0,
                            is_last: idx + 1 == row_count,
                            rows_snapshot: rows.clone(),
                            idx,
                            on_update: move |(field_id, req)| handle_update_field.call(field_id, req),
                            on_delete: move |field_id| handle_delete_field.call(field_id),
                            on_reorder: move |ids| handle_reorder.call(ids),
                        }
                    }
                }

                button {
                    class: "add-field-btn",
                    id: "add-field",
                    "data-testid": "sub-team-form-field-create-btn",
                    onclick: move |_| {
                        handle_create_field
                            .call(CreateSubTeamFormFieldRequest {
                                label: "New field".to_string(),
                                field_type: SubTeamFormFieldType::ShortText,
                                required: false,
                                order: None,
                                options: None,
                            });
                    },
                    lucide_dioxus::Plus { class: "w-3 h-3 [&>path]:stroke-current" }
                    "{tr.add_field}"
                }
            }
        }
    }
}

#[component]
fn FieldRow(
    field: SubTeamFormFieldResponse,
    is_first: bool,
    is_last: bool,
    rows_snapshot: Vec<SubTeamFormFieldResponse>,
    idx: usize,
    on_update: EventHandler<(String, UpdateSubTeamFormFieldRequest)>,
    on_delete: EventHandler<String>,
    on_reorder: EventHandler<Vec<String>>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let field_id = field.id.clone();
    let field_id_for_label = field_id.clone();
    let field_id_for_required = field_id.clone();
    let field_id_for_delete = field_id.clone();
    let label = field.label.clone();
    let required = field.required;
    let field_type = field.field_type;

    let move_up = {
        let rows_snapshot = rows_snapshot.clone();
        move |_| {
            if is_first {
                return;
            }
            let mut ids: Vec<String> = rows_snapshot.iter().map(|f| f.id.clone()).collect();
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
            let mut ids: Vec<String> = rows_snapshot.iter().map(|f| f.id.clone()).collect();
            ids.swap(idx, idx + 1);
            on_reorder.call(ids);
        }
    };

    rsx! {
        div {
            class: "field-row",
            "data-linked": "none",
            "data-testid": "sub-team-form-field-row",
            span { class: "field-row__drag",
                button {
                    class: "req-card__stepper",
                    disabled: is_first,
                    onclick: move_up,
                    lucide_dioxus::ChevronUp { class: "w-3 h-3 [&>path]:stroke-current" }
                }
                button {
                    class: "req-card__stepper",
                    disabled: is_last,
                    onclick: move_down,
                    lucide_dioxus::ChevronDown { class: "w-3 h-3 [&>path]:stroke-current" }
                }
            }
            select {
                class: "field-row__type",
                value: "{field_type_key(field_type)}",
                onchange: move |e| {
                    if let Some(ty) = parse_field_type(&e.value()) {
                        on_update
                            .call((
                                field_id.clone(),
                                UpdateSubTeamFormFieldRequest {
                                    field_type: Some(ty),
                                    ..Default::default()
                                },
                            ));
                    }
                },
                option { value: "short_text", "{tr.type_short_text}" }
                option { value: "long_text", "{tr.type_long_text}" }
                option { value: "number", "{tr.type_number}" }
                option { value: "date", "{tr.type_date}" }
                option { value: "single_select", "{tr.type_single_select}" }
                option { value: "multi_select", "{tr.type_multi_select}" }
                option { value: "url", "{tr.type_url}" }
            }
            input {
                class: "field-row__label",
                "data-testid": "sub-team-form-field-label-input",
                value: "{label}",
                onchange: move |e| {
                    on_update
                        .call((
                            field_id_for_label.clone(),
                            UpdateSubTeamFormFieldRequest {
                                label: Some(e.value()),
                                ..Default::default()
                            },
                        ));
                },
            }
            label { class: "field-row__req",
                input {
                    r#type: "checkbox",
                    "data-testid": "sub-team-form-field-required-check",
                    checked: required,
                    onchange: move |e| {
                        on_update
                            .call((
                                field_id_for_required.clone(),
                                UpdateSubTeamFormFieldRequest {
                                    required: Some(e.checked()),
                                    ..Default::default()
                                },
                            ));
                    },
                }
                " {tr.field_required}"
            }
            button {
                class: "field-row__del",
                "aria-label": "{tr.delete_field}",
                onclick: move |_| on_delete.call(field_id_for_delete.clone()),
                lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
            }
        }
    }
}

fn field_type_key(t: SubTeamFormFieldType) -> &'static str {
    match t {
        SubTeamFormFieldType::ShortText => "short_text",
        SubTeamFormFieldType::LongText => "long_text",
        SubTeamFormFieldType::Number => "number",
        SubTeamFormFieldType::Date => "date",
        SubTeamFormFieldType::SingleSelect => "single_select",
        SubTeamFormFieldType::MultiSelect => "multi_select",
        SubTeamFormFieldType::Url => "url",
    }
}

fn parse_field_type(s: &str) -> Option<SubTeamFormFieldType> {
    Some(match s {
        "short_text" => SubTeamFormFieldType::ShortText,
        "long_text" => SubTeamFormFieldType::LongText,
        "number" => SubTeamFormFieldType::Number,
        "date" => SubTeamFormFieldType::Date,
        "single_select" => SubTeamFormFieldType::SingleSelect,
        "multi_select" => SubTeamFormFieldType::MultiSelect,
        "url" => SubTeamFormFieldType::Url,
        _ => return None,
    })
}
