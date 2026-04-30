//! Apply-as-sub-team page.
//!
//! Mirrors `assets/design/sub-team/subteam-apply.html`. Consumes
//! `UseSubTeamApply` — the page only needs to wire form inputs to the
//! `form_values` signal, doc-agreement clicks to the modal + hook
//! signals, and the Submit button to `handle_submit.call(())`.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::models::SubTeamFormFieldType;
use crate::features::sub_team::{
    use_sub_team_apply, ApplyContextDocument, SubTeamFormFieldResponse, SubTeamTranslate,
    UseSubTeamApply,
};
use crate::route::Route;
use crate::*;

use super::DocAgreementModal;

#[component]
pub fn TeamSubTeamApplyPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let team_display = if team_data.nickname.is_empty() {
        team_data.username.clone()
    } else {
        team_data.nickname.clone()
    };
    let team_handle = team_data.username.clone();
    let team_pk = team_data.pk.clone();
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);

    rsx! {
        SeoMeta { title: "{tr.apply_page_title}" }
        ApplyForm {
            username: username.clone(),
            team_display: team_display.clone(),
            team_handle: team_handle.clone(),
        }
    }
}

#[component]
fn ApplyForm(username: String, team_display: String, team_handle: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamApply {
        mut parent_team_id,
        apply_context,
        mut form_values,
        mut agreed_doc_ids,
        mut handle_submit,
        ..
    } = use_sub_team_apply()?;

    let context = apply_context();
    let form_fields = context.form_fields.clone();
    let required_docs = context.required_docs.clone();
    let is_parent_eligible = context.is_parent_eligible;
    let min_sub_team_members = context.min_sub_team_members;

    // Modal state: which doc (if any) is open
    let mut active_doc_idx: Signal<Option<usize>> = use_signal(|| None);

    // Eligibility calculations
    let required_field_count = form_fields.iter().filter(|f| f.required).count();
    let values_snapshot = form_values.read().clone();
    let required_filled_count = form_fields
        .iter()
        .filter(|f| {
            if !f.required {
                return false;
            }
            match values_snapshot.get(&f.id) {
                Some(serde_json::Value::String(s)) => !s.trim().is_empty(),
                Some(serde_json::Value::Array(a)) => !a.is_empty(),
                Some(serde_json::Value::Number(_)) => true,
                Some(serde_json::Value::Bool(b)) => *b,
                Some(serde_json::Value::Null) | None => false,
                _ => true,
            }
        })
        .count();
    let agreed_snapshot = agreed_doc_ids.read().clone();
    let agreed_count = required_docs
        .iter()
        .filter(|d| agreed_snapshot.contains_key(&d.id))
        .count();
    let docs_met = required_docs.is_empty() || agreed_count == required_docs.len();
    let form_met = required_filled_count == required_field_count;
    let min_members_met = min_sub_team_members <= 0;
    let eligibility_met =
        is_parent_eligible && docs_met && form_met && min_members_met;

    let doc_agreed_snapshot = agreed_snapshot.clone();
    let required_docs_for_modal = required_docs.clone();

    let selected_doc = active_doc_idx().and_then(|i| required_docs_for_modal.get(i).cloned());
    let modal_open = selected_doc.is_some();
    let modal_doc = selected_doc.clone().unwrap_or_default();
    let modal_already_agreed = selected_doc
        .as_ref()
        .map(|d| doc_agreed_snapshot.contains_key(&d.id))
        .unwrap_or(false);
    let modal_index = active_doc_idx().map(|i| i + 1).unwrap_or(0);
    let modal_total = required_docs.len();

    let username_for_back = username.clone();
    let username_for_status = username.clone();

    rsx! {
        div { class: "arena sub-team-apply",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.push(Route::TeamSubTeamApplicationStatusPage {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.apply_page_eyebrow}" }
                        span { class: "topbar-title__main", "{team_display}" }
                    }
                }
            }

            div { class: "page page--wide",
                // Target summary
                div { class: "target-summary",
                    div { class: "target-summary__avatar",
                        {team_display.chars().take(3).collect::<String>().to_uppercase()}
                    }
                    div { class: "target-summary__body",
                        div { class: "target-summary__label", "{tr.apply_target_label}" }
                        div { class: "target-summary__name", "{team_display}" }
                        div { class: "target-summary__handle", "@{team_handle}" }
                    }
                }

                // Parent team id picker (simple input — a full admin-team
                // dropdown is a Phase-2 enhancement).
                div { class: "team-picker",
                    label { class: "team-picker__label", "{tr.apply_select_parent}" }
                    input {
                        r#type: "text",
                        class: "team-picker__input",
                        id: "parent-team-input",
                        "data-testid": "sub-team-apply-parent-input",
                        placeholder: "team-pk",
                        value: "{parent_team_id()}",
                        oninput: move |e| parent_team_id.set(e.value()),
                    }
                }

                if !is_parent_eligible && !parent_team_id().is_empty() {
                    div { class: "notice notice--warn",
                        "{tr.apply_parent_eligible_off}"
                    }
                }

                div { class: "apply-grid",
                    // Left: composer + form
                    div { class: "composer-col",

                        // Required documents
                        if !required_docs.is_empty() {
                            div { class: "req-docs", id: "req-docs",
                                div { class: "req-docs__head",
                                    span { class: "req-docs__title",
                                        lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                                        "{tr.apply_required_docs}"
                                    }
                                    span {
                                        class: "req-docs__progress",
                                        "data-all-read": "{docs_met}",
                                        "{agreed_count} / {required_docs.len()}"
                                    }
                                }
                                for (idx, doc) in required_docs.iter().enumerate() {
                                    {
                                        let agreed = agreed_snapshot.contains_key(&doc.id);
                                        let doc_title = doc.title.clone();
                                        rsx! {
                                            button {
                                                key: "{doc.id}",
                                                r#type: "button",
                                                class: "req-doc",
                                                "data-agreed": "{agreed}",
                                                "data-id": "{doc.id}",
                                                "data-testid": "sub-team-apply-req-doc",
                                                onclick: move |_| {
                                                    active_doc_idx.set(Some(idx));
                                                },
                                                span { class: "req-doc__badge", "{tr.bylaws_required_badge}" }
                                                span { class: "req-doc__title", "{doc_title}" }
                                                span { class: "req-doc__status",
                                                    if agreed {
                                                        lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                                        "{tr.apply_docs_agreed}"
                                                    } else {
                                                        lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                                                        "{tr.apply_docs_open_review}"
                                                    }
                                                }
                                                span { class: "req-doc__chev",
                                                    lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Form fields
                        div { class: "composer-card",
                            div { class: "composer-card__title",
                                lucide_dioxus::ListChecks { class: "w-3 h-3 [&>path]:stroke-current" }
                                "{tr.apply_form_fields}"
                            }
                            for field in form_fields.iter() {
                                FieldRow {
                                    key: "{field.id}",
                                    field: field.clone(),
                                    value: values_snapshot.get(&field.id).cloned().unwrap_or_default(),
                                    on_change: move |(id, v): (String, serde_json::Value)| {
                                        let mut map = form_values.read().clone();
                                        map.insert(id, v);
                                        form_values.set(map);
                                    },
                                }
                            }
                        }
                    }

                    // Right: eligibility panel
                    aside { class: "eligibility-col",
                        div { class: "eligibility-panel",
                            div { class: "eligibility-panel__title",
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                "{tr.apply_eligibility_title}"
                            }
                            div { class: "elig-list",
                                EligibilityItem {
                                    met: is_parent_eligible,
                                    title: tr.settings_is_parent_eligible.to_string(),
                                }
                                EligibilityItem {
                                    met: min_members_met,
                                    title: tr.apply_elig_min_members.to_string(),
                                }
                                EligibilityItem {
                                    met: form_met,
                                    title: tr.apply_elig_form_filled.to_string(),
                                }
                                EligibilityItem {
                                    met: docs_met,
                                    title: tr.apply_elig_docs_agreed.to_string(),
                                }
                            }
                            div { class: "submit-bar",
                                div { class: "submit-bar__status",
                                    div {
                                        class: "submit-bar__title",
                                        "data-ready": "{eligibility_met}",
                                        if eligibility_met {
                                            "{tr.apply_submit}"
                                        } else {
                                            "{tr.apply_submit_sub}"
                                        }
                                    }
                                }
                                button {
                                    class: "btn btn--primary",
                                    id: "submit-btn",
                                    "data-testid": "sub-team-apply-submit-btn",
                                    disabled: !eligibility_met,
                                    onclick: move |_| {
                                        if !eligibility_met {
                                            return;
                                        }
                                        handle_submit.call();
                                        nav.push(Route::TeamSubTeamApplicationStatusPage {
                                            username: username_for_status.clone(),
                                        });
                                    },
                                    lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                                    "{tr.apply_submit}"
                                }
                            }
                        }
                    }
                }
            }
        }

        DocAgreementModal {
            open: modal_open,
            doc: modal_doc.clone(),
            already_agreed: modal_already_agreed,
            index: modal_index,
            total: modal_total,
            on_cancel: move |_| {
                active_doc_idx.set(None);
            },
            on_agree: move |_| {
                if let Some(doc) = selected_doc.clone() {
                    let mut map = agreed_doc_ids.read().clone();
                    map.insert(doc.id.clone(), doc.body_hash.clone());
                    agreed_doc_ids.set(map);
                }
                active_doc_idx.set(None);
            },
        }
    }
}

#[component]
fn EligibilityItem(met: bool, title: String) -> Element {
    rsx! {
        div { class: "elig-item", "data-met": "{met}",
            div { class: "elig-item__check",
                if met {
                    lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                } else {
                    lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                }
            }
            div { class: "elig-item__body",
                div { class: "elig-item__title", "{title}" }
            }
        }
    }
}

#[component]
fn FieldRow(
    field: SubTeamFormFieldResponse,
    value: serde_json::Value,
    on_change: EventHandler<(String, serde_json::Value)>,
) -> Element {
    let id = field.id.clone();
    let label = field.label.clone();
    let required = field.required;
    let field_type = field.field_type;
    let options = field.options.clone();

    let text_value = match &value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        _ => String::new(),
    };
    let selected_values: Vec<String> = match &value {
        serde_json::Value::Array(a) => a
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    };

    let id_for_input = id.clone();
    let id_for_select = id.clone();
    let id_for_multi = id.clone();
    let id_for_textarea = id.clone();
    let id_for_number = id.clone();
    let id_for_date = id.clone();

    rsx! {
        div {
            class: "field",
            "data-testid": "sub-team-apply-field",
            label { class: "field__label",
                "{label}"
                if required {
                    span { class: "req", " *" }
                }
            }
            match field_type {
                SubTeamFormFieldType::LongText => rsx! {
                    textarea {
                        class: "field__textarea",
                        value: "{text_value}",
                        oninput: move |e| {
                            on_change
                                .call((
                                    id_for_textarea.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                    }
                },
                SubTeamFormFieldType::Number => rsx! {
                    input {
                        class: "field__input",
                        r#type: "number",
                        value: "{text_value}",
                        oninput: move |e| {
                            on_change
                                .call((
                                    id_for_number.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                    }
                },
                SubTeamFormFieldType::Date => rsx! {
                    input {
                        class: "field__input",
                        r#type: "date",
                        value: "{text_value}",
                        oninput: move |e| {
                            on_change
                                .call((
                                    id_for_date.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                    }
                },
                SubTeamFormFieldType::Url => rsx! {
                    input {
                        class: "field__input",
                        r#type: "url",
                        value: "{text_value}",
                        oninput: move |e| {
                            on_change
                                .call((
                                    id_for_input.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                    }
                },
                SubTeamFormFieldType::SingleSelect => rsx! {
                    select {
                        class: "field__select",
                        value: "{text_value}",
                        onchange: move |e| {
                            on_change
                                .call((
                                    id_for_select.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                        option { value: "", "—" }
                        for opt in options.iter() {
                            option { key: "{opt}", value: "{opt}", "{opt}" }
                        }
                    }
                },
                SubTeamFormFieldType::MultiSelect => rsx! {
                    div { class: "field__multi",
                        for opt in options.iter() {
                            {
                                let opt_val = opt.clone();
                                let selected = selected_values.contains(&opt_val);
                                let id_clone = id_for_multi.clone();
                                let selected_clone = selected_values.clone();
                                rsx! {
                                    label { key: "{opt}", class: "field__checkbox",
                                        input {
                                            r#type: "checkbox",
                                            checked: selected,
                                            onchange: move |e| {
                                                let mut vs = selected_clone.clone();
                                                if e.checked() {
                                                    if !vs.contains(&opt_val) {
                                                        vs.push(opt_val.clone());
                                                    }
                                                } else {
                                                    vs.retain(|v| v != &opt_val);
                                                }
                                                let arr: Vec<serde_json::Value> = vs
                                                    .into_iter()
                                                    .map(serde_json::Value::String)
                                                    .collect();
                                                on_change
                                                    .call((
                                                        id_clone.clone(),
                                                        serde_json::Value::Array(arr),
                                                    ));
                                            },
                                        }
                                        span { "{opt}" }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    input {
                        class: "field__input",
                        r#type: "text",
                        value: "{text_value}",
                        oninput: move |e| {
                            on_change
                                .call((
                                    id_for_input.clone(),
                                    serde_json::Value::String(e.value()),
                                ));
                        },
                    }
                },
            }
        }
    }
}

// Silence dead-code warnings on the re-exported doc type.
const _: fn() -> ApplyContextDocument = ApplyContextDocument::default;
