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
use crate::features::sub_team::types::TeamProfileLink;
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
                                linked_to: None,
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
    let field_id_for_link = field_id.clone();
    let field_id_for_options_change = field_id.clone();
    let field_id_for_delete = field_id.clone();
    let label = field.label.clone();
    let required = field.required;
    let field_type = field.field_type;
    let linked_to = field.linked_to;
    let locked = field.locked;
    let options_snapshot = field.options.clone();
    let has_options_panel = matches!(
        field_type,
        SubTeamFormFieldType::SingleSelect | SubTeamFormFieldType::MultiSelect
    );
    let linked_attr = link_data_attr(linked_to);
    let linked_label = link_button_label(linked_to, &tr);
    let mut menu_open: Signal<bool> = use_signal(|| false);
    let btn_linked_attr = if linked_to.is_some() { "set" } else { "none" };

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
        div { class: "field-row-wrap", "data-testid": "sub-team-form-field-row",

            div {
                class: "field-row",
                "data-linked": "{linked_attr}",
                "data-locked": "{locked}",

                // Mockup-style 6-dot drag handle (visual only, paired
                // with up/down reorder buttons that are still
                // keyboard-clickable).
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
                    disabled: locked,
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
                    readonly: locked,
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

                // ── "Default" lock badge ──────────────────────────
                // Renders only for system-seeded locked fields like
                // "팀 이름" / "설립 목적" — admins can't change the
                // type, label, link, or delete these.
                if locked {
                    span { class: "field-row__lock-badge", "Default" }
                }

                // ── 🔗 LINK button + popover menu ──────────────────
                // Mirrors mockup §123-136 (`.field-row__link-btn` +
                // `.field-row__link-menu`). Set `linked_to` to pull
                // the value from the applicant team's profile at
                // submit time. Set-only today — clearing isn't
                // supported via PATCH.
                span { class: "field-row__link-wrap",
                    button {
                        class: "field-row__link-btn",
                        "data-linked": "{btn_linked_attr}",
                        "data-testid": "sub-team-form-field-link-btn",
                        r#type: "button",
                        disabled: locked,
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            if !locked {
                                menu_open.toggle();
                            }
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                        }
                        span { "{linked_label}" }
                    }
                    if menu_open() {
                        div {
                            class: "field-row__link-menu",
                            "data-open": "true",
                            LinkOption {
                                label: tr.form_link_team_name,
                                hint: "team.name",
                                selected: matches!(linked_to, Some(TeamProfileLink::TeamName)),
                                on_pick: {
                                    let field_id = field_id_for_link.clone();
                                    move |_| {
                                        on_update
                                            .call((
                                                field_id.clone(),
                                                UpdateSubTeamFormFieldRequest {
                                                    linked_to: Some(TeamProfileLink::TeamName),
                                                    ..Default::default()
                                                },
                                            ));
                                        menu_open.set(false);
                                    }
                                },
                            }
                            LinkOption {
                                label: tr.form_link_team_username,
                                hint: "team.username",
                                selected: matches!(linked_to, Some(TeamProfileLink::TeamUsername)),
                                on_pick: {
                                    let field_id = field_id_for_link.clone();
                                    move |_| {
                                        on_update
                                            .call((
                                                field_id.clone(),
                                                UpdateSubTeamFormFieldRequest {
                                                    linked_to: Some(TeamProfileLink::TeamUsername),
                                                    ..Default::default()
                                                },
                                            ));
                                        menu_open.set(false);
                                    }
                                },
                            }
                            LinkOption {
                                label: tr.form_link_team_bio,
                                hint: "team.bio",
                                selected: matches!(linked_to, Some(TeamProfileLink::TeamBio)),
                                on_pick: {
                                    let field_id = field_id_for_link.clone();
                                    move |_| {
                                        on_update
                                            .call((
                                                field_id.clone(),
                                                UpdateSubTeamFormFieldRequest {
                                                    linked_to: Some(TeamProfileLink::TeamBio),
                                                    ..Default::default()
                                                },
                                            ));
                                        menu_open.set(false);
                                    }
                                },
                            }
                            LinkOption {
                                label: tr.form_link_team_profile_url,
                                hint: "team.profile_url",
                                selected: matches!(linked_to, Some(TeamProfileLink::TeamProfileUrl)),
                                on_pick: {
                                    let field_id = field_id_for_link.clone();
                                    move |_| {
                                        on_update
                                            .call((
                                                field_id.clone(),
                                                UpdateSubTeamFormFieldRequest {
                                                    linked_to: Some(TeamProfileLink::TeamProfileUrl),
                                                    ..Default::default()
                                                },
                                            ));
                                        menu_open.set(false);
                                    }
                                },
                            }
                        }
                    }
                }

                label { class: "field-row__req",
                    input {
                        r#type: "checkbox",
                        "data-testid": "sub-team-form-field-required-check",
                        checked: required,
                        disabled: locked,
                        onchange: move |e| {
                            if locked {
                                return;
                            }
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
                    disabled: locked,
                    onclick: move |_| {
                        if locked {
                            return;
                        }
                        on_delete.call(field_id_for_delete.clone());
                    },
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

            // ── Options sub-panel (single/multi select only) ───────
            // Renders one input per allowed choice with a remove
            // button, plus a `+ Add option` row that appends an empty
            // entry. All edits autosave via on_update.
            if has_options_panel {
                FieldOptionsPanel {
                    field_id: field_id_for_options_change.clone(),
                    options: options_snapshot,
                    on_change: move |opts: Vec<String>| {
                        on_update
                            .call((
                                field_id_for_options_change.clone(),
                                UpdateSubTeamFormFieldRequest {
                                    options: Some(opts),
                                    ..Default::default()
                                },
                            ));
                    },
                }
            }
        }
    }
}

#[component]
fn FieldOptionsPanel(
    field_id: String,
    options: Vec<String>,
    on_change: EventHandler<Vec<String>>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = field_id;
    rsx! {
        div { class: "field-options",
            div { class: "field-options__list",
                for (idx, opt) in options.iter().enumerate() {
                    div { key: "opt-{idx}", class: "field-options__item",
                        input {
                            class: "field-options__input",
                            r#type: "text",
                            value: "{opt}",
                            placeholder: "{tr.form_options_placeholder}",
                            onchange: {
                                let options = options.clone();
                                move |e| {
                                    let mut next = options.clone();
                                    if idx < next.len() {
                                        next[idx] = e.value();
                                        on_change.call(next);
                                    }
                                }
                            },
                        }
                        button {
                            class: "field-options__del",
                            "aria-label": "Remove option",
                            r#type: "button",
                            onclick: {
                                let options = options.clone();
                                move |_| {
                                    let mut next = options.clone();
                                    if idx < next.len() {
                                        next.remove(idx);
                                        on_change.call(next);
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
            button {
                class: "field-options__add",
                r#type: "button",
                onclick: {
                    let options = options.clone();
                    move |_| {
                        let mut next = options.clone();
                        next.push(String::new());
                        on_change.call(next);
                    }
                },
                "+ {tr.form_options_add}"
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

#[component]
fn LinkOption(
    label: &'static str,
    hint: &'static str,
    selected: bool,
    on_pick: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "field-row__link-option",
            "aria-selected": "{selected}",
            r#type: "button",
            onclick: move |_| on_pick.call(()),
            span { "{label}" }
            span { class: "field-row__link-option-key", "{hint}" }
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "3",
                polyline { points: "20 6 9 17 4 12" }
            }
        }
    }
}

fn link_button_label(link: Option<TeamProfileLink>, tr: &SubTeamTranslate) -> &'static str {
    match link {
        Some(TeamProfileLink::TeamName) => tr.form_link_team_name,
        Some(TeamProfileLink::TeamUsername) => tr.form_link_team_username,
        Some(TeamProfileLink::TeamBio) => tr.form_link_team_bio,
        Some(TeamProfileLink::TeamProfileUrl) => tr.form_link_team_profile_url,
        None => tr.form_link_none,
    }
}

/// Datatag for CSS targeting (`.field-row[data-linked="team.name"] .field-row__link`).
fn link_data_attr(link: Option<TeamProfileLink>) -> &'static str {
    match link {
        Some(TeamProfileLink::TeamName) => "team.name",
        Some(TeamProfileLink::TeamUsername) => "team.username",
        Some(TeamProfileLink::TeamBio) => "team.bio",
        Some(TeamProfileLink::TeamProfileUrl) => "team.profile_url",
        None => "none",
    }
}
