//! "Application form / 신청폼" tab — consumes `UseSubTeamForm`.
//!
//! Mirrors `assets/design/sub-team/subteam-management-page.html` line
//! 433–515: collapsible card with toolbar toggle, a Linked-field notice
//! banner, then a list of field rows with drag handle / type select /
//! label input / required toggle / delete button.
//!
//! The "Default" / locked rows in the mockup (제안하는 팀 이름, 설립
//! 목적) come from server-side seed data when the team is created —
//! they're indistinguishable from custom fields at the type level, so
//! we just render every backend field uniformly here. The `Default`
//! lock badge is left for a follow-up once the backend marks system-
//! seeded fields.

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
    let required_count = rows.iter().filter(|f| f.required).count();

    let mut collapsed = use_signal(|| false);
    let collapsed_attr = if collapsed() { "true" } else { "false" };

    rsx! {
        section {
            class: "card card--collapsible",
            id: "form",
            "data-collapsed": "{collapsed_attr}",
            div {
                class: "card__head",
                id: "form-head",
                onclick: move |_| collapsed.toggle(),
                h2 { class: "card__title", "{tr.form_card_title}" }
                span { class: "card__dash" }
                span { class: "card__meta", id: "form-meta",
                    "{row_count} {tr.form_meta_fields} · {required_count} {tr.form_meta_required}"
                }
                div { class: "card__head-toolbar",
                    button {
                        class: "card__toggle",
                        id: "form-toggle",
                        "aria-label": "Toggle form builder",
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            collapsed.toggle();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                }
            }

            div { class: "card__body",
                // ── Linked field notice banner ─────────────────
                div { class: "notice", style: "margin-bottom:10px",
                    div { class: "notice__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                        }
                    }
                    div { class: "notice__body",
                        span { class: "notice__title", "{tr.form_notice_title}" }
                        span { class: "notice__text", "{tr.form_notice_text}" }
                    }
                }

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
                                label: tr.form_new_field.to_string(),
                                field_type: SubTeamFormFieldType::ShortText,
                                required: false,
                                order: None,
                                options: None,
                            });
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

    // Drag handle currently doubles as up/down reorder buttons because
    // a real drag UX needs a touch/mouse-aware engine we haven't wired
    // yet. Mockup shows a 6-dot drag glyph; we keep the glyph visually
    // and stack tiny up/down buttons next to it.
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
            // Mockup-style 6-dot drag handle (visual only, paired with
            // hidden up/down buttons that are still keyboard-clickable).
            span { class: "field-row__drag",
                button {
                    style: "background:transparent;border:0;color:inherit;padding:0;cursor:pointer;display:flex;",
                    disabled: is_first,
                    onclick: move_up,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "9", cy: "5", r: "1" }
                        circle { cx: "9", cy: "12", r: "1" }
                        circle { cx: "9", cy: "19", r: "1" }
                        circle { cx: "15", cy: "5", r: "1" }
                        circle { cx: "15", cy: "12", r: "1" }
                        circle { cx: "15", cy: "19", r: "1" }
                    }
                }
                button {
                    style: "background:transparent;border:0;color:inherit;padding:0;cursor:pointer;display:none;",
                    disabled: is_last,
                    onclick: move_down,
                    "v"
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
                option { value: "short_text", "Short text" }
                option { value: "long_text", "Long text" }
                option { value: "number", "Number" }
                option { value: "date", "Date" }
                option { value: "single_select", "Single select" }
                option { value: "multi_select", "Multi select" }
                option { value: "url", "URL" }
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
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    polyline { points: "3 6 5 6 21 6" }
                    path { d: "M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
                }
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
