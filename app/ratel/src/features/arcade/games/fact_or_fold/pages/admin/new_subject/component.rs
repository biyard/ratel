use crate::features::arcade::games::fact_or_fold::hooks::{
    UseFactFoldAdminSubjects, use_fact_fold_admin_subjects_provider,
};
use crate::features::arcade::games::fact_or_fold::types::{
    CreateSubjectRequest, HEADLINE_BODY_MAX, HEADLINE_DIFFICULTY_MAX, HEADLINE_DIFFICULTY_MIN,
    HEADLINE_TEXT_MAX, REVEAL_SOURCES_MAX, RevealSource, Verdict,
};
use crate::route::Route;
use crate::*;

use super::i18n::FactFoldAdminNewSubjectTranslate;

/// `/admin/fact-or-fold/subjects/new` — author a new draft (or
/// schedule it directly). On submit, navigates back to the list.
#[component]
pub fn FactFoldAdminNewSubjectPage() -> Element {
    let tr: FactFoldAdminNewSubjectTranslate = use_translate();
    let UseFactFoldAdminSubjects { .. } = use_fact_fold_admin_subjects_provider()?;
    let mut ctx = use_fact_fold_admin_subjects_provider()?;
    let nav = use_navigator();

    // ── Form signals ──────────────────────────────────────────────
    let verdict = use_signal(|| Verdict::Real);
    let headline_text = use_signal(String::new);
    let body_excerpt = use_signal(String::new);
    let difficulty = use_signal(|| 3i32);
    let category_tags_raw = use_signal(String::new); // comma-separated
    let source_label = use_signal(String::new);
    let insider_statement = use_signal(String::new);
    let reveal_summary = use_signal(String::new);
    let reveal_sources = use_signal(Vec::<RevealSource>::new);
    let scheduled_at_iso = use_signal(String::new); // ISO yyyy-mm-ddTHH:MM (datetime-local)

    let mut submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let body_len = body_excerpt().chars().count();
    let body_in_range = !body_excerpt().trim().is_empty() && body_len <= HEADLINE_BODY_MAX;
    let subject_in_range =
        !headline_text().trim().is_empty() && headline_text().len() <= HEADLINE_TEXT_MAX;
    let insider_filled = !insider_statement().trim().is_empty();
    let summary_filled = !reveal_summary().trim().is_empty();
    let source_filled = !source_label().trim().is_empty();
    let difficulty_in_range =
        (HEADLINE_DIFFICULTY_MIN..=HEADLINE_DIFFICULTY_MAX).contains(&difficulty());

    // Submit-now is allowed for "save draft" even with looser fields,
    // but to be safe and surface server errors early we require the
    // same rules the publish endpoint enforces. Save Draft path
    // stores `scheduled_at = None` which keeps status=Draft.
    let core_ok = subject_in_range
        && body_in_range
        && difficulty_in_range
        && insider_filled
        && summary_filled
        && source_filled;

    let make_request = move |with_schedule: bool| -> Option<CreateSubjectRequest> {
        let scheduled_ts = if with_schedule {
            parse_iso_local_to_millis(&scheduled_at_iso())?
        } else {
            return Some(CreateSubjectRequest {
                headline_text: headline_text(),
                body_excerpt: body_excerpt(),
                verdict: verdict(),
                difficulty: difficulty(),
                category_tags: parse_csv(&category_tags_raw()),
                source_label: source_label(),
                insider_statement: insider_statement(),
                reveal_summary: reveal_summary(),
                reveal_sources: reveal_sources(),
                scheduled_at: None,
            });
        };
        Some(CreateSubjectRequest {
            headline_text: headline_text(),
            body_excerpt: body_excerpt(),
            verdict: verdict(),
            difficulty: difficulty(),
            category_tags: parse_csv(&category_tags_raw()),
            source_label: source_label(),
            insider_statement: insider_statement(),
            reveal_summary: reveal_summary(),
            reveal_sources: reveal_sources(),
            scheduled_at: Some(scheduled_ts),
        })
    };

    let save_draft = move |_| async move {
        if !core_ok {
            return;
        }
        submitting.set(true);
        error_msg.set(None);
        let req = match make_request(false) {
            Some(r) => r,
            None => {
                submitting.set(false);
                return;
            }
        };
        match ctx.create(req).await {
            Ok(_) => {
                nav.push(Route::FactFoldAdminSubjectsPage {});
            }
            Err(e) => error_msg.set(Some(format!("{e}"))),
        }
        submitting.set(false);
    };

    let schedule_publish = move |_| async move {
        if !core_ok {
            return;
        }
        submitting.set(true);
        error_msg.set(None);
        let req = match make_request(true) {
            Some(r) => r,
            None => {
                error_msg.set(Some(
                    "Pick a future schedule time first.".to_string(),
                ));
                submitting.set(false);
                return;
            }
        };
        match ctx.create(req).await {
            Ok(_) => {
                nav.push(Route::FactFoldAdminSubjectsPage {});
            }
            Err(e) => error_msg.set(Some(format!("{e}"))),
        }
        submitting.set(false);
    };

    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        form {
            class: "ff-new-subject",
            onsubmit: move |e| {
                e.prevent_default();
            },

            // Section 01 — verdict + difficulty
            FormSection {
                title: "{tr.section_truth_title}",
                sub: "{tr.section_truth_sub}",
                VerdictPicker { value: verdict }
                DifficultyPicker { value: difficulty }
            }

            // Section 02 — subject text + body
            FormSection {
                title: "{tr.section_text_title}",
                sub: "{tr.section_text_sub}",
                TextInputField {
                    label: "{tr.headline_text}",
                    placeholder: "{tr.headline_text_placeholder}",
                    value: headline_text,
                    counter: format!("{} / {}", headline_text().len(), HEADLINE_TEXT_MAX),
                    invalid: !subject_in_range && !headline_text().is_empty(),
                }
                TextAreaField {
                    label: "{tr.body_excerpt}",
                    placeholder: "{tr.body_excerpt_placeholder}",
                    value: body_excerpt,
                    rows: 6,
                    counter: format!("{} / {}", body_len, HEADLINE_BODY_MAX),
                    invalid: !body_in_range && !body_excerpt().is_empty(),
                }
            }

            // Section 03 — taxonomy
            FormSection {
                title: "{tr.section_meta_title}",
                sub: "{tr.section_meta_sub}",
                TextInputField {
                    label: "{tr.source_label}",
                    placeholder: "{tr.source_label_placeholder}",
                    value: source_label,
                    counter: String::new(),
                    invalid: false,
                }
                TextInputField {
                    label: "{tr.category_tags}",
                    placeholder: "{tr.category_tags_placeholder}",
                    value: category_tags_raw,
                    counter: String::new(),
                    invalid: false,
                }
            }

            // Section 04 — insider statement
            FormSection {
                title: "{tr.section_insider_title}",
                sub: "{tr.section_insider_sub}",
                TextAreaField {
                    label: "{tr.insider_statement}",
                    placeholder: "{tr.insider_statement_placeholder}",
                    value: insider_statement,
                    rows: 4,
                    counter: String::new(),
                    invalid: false,
                }
                p { class: "ff-new-subject__hint", "{tr.insider_hint}" }
            }

            // Section 05 — reveal
            FormSection {
                title: "{tr.section_reveal_title}",
                sub: "{tr.section_reveal_sub}",
                TextAreaField {
                    label: "{tr.reveal_summary}",
                    placeholder: "{tr.reveal_summary_placeholder}",
                    value: reveal_summary,
                    rows: 3,
                    counter: String::new(),
                    invalid: false,
                }
                RevealSourcesEditor { value: reveal_sources }
            }

            // Section 06 — schedule + submit
            FormSection {
                title: "{tr.section_publish_title}",
                sub: "{tr.section_publish_sub}",
                ScheduleField { value: scheduled_at_iso }
                div { class: "ff-new-subject__actions",
                    if let Some(err) = error_msg() {
                        span { class: "ff-new-subject__error", "{err}" }
                    }
                    if !core_ok {
                        span { class: "ff-new-subject__hint", "{tr.fields_incomplete}" }
                    }
                    button {
                        class: "btn btn--ghost",
                        disabled: submitting() || !core_ok,
                        onclick: save_draft,
                        "{tr.save_draft}"
                    }
                    button {
                        class: "btn btn--primary",
                        disabled: submitting() || !core_ok || scheduled_at_iso().is_empty(),
                        onclick: schedule_publish,
                        "{tr.schedule_publish}"
                    }
                }
            }
        }
    }
}

// ── Sub-components ────────────────────────────────────────────────

#[component]
fn FormSection(title: String, sub: String, children: Element) -> Element {
    rsx! {
        section { class: "ff-new-subject__section",
            header { class: "ff-new-subject__section-head",
                span { class: "ff-new-subject__section-title", "{title}" }
                span { class: "ff-new-subject__section-sub", "{sub}" }
            }
            div { class: "ff-new-subject__panel", {children} }
        }
    }
}

#[component]
fn VerdictPicker(value: Signal<Verdict>) -> Element {
    let tr: FactFoldAdminNewSubjectTranslate = use_translate();
    let mut value = value;
    let cur = value();
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label", "{tr.verdict_label}" }
            div { class: "ff-new-subject__verdict-row",
                button {
                    r#type: "button",
                    class: "ff-new-subject__verdict-btn",
                    "data-variant": "real",
                    "aria-selected": matches!(cur, Verdict::Real),
                    onclick: move |_| value.set(Verdict::Real),
                    "REAL"
                }
                button {
                    r#type: "button",
                    class: "ff-new-subject__verdict-btn",
                    "data-variant": "fake",
                    "aria-selected": matches!(cur, Verdict::Fake),
                    onclick: move |_| value.set(Verdict::Fake),
                    "FAKE"
                }
            }
            div { class: "ff-new-subject__hint", "{tr.verdict_hint}" }
        }
    }
}

#[component]
fn DifficultyPicker(value: Signal<i32>) -> Element {
    let tr: FactFoldAdminNewSubjectTranslate = use_translate();
    let mut value = value;
    let cur = value();
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label", "{tr.difficulty_label}" }
            div { class: "ff-new-subject__star-row",
                for star in HEADLINE_DIFFICULTY_MIN..=HEADLINE_DIFFICULTY_MAX {
                    button {
                        r#type: "button",
                        class: "ff-new-subject__star",
                        "aria-selected": star <= cur,
                        onclick: move |_| value.set(star),
                        "★"
                    }
                }
                span { class: "ff-new-subject__star-label", "{cur} / {HEADLINE_DIFFICULTY_MAX}" }
            }
        }
    }
}

#[component]
fn TextInputField(
    label: String,
    placeholder: String,
    value: Signal<String>,
    counter: String,
    invalid: bool,
) -> Element {
    let mut value = value;
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label-row",
                div { class: "ff-new-subject__label", "{label}" }
                if !counter.is_empty() {
                    span {
                        class: "ff-new-subject__counter",
                        "data-invalid": invalid,
                        "{counter}"
                    }
                }
            }
            input {
                class: "ff-new-subject__input",
                "data-invalid": invalid,
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| value.set(e.value()),
            }
        }
    }
}

#[component]
fn TextAreaField(
    label: String,
    placeholder: String,
    value: Signal<String>,
    rows: i32,
    counter: String,
    invalid: bool,
) -> Element {
    let mut value = value;
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label-row",
                div { class: "ff-new-subject__label", "{label}" }
                if !counter.is_empty() {
                    span {
                        class: "ff-new-subject__counter",
                        "data-invalid": invalid,
                        "{counter}"
                    }
                }
            }
            textarea {
                class: "ff-new-subject__textarea",
                "data-invalid": invalid,
                rows: "{rows}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| value.set(e.value()),
            }
        }
    }
}

#[component]
fn RevealSourcesEditor(value: Signal<Vec<RevealSource>>) -> Element {
    let tr: FactFoldAdminNewSubjectTranslate = use_translate();
    let mut value = value;
    let items = value();
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label-row",
                div { class: "ff-new-subject__label", "{tr.reveal_sources}" }
                span { class: "ff-new-subject__counter", "{items.len()} / {REVEAL_SOURCES_MAX}" }
            }
            div { class: "ff-new-subject__sources",
                for (idx, src) in items.iter().enumerate() {
                    div { class: "ff-new-subject__source-row", key: "{idx}",
                        input {
                            class: "ff-new-subject__input ff-new-subject__source-label",
                            r#type: "text",
                            placeholder: "{tr.reveal_source_label_placeholder}",
                            value: "{src.label}",
                            oninput: move |e| {
                                let mut next = value();
                                if let Some(item) = next.get_mut(idx) {
                                    item.label = e.value();
                                }
                                value.set(next);
                            },
                        }
                        input {
                            class: "ff-new-subject__input ff-new-subject__source-url",
                            r#type: "url",
                            placeholder: "https://…",
                            value: "{src.url}",
                            oninput: move |e| {
                                let mut next = value();
                                if let Some(item) = next.get_mut(idx) {
                                    item.url = e.value();
                                }
                                value.set(next);
                            },
                        }
                        button {
                            r#type: "button",
                            class: "ff-new-subject__source-remove",
                            onclick: move |_| {
                                let mut next = value();
                                if idx < next.len() {
                                    next.remove(idx);
                                }
                                value.set(next);
                            },
                            "✕"
                        }
                    }
                }
                if items.len() < REVEAL_SOURCES_MAX {
                    button {
                        r#type: "button",
                        class: "ff-new-subject__source-add",
                        onclick: move |_| {
                            let mut next = value();
                            next.push(RevealSource::default());
                            value.set(next);
                        },
                        "+ {tr.reveal_source_add}"
                    }
                }
            }
        }
    }
}

#[component]
fn ScheduleField(value: Signal<String>) -> Element {
    let tr: FactFoldAdminNewSubjectTranslate = use_translate();
    let mut value = value;
    rsx! {
        div { class: "ff-new-subject__field",
            div { class: "ff-new-subject__label", "{tr.schedule_label}" }
            input {
                class: "ff-new-subject__input",
                r#type: "datetime-local",
                value: "{value}",
                oninput: move |e| value.set(e.value()),
            }
            div { class: "ff-new-subject__hint", "{tr.schedule_hint}" }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────

fn parse_csv(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse `<input type="datetime-local">` value (`YYYY-MM-DDTHH:MM`)
/// into UTC milliseconds. Treats the input as local time and folds
/// it through the JS Date constructor implicitly via the browser —
/// but since we're running in WASM and `chrono` parsing is cheap,
/// just hand-parse the digits and assume UTC for now. Timezone
/// handling lands when we wire chrono-tz. Returns None on parse fail.
fn parse_iso_local_to_millis(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    // Expected: 2026-05-14T13:30  (no seconds/timezone)
    let bytes = s.as_bytes();
    if bytes.len() < 16 {
        return None;
    }
    let year: i32 = s.get(0..4)?.parse().ok()?;
    let month: u32 = s.get(5..7)?.parse().ok()?;
    let day: u32 = s.get(8..10)?.parse().ok()?;
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    let minute: i64 = s.get(14..16)?.parse().ok()?;

    // Days since unix epoch using a basic algorithm — leap years
    // handled. Good enough for "is this in the future" admin checks;
    // we're not using this for billing or security.
    let days = days_since_epoch(year, month, day)?;
    let secs: i64 = days * 86_400 + hour * 3600 + minute * 60;
    Some(secs * 1000)
}

fn days_since_epoch(year: i32, month: u32, day: u32) -> Option<i64> {
    if !(1970..=9999).contains(&year) || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let mut days: i64 = 0;
    for y in 1970..year {
        days += if is_leap(y) { 366 } else { 365 };
    }
    let dim = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_feb = if is_leap(year) { 29 } else { 28 };
    for m in 1..month {
        if m == 2 {
            days += leap_feb;
        } else {
            days += dim[(m - 1) as usize] as i64;
        }
    }
    days += (day as i64) - 1;
    Some(days)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
