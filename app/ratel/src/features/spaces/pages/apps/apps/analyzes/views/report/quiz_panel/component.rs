use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

/// Quiz panel — same shape as poll, with one extra signal: the
/// correct option in each question gets a green accent + ✓ check.
#[component]
pub fn QuizPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let q1: Vec<BarItem> = vec![
        BarItem {
            label: "1".into(),
            value: "7명 · 41 (47.7%)".into(),
            width: "47.7%",
            color: "var(--c1)",
            group: "quiz-q1",
            group_label: "재판관 수",
            filter_value: "7명".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "2".into(),
            value: "9명 · 19 (22.1%) · 정답".into(),
            width: "22.1%",
            color: "var(--quiz-correct)",
            group: "quiz-q1",
            group_label: "재판관 수",
            filter_value: "9명 (정답)".into(),
            correct: true,
            correct_text: true,
        },
        BarItem {
            label: "3".into(),
            value: "11명 · 17 (19.8%)".into(),
            width: "19.8%",
            color: "var(--c2)",
            group: "quiz-q1",
            group_label: "재판관 수",
            filter_value: "11명".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "4".into(),
            value: "13명 · 9 (10.5%)".into(),
            width: "10.5%",
            color: "var(--c4)",
            group: "quiz-q1",
            group_label: "재판관 수",
            filter_value: "13명".into(),
            correct: false,
            correct_text: false,
        },
    ];

    let q2: Vec<BarItem> = vec![
        BarItem {
            label: "1".into(),
            value: "국회 · 76 (88.4%) · 정답".into(),
            width: "88.4%",
            color: "var(--quiz-correct)",
            group: "quiz-q2",
            group_label: "입법권",
            filter_value: "국회 (정답)".into(),
            correct: true,
            correct_text: true,
        },
        BarItem {
            label: "2".into(),
            value: "정부 · 6 (7.0%)".into(),
            width: "7.0%",
            color: "var(--c1)",
            group: "quiz-q2",
            group_label: "입법권",
            filter_value: "정부".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "3".into(),
            value: "대통령 · 3 (3.5%)".into(),
            width: "3.5%",
            color: "var(--c2)",
            group: "quiz-q2",
            group_label: "입법권",
            filter_value: "대통령".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "4".into(),
            value: "법원 · 1 (1.2%)".into(),
            width: "1.2%",
            color: "var(--c4)",
            group: "quiz-q2",
            group_label: "입법권",
            filter_value: "법원".into(),
            correct: false,
            correct_text: false,
        },
    ];

    let q3: Vec<BarItem> = vec![
        BarItem {
            label: "1".into(),
            value: "대한민국은 민주공화국이다 · 81 (94.2%) · 정답".into(),
            width: "94.2%",
            color: "var(--quiz-correct)",
            group: "quiz-q3",
            group_label: "헌법 제1조",
            filter_value: "민주공화국 (정답)".into(),
            correct: true,
            correct_text: true,
        },
        BarItem {
            label: "2".into(),
            value: "대한민국의 영토는 한반도 · 3 (3.5%)".into(),
            width: "3.5%",
            color: "var(--c1)",
            group: "quiz-q3",
            group_label: "헌법 제1조",
            filter_value: "영토는 한반도".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "3".into(),
            value: "모든 국민은 인간으로서의 존엄 · 2 (2.3%)".into(),
            width: "2.3%",
            color: "var(--c2)",
            group: "quiz-q3",
            group_label: "헌법 제1조",
            filter_value: "인간 존엄".into(),
            correct: false,
            correct_text: false,
        },
    ];

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

            // Card 1 — quiz with hint
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_quiz_card1_title}" }
                    span { class: "card__count",
                        "86명 {tr.detail_attempts_unit} · {tr.detail_correct_rate_prefix} 22%"
                    }
                }
                span { class: "card__hint",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
                    }
                    "{tr.detail_card_hint_quiz}"
                }
                div { class: "bar-chart",
                    for (idx, item) in q1.iter().enumerate() {
                        BarRow {
                            key: "quiz-q1-{idx}",
                            source: "quiz",
                            item: item.clone(),
                        }
                    }
                }
            }

            // Card 2 — quiz
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_quiz_card2_title}" }
                    span { class: "card__count",
                        "86명 {tr.detail_attempts_unit} · {tr.detail_correct_rate_prefix} 88%"
                    }
                }
                div { class: "bar-chart",
                    for (idx, item) in q2.iter().enumerate() {
                        BarRow {
                            key: "quiz-q2-{idx}",
                            source: "quiz",
                            item: item.clone(),
                        }
                    }
                }
            }

            // Card 3 — quiz
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_quiz_card3_title}" }
                    span { class: "card__count",
                        "86명 {tr.detail_attempts_unit} · {tr.detail_correct_rate_prefix} 94%"
                    }
                }
                div { class: "bar-chart",
                    for (idx, item) in q3.iter().enumerate() {
                        BarRow {
                            key: "quiz-q3-{idx}",
                            source: "quiz",
                            item: item.clone(),
                        }
                    }
                }
            }
        }
    }
}
