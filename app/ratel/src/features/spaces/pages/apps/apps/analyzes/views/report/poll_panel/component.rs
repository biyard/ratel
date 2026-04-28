use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

/// One bar-row inside a poll/quiz card.
#[derive(Clone, PartialEq)]
pub struct BarItem {
    pub label: String,
    pub value: String,
    pub width: String,
    pub color: &'static str,
    pub group: String,
    pub group_label: String,
    pub filter_value: String,
    pub correct: bool,
    pub correct_text: bool,
}

/// Stable rotation of bar colors. Mirrors the `--c1`..`--c5` palette
/// the existing CSS exposes. Cycles per question for visual variety
/// across cards without requiring per-aggregate palette config.
const BAR_COLORS: &[&str] = &[
    "var(--c1)",
    "var(--c2)",
    "var(--c3)",
    "var(--c4)",
    "var(--c5)",
];

#[component]
pub fn PollPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_context::<UseAnalyzeReportDetail>();
    let detail = ctrl.detail.read().clone();
    let all_aggregates = detail
        .result
        .map(|r| r.poll_aggregates)
        .unwrap_or_default();

    // Filter to the sidebar-selected poll. Falls back to the first
    // poll's id when nothing's selected yet so the panel always
    // renders the active sidebar item.
    let selected = ctrl.selected_poll.read().clone();
    let active_poll_id = selected
        .or_else(|| all_aggregates.first().map(|q| q.poll_id.clone()));
    let aggregates: Vec<PollQuestionAggregate> = match active_poll_id.as_ref() {
        Some(id) => all_aggregates
            .into_iter()
            .filter(|q| q.poll_id == *id)
            .collect(),
        None => Vec::new(),
    };
    let export_target = active_poll_id.clone();
    let export_disabled = export_target.is_none() || ctrl.handle_export_excel.pending();

    rsx! {
        section { class: "panel", "data-panel": "poll", "data-active": "true",
            // Action toolbar — sits above the title, right-aligned.
            // Lives outside `h1.main-title` so the title's chip+text
            // baseline stays untouched and the button stack can grow
            // (additional exports, filter resets, etc.) without
            // re-flowing the heading.
            div { class: "panel-toolbar",
                button {
                    class: "btn btn--primary panel-toolbar__action",
                    r#type: "button",
                    disabled: export_disabled,
                    onclick: move |_| {
                        if let Some(id) = export_target.clone() {
                            ctrl.handle_export_excel.call(id);
                        }
                    },
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line {
                            x1: "12",
                            y1: "15",
                            x2: "12",
                            y2: "3",
                        }
                    }
                    "{tr.download_excel}"
                }
            }
            h1 { class: "main-title",
                span { class: "main-title__chip main-title__chip--poll",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        line {
                            x1: "18",
                            y1: "20",
                            x2: "18",
                            y2: "10",
                        }
                        line {
                            x1: "12",
                            y1: "20",
                            x2: "12",
                            y2: "4",
                        }
                        line {
                            x1: "6",
                            y1: "20",
                            x2: "6",
                            y2: "14",
                        }
                    }
                    "{tr.detail_panel_chip_poll}"
                }
                span { "data-poll-title": true, "{tr.detail_poll_title}" }
            }

            if aggregates.is_empty() {
                section { class: "card",
                    div { class: "card__head",
                        div { class: "card__title", "{tr.detail_panel_empty_title}" }
                    }
                    div { class: "card__hint", "{tr.detail_panel_empty_poll}" }
                }
            } else {
                for (q_idx, q) in aggregates.iter().enumerate() {
                    PollQuestionCard {
                        key: "poll-q-{q_idx}-{q.poll_id}",
                        question: q.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn PollQuestionCard(question: PollQuestionAggregate) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let total = question.respondent_count.max(1) as f64;
    let group_id = format!("poll-q-{}-{}", question.poll_id, question.question_idx);

    let bars: Vec<BarItem> = question
        .options
        .iter()
        .enumerate()
        .map(|(idx, opt)| {
            let pct = (opt.count as f64 / total * 100.0).clamp(0.0, 100.0);
            BarItem {
                label: format!("{}", idx + 1),
                value: format!("{} · {} ({:.1}%)", opt.label, opt.count, pct),
                width: format!("{:.1}%", pct),
                color: BAR_COLORS[idx % BAR_COLORS.len()],
                group: group_id.clone(),
                group_label: question.question_title.clone(),
                filter_value: opt.label.clone(),
                correct: false,
                correct_text: false,
            }
        })
        .collect();

    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{question.question_title}" }
                span { class: "card__count", "{question.respondent_count}명 {tr.detail_responses_unit}" }
            }
            if bars.is_empty() {
                // No-options question (free text). Render its text answers instead.
                if question.text_answers.is_empty() {
                    div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
                } else {
                    div { class: "text-list",
                        for (idx, txt) in question.text_answers.iter().enumerate() {
                            div { key: "txt-{idx}", class: "text-item", "{txt}" }
                        }
                    }
                }
            } else {
                div { class: "bar-chart",
                    for (idx, item) in bars.iter().enumerate() {
                        BarRow {
                            key: "{group_id}-{idx}",
                            source: "poll",
                            item: item.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BarRow(source: &'static str, item: BarItem) -> Element {
    let style = format!("width: {}; background: {};", item.width, item.color);
    let correct_attr = item.correct.to_string();
    rsx! {
        button {
            class: "bar-row",
            r#type: "button",
            "aria-pressed": "false",
            "data-correct": "{correct_attr}",
            "data-filter-source": "{source}",
            "data-filter-group": "{item.group}",
            "data-filter-group-label": "{item.group_label}",
            "data-filter-value": "{item.filter_value}",
            span { class: "bar-row__label", "{item.label}" }
            div { class: "bar-row__track",
                div { class: "bar-row__fill", style: "{style}" }
            }
            span { class: "bar-row__value",
                if item.correct_text {
                    span { class: "bar-row__check", "✓" }
                }
                "{item.value}"
            }
        }
    }
}
