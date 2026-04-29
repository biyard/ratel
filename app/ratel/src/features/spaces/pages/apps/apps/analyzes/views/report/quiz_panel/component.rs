use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

const BAR_COLORS: &[&str] = &[
    "var(--c1)",
    "var(--c2)",
    "var(--c3)",
    "var(--c4)",
    "var(--c5)",
];

#[component]
pub fn QuizPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let ctrl = use_context::<UseAnalyzeReportDetail>();
    let detail = ctrl.detail.read().clone();
    let all_aggregates = detail
        .result
        .map(|r| r.quiz_aggregates)
        .unwrap_or_default();

    let selected = ctrl.selected_quiz.read().clone();
    let active_quiz_id = selected
        .or_else(|| all_aggregates.first().map(|q| q.quiz_id.clone()));
    let aggregates: Vec<QuizQuestionAggregate> = match active_quiz_id {
        Some(ref id) => all_aggregates
            .into_iter()
            .filter(|q| q.quiz_id == *id)
            .collect(),
        None => Vec::new(),
    };

    rsx! {
        section { class: "panel", "data-panel": "quiz", "data-active": "false",
            h1 { class: "main-title",
                span { class: "main-title__chip main-title__chip--quiz",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M9 9a3 3 0 0 1 6 0c0 2-3 3-3 3" }
                        line {
                            x1: "12",
                            y1: "17",
                            x2: "12.01",
                            y2: "17",
                        }
                    }
                    "{tr.detail_panel_chip_quiz}"
                }
                span { "data-quiz-title": true, "{tr.detail_quiz_title}" }
            }

            if aggregates.is_empty() {
                section { class: "card",
                    div { class: "card__head",
                        div { class: "card__title", "{tr.detail_panel_empty_title}" }
                    }
                    div { class: "card__hint", "{tr.detail_panel_empty_quiz}" }
                }
            } else {
                for (q_idx, q) in aggregates.iter().enumerate() {
                    QuizQuestionCard {
                        key: "quiz-q-{q_idx}-{q.quiz_id}",
                        question: q.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn QuizQuestionCard(question: QuizQuestionAggregate) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let total = question.respondent_count.max(1) as f64;
    let group_id = format!("quiz-q-{}-{}", question.quiz_id, question.question_idx);
    let correct_set: std::collections::HashSet<u32> = question.correct_indices.iter().copied().collect();
    let correct_pct = if question.respondent_count == 0 {
        0
    } else {
        ((question.correct_count as f64 / question.respondent_count as f64) * 100.0).round() as u32
    };

    let bars: Vec<BarItem> = question
        .options
        .iter()
        .enumerate()
        .map(|(idx, opt)| {
            let pct = (opt.count as f64 / total * 100.0).clamp(0.0, 100.0);
            let is_correct = correct_set.contains(&(idx as u32));
            let suffix = if is_correct {
                format!(" · {}", tr.create_sunji_correct_badge)
            } else {
                String::new()
            };
            BarItem {
                label: format!("{}", idx + 1),
                value: format!("{} · {} ({:.1}%){}", opt.label, opt.count, pct, suffix),
                width: format!("{:.1}%", pct),
                color: if is_correct {
                    "var(--quiz-correct)"
                } else {
                    BAR_COLORS[idx % BAR_COLORS.len()]
                },
                group: group_id.clone(),
                group_label: question.question_title.clone(),
                filter_value: opt.label.clone(),
                correct: is_correct,
                correct_text: is_correct,
            }
        })
        .collect();

    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{question.question_title}" }
                span { class: "card__count",
                    "{question.respondent_count}명 {tr.detail_attempts_unit} · {tr.detail_correct_rate_prefix} {correct_pct}%"
                }
            }
            if bars.is_empty() {
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
                            source: "quiz",
                            item: item.clone(),
                        }
                    }
                }
            }
        }
    }
}
