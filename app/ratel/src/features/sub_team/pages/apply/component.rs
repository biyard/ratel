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

    let ctx = use_sub_team_apply()?;
    let UseSubTeamApply {
        mut applicant_team_id,
        parent_team_id,
        apply_context,
        my_teams,
        mut form_values,
        mut agreed_doc_ids,
        mut handle_save_draft,
        ..
    } = ctx;
    let _ = parent_team_id;

    let context = apply_context();
    let form_fields = context.form_fields.clone();
    let required_docs = context.required_docs.clone();
    let my_teams_list = my_teams().items;
    // `TeamItem.pk` is the full `TEAM#uuid` partition string, whereas
    // `applicant_team_id().0` is just the uuid. Compare by parsing
    // both to `TeamPartition` so the lookup actually finds the row.
    let current_applicant = applicant_team_id();
    let selected_team = my_teams_list
        .iter()
        .find(|t| {
            t.pk
                .parse::<TeamPartition>()
                .map(|p| p == current_applicant)
                .unwrap_or(false)
        })
        .cloned();
    let selected_team_name = selected_team
        .as_ref()
        .map(|t| {
            if t.nickname.is_empty() {
                t.username.clone()
            } else {
                t.nickname.clone()
            }
        })
        .unwrap_or_else(|| tr.apply_picker_placeholder.to_string());
    let selected_team_handle = selected_team
        .as_ref()
        .map(|t| format!("@{}", t.username))
        .unwrap_or_default();
    let mut picker_open: Signal<bool> = use_signal(|| false);

    // Linked-field prefill — when applicant team changes, fill any
    // form field whose `linked_to` points at a team profile attribute
    // with the corresponding value from the selected team. Subscribes
    // ONLY to `applicant_team_id` + `my_teams`; reads `form_values`
    // with `.peek()` so its own set() doesn't re-trigger the effect.
    {
        let form_fields_for_prefill = form_fields.clone();
        use_effect(move || {
            let current = applicant_team_id();
            let teams = my_teams().items;
            let Some(team) = teams
                .iter()
                .find(|t| {
                    t.pk
                        .parse::<TeamPartition>()
                        .map(|p| p == current)
                        .unwrap_or(false)
                })
                .cloned()
            else {
                return;
            };
            let mut next = form_values.peek().clone();
            let mut changed = false;
            for f in &form_fields_for_prefill {
                if let Some(link) = f.linked_to {
                    let value = match link {
                        crate::features::sub_team::types::TeamProfileLink::TeamName => {
                            if team.nickname.is_empty() {
                                team.username.clone()
                            } else {
                                team.nickname.clone()
                            }
                        }
                        crate::features::sub_team::types::TeamProfileLink::TeamUsername => {
                            team.username.clone()
                        }
                        crate::features::sub_team::types::TeamProfileLink::TeamBio => {
                            team.description.clone()
                        }
                        crate::features::sub_team::types::TeamProfileLink::TeamProfileUrl => {
                            team.profile_url.clone()
                        }
                    };
                    let prev = next.get(&f.id).and_then(|v| v.as_str()).map(|s| s.to_string());
                    if prev.as_deref() != Some(value.as_str()) {
                        next.insert(f.id.clone(), serde_json::Value::String(value));
                        changed = true;
                    }
                }
            }
            if changed {
                form_values.set(next);
            }
        });
    }

    // Debounced autosave — `bump_save` increments `save_version` from
    // user-input event handlers; the effect waits 2s then fires the
    // save if a fresher edit hasn't arrived. The effect subscribes
    // ONLY to `save_version` — subscribing to `form_values` here
    // makes every programmatic set (draft hydration, linked prefill)
    // re-trigger the autosave and creates a save-loop.
    let mut save_version: Signal<u64> = use_signal(|| 0);
    use_effect(move || {
        let ver = save_version();
        if ver == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(2)).await;
            if save_version() != ver {
                return;
            }
            handle_save_draft.call();
        });
    });
    let mut bump_save = move || {
        *save_version.write() += 1;
    };
    let is_parent_eligible = context.is_parent_eligible;
    let min_sub_team_members = context.min_sub_team_members;
    let min_sub_team_age_days = context.min_sub_team_age_days;

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
    // Only `required` docs gate the submit. Reference-only docs are
    // listed for the applicant to read but never block submission.
    let required_doc_total = required_docs.iter().filter(|d| d.required).count();
    let agreed_count = required_docs
        .iter()
        .filter(|d| d.required && agreed_snapshot.contains_key(&d.id))
        .count();
    let docs_met = required_doc_total == 0 || agreed_count == required_doc_total;
    let form_met = required_filled_count == required_field_count;
    // Eligibility is live-evaluated against the picked applicant team's
    // member_count + created_at — both populated by `list_admin_teams_handler`.
    let applicant_member_count: i64 = selected_team.as_ref().map(|t| t.member_count).unwrap_or(0);
    let applicant_created_at: i64 = selected_team.as_ref().map(|t| t.created_at).unwrap_or(0);
    let min_members_met = if min_sub_team_members <= 0 {
        true
    } else {
        applicant_member_count >= min_sub_team_members as i64
    };
    let min_days_met = if min_sub_team_age_days <= 0 {
        true
    } else if applicant_created_at <= 0 {
        false
    } else {
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let age_days = ((now_ms - applicant_created_at) / 86_400_000).max(0);
        age_days >= min_sub_team_age_days as i64
    };
    // Server filters `list_admin_teams_handler` to admin/owner roles, so
    // any team in the picker is admin-eligible. If the user has zero
    // admin teams the picker shows the empty placeholder and the
    // submit button stays disabled via the `form_met` / docs gates.
    let admin_met = selected_team.is_some();

    // 5-condition live progress (admin, min_members, min_days, docs,
    // form). Mockup's right-rail eligibility panel mirrors this list.
    let met_count = [admin_met, min_members_met, min_days_met, docs_met, form_met]
        .iter()
        .filter(|&&b| b)
        .count();
    let total_count = 5;
    let progress_pct = (met_count as f64 / total_count as f64 * 100.0).round() as i32;
    let eligibility_met = is_parent_eligible && met_count == total_count;

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
        // `.arena` would set height:100vh + overflow:hidden and clip the
        // form — apply is a stacked scroll page, not a viewport-locked
        // arena. The page's own `.arena-topbar` block below is the only
        // topbar now since the route lives OUTSIDE `TeamArenaLayout`.
        div { class: "sub-team-apply",
            // ── Topbar — mockup `arena-topbar` (back + home + brand + status)
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    button {
                        class: "brand-home",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.replace(Route::SocialIndex {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo",
                        {team_display.chars().take(3).collect::<String>().to_uppercase()}
                    }
                    div { class: "u-col",
                        span { class: "arena-topbar__title", "{team_display}" }
                        span { class: "arena-topbar__handle", "{tr.apply_page_eyebrow}" }
                    }
                    span { class: "arena-topbar__status arena-topbar__status--scheduled",
                        "{tr.apply_status_drafting}"
                    }
                }
            }

            div { class: "page page--wide",
                // ── Page header (mockup hero) — eyebrow + title + sub
                div { class: "page-header",
                    div { class: "page-header__main",
                        span { class: "page-header__eyebrow",
                            lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.apply_page_eyebrow}"
                        }
                        h1 { class: "page-header__title",
                            strong { "{team_display}" }
                            "{tr.apply_target_label}"
                        }
                        p { class: "page-header__sub", "{tr.apply_page_header_sub}" }
                    }
                }

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

                // Applicant team picker — lists every team the current
                // user belongs to (via `my_teams` loader). The selected
                // team becomes the path-param `team_pk` for the submit
                // call, and its profile attributes auto-fill any
                // linked fields below.
                div { class: "team-picker",
                    label { class: "team-picker__label", "{tr.apply_pick_your_team}" }
                    div { class: "team-dropdown",
                        button {
                            class: "team-dropdown__trigger",
                            "data-testid": "sub-team-apply-picker-trigger",
                            r#type: "button",
                            "aria-expanded": "{picker_open()}",
                            onclick: move |_| picker_open.toggle(),
                            div { class: "team-dropdown__body",
                                span { class: "team-dropdown__name", "{selected_team_name}" }
                                if !selected_team_handle.is_empty() {
                                    span { class: "team-dropdown__meta", "{selected_team_handle}" }
                                }
                            }
                            lucide_dioxus::ChevronDown { class: "w-4 h-4 [&>path]:stroke-current team-dropdown__chev" }
                        }
                        if picker_open() && !my_teams_list.is_empty() {
                            div {
                                class: "team-dropdown__menu",
                                "data-open": "true",
                                role: "listbox",
                                for team in my_teams_list.iter() {
                                    {
                                        let is_selected = team
                                            .pk
                                            .parse::<TeamPartition>()
                                            .map(|p| p == current_applicant)
                                            .unwrap_or(false);
                                        let team_name = if team.nickname.is_empty() {
                                            team.username.clone()
                                        } else {
                                            team.nickname.clone()
                                        };
                                        let team_handle = team.username.clone();
                                        let team_pk_str = team.pk.clone();
                                        rsx! {
                                            button {
                                                key: "{team.pk}",
                                                r#type: "button",
                                                class: "team-dropdown__item",
                                                "data-testid": "sub-team-apply-picker-item-{team_handle}",
                                                "aria-selected": "{is_selected}",
                                                onclick: move |_| {
                                                    if let Ok(pk) = team_pk_str.parse::<TeamPartition>() {
                                                        applicant_team_id.set(pk);
                                                    }
                                                    picker_open.set(false);
                                                },
                                                div { class: "team-dropdown__body",
                                                    span { class: "team-dropdown__name", "{team_name}" }
                                                    span { class: "team-dropdown__meta", "@{team_handle}" }
                                                }
                                                if is_selected {
                                                    span { class: "team-dropdown__check",
                                                        lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !is_parent_eligible {
                    div { class: "notice notice--warn", "{tr.apply_parent_eligible_off}" }
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
                                        "{agreed_count} / {required_doc_total}"
                                    }
                                }
                                for (idx, doc) in required_docs.iter().enumerate() {
                                    {
                                        let agreed = agreed_snapshot.contains_key(&doc.id);
                                        let doc_title = doc.title.clone();
                                        let is_required = doc.required;
                                        rsx! {
                                            button {
                                                key: "{doc.id}",
                                                r#type: "button",
                                                class: "req-doc",
                                                "data-agreed": "{agreed}",
                                                "data-required": "{is_required}",
                                                "data-id": "{doc.id}",
                                                "data-testid": "sub-team-apply-req-doc",
                                                onclick: move |_| {
                                                    active_doc_idx.set(Some(idx));
                                                },
                                                span { class: "req-doc__badge",
                                                    if is_required {
                                                        "{tr.bylaws_required_badge}"
                                                    } else {
                                                        "{tr.apply_doc_reference_badge}"
                                                    }
                                                }
                                                span { class: "req-doc__title", "{doc_title}" }
                                                span { class: "req-doc__status",
                                                    if is_required {
                                                        if agreed {
                                                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                                            "{tr.apply_docs_agreed}"
                                                        } else {
                                                            lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                                                            "{tr.apply_docs_open_review}"
                                                        }
                                                    } else {
                                                        lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                                                        "{tr.apply_doc_view_open}"
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
                            // Composer meta — "{applicant} → {parent}" arrow
                            // flow (mockup §composer-meta). Applicant slot
                            // Applicant → parent flow indicator. Left
                            // slot shows the currently-picked applicant
                            // team (from `my_teams` API); right slot is
                            // the parent team being applied to.
                            div { class: "composer-meta",
                                div { class: "composer-meta__selected",
                                    span { "{selected_team_name}" }
                                }
                                span { class: "composer-meta__arrow",
                                    lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                }
                                div { class: "composer-meta__selected",
                                    span { class: "composer-meta__target", "{team_display}" }
                                }
                            }
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
                                        bump_save();
                                    },
                                }
                            }
                        }

                        // Sticky submit bar — lives inside `.composer-col`
                        // so its width matches the form column (not the
                        // full page including the right eligibility rail).
                        div { class: "submit-bar",
                            div { class: "submit-bar__status",
                                div { class: if eligibility_met { "submit-bar__title submit-bar__title--ready" } else { "submit-bar__title" },
                                    "{tr.apply_submit_progress_prefix} {met_count} / {total_count} {tr.apply_submit_progress_suffix}"
                                }
                                div { class: "submit-bar__sub",
                                    if eligibility_met {
                                        "{tr.apply_submit}"
                                    } else {
                                        "{tr.apply_submit_sub}"
                                    }
                                }
                            }
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| {
                                    nav.replace(Route::SocialIndex {
                                        username: username_for_status.clone(),
                                    });
                                },
                                "{tr.cancel}"
                            }
                            button {
                                class: "btn btn--primary",
                                id: "submit-btn",
                                "data-testid": "sub-team-apply-submit-btn",
                                disabled: !eligibility_met,
                                onclick: {
                                    let username_for_submit = username.clone();
                                    move |_| {
                                        let username = username_for_submit.clone();
                                        async move {
                                            if !eligibility_met {
                                                return;
                                            }
                                            if ctx.submit().await.is_ok() {
                                                nav.replace(Route::TeamSubTeamApplicationStatusPage {
                                                    username,
                                                });
                                            }
                                        }
                                    }
                                },
                                lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                                "{tr.apply_submit}"
                            }
                        }
                    }

                    // Right: eligibility panel (mockup: progress + 5 items
                    // with description + status text).
                    aside { class: "eligibility-col",
                        div { class: "eligibility-panel",
                            div { class: "eligibility-panel__title",
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                "{tr.apply_eligibility_title}"
                            }

                            // Progress bar — fraction + filled track
                            div { class: "elig-progress",
                                div { class: "elig-progress__top",
                                    span { "{tr.apply_progress_label}" }
                                    span { class: if eligibility_met { "elig-progress__fraction elig-progress__fraction--ready" } else { "elig-progress__fraction" },
                                        "{met_count} / {total_count}"
                                    }
                                }
                                div { class: "elig-progress__track",
                                    div {
                                        class: "elig-progress__fill",
                                        style: "width:{progress_pct}%",
                                        "data-ready": "{eligibility_met}",
                                    }
                                }
                            }

                            div { class: "elig-list",
                                EligibilityItem {
                                    met: admin_met,
                                    title: tr.apply_elig_admin_title.to_string(),
                                    desc: tr.apply_elig_admin_desc.to_string(),
                                }
                                EligibilityItem {
                                    met: min_members_met,
                                    title: tr.apply_elig_min_members.to_string(),
                                    desc: tr.apply_elig_min_members_desc.to_string(),
                                }
                                EligibilityItem {
                                    met: min_days_met,
                                    title: tr.apply_elig_min_days_title.to_string(),
                                    desc: tr.apply_elig_min_days_desc.to_string(),
                                }
                                EligibilityItem {
                                    met: docs_met,
                                    title: tr.apply_elig_docs_agreed.to_string(),
                                    desc: tr.apply_elig_docs_desc.to_string(),
                                }
                                EligibilityItem {
                                    met: form_met,
                                    title: tr.apply_elig_form_filled.to_string(),
                                    desc: tr.apply_elig_form_desc.to_string(),
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
                    bump_save();
                }
                active_doc_idx.set(None);
            },
        }
    }
}

#[component]
fn EligibilityItem(met: bool, title: String, desc: String) -> Element {
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
                div { class: "elig-item__desc", "{desc}" }
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
    let tr: SubTeamTranslate = use_translate();
    let id = field.id.clone();
    let label = field.label.clone();
    let required = field.required;
    let field_type = field.field_type;
    let options = field.options.clone();
    // When `linked_to` is set the value will be populated server-side
    // at submit time from the applicant team's profile, so the input
    // is rendered read-only with a 🔗 hint. The actual prefill on the
    // client (echoing the value back into the field for visual feedback)
    // requires loading the applicant team profile here — deferred.
    let linked_to = field.linked_to;
    let is_linked = linked_to.is_some();
    let linked_hint: Option<&'static str> = match linked_to {
        Some(crate::features::sub_team::types::TeamProfileLink::TeamName) => {
            Some(tr.form_link_team_name)
        }
        Some(crate::features::sub_team::types::TeamProfileLink::TeamUsername) => {
            Some(tr.form_link_team_username)
        }
        Some(crate::features::sub_team::types::TeamProfileLink::TeamBio) => {
            Some(tr.form_link_team_bio)
        }
        Some(crate::features::sub_team::types::TeamProfileLink::TeamProfileUrl) => {
            Some(tr.form_link_team_profile_url)
        }
        None => None,
    };

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
            "data-linked": "{is_linked}",
            label { class: "field__label",
                "{label}"
                if required {
                    span { class: "req", " *" }
                }
                if let Some(hint) = linked_hint {
                    span { class: "field__linked-hint", " · 🔗 {hint}" }
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
