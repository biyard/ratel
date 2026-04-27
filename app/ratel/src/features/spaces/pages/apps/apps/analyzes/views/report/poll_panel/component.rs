use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

/// One bar-row inside a poll/quiz card. Mockup-shaped struct so
/// poll_panel and quiz_panel can share rendering helpers if they grow.
#[derive(Clone, PartialEq)]
pub struct BarItem {
    pub label: String,    // "1", "2", … (column 1 of the bar-row)
    pub value: String,    // "기본권 강화 · 52 (40.6%)" (column 3)
    pub width: &'static str, // CSS width value, e.g. "40.6%"
    pub color: &'static str, // CSS color reference, e.g. "var(--c1)"
    pub group: &'static str, // data-filter-group, e.g. "poll-q1"
    pub group_label: &'static str,
    pub filter_value: String, // data-filter-value
    pub correct: bool,    // quiz-only: green accent + ✓
    pub correct_text: bool, // value text starts with "✓ … · 정답"
}

/// Poll panel — visible by default. JS toggles `data-active` based on
/// which sb-item the user clicked. Two multi-choice charts + one
/// free-text card. Mockup data hardcoded — see analyze-detail-arena.html.
#[component]
pub fn PollPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let q1: Vec<BarItem> = vec![
        BarItem {
            label: "1".into(),
            value: "기본권 강화 · 52 (40.6%)".into(),
            width: "40.6%",
            color: "var(--c1)",
            group: "poll-q1",
            group_label: "시급 분야",
            filter_value: "기본권 강화".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "2".into(),
            value: "권력구조 개편 · 38 (29.7%)".into(),
            width: "29.7%",
            color: "var(--c2)",
            group: "poll-q1",
            group_label: "시급 분야",
            filter_value: "권력구조 개편".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "3".into(),
            value: "지방분권 강화 · 26 (20.3%)".into(),
            width: "20.3%",
            color: "var(--c3)",
            group: "poll-q1",
            group_label: "시급 분야",
            filter_value: "지방분권 강화".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "4".into(),
            value: "사법 독립 · 12 (9.4%)".into(),
            width: "9.4%",
            color: "var(--c4)",
            group: "poll-q1",
            group_label: "시급 분야",
            filter_value: "사법 독립".into(),
            correct: false,
            correct_text: false,
        },
    ];

    let q2: Vec<BarItem> = vec![
        BarItem {
            label: "1".into(),
            value: "자유 · 64 (51.6%)".into(),
            width: "51.6%",
            color: "var(--c5)",
            group: "poll-q2",
            group_label: "중요 가치",
            filter_value: "자유".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "2".into(),
            value: "평등 · 41 (33.1%)".into(),
            width: "33.1%",
            color: "var(--c6)",
            group: "poll-q2",
            group_label: "중요 가치",
            filter_value: "평등".into(),
            correct: false,
            correct_text: false,
        },
        BarItem {
            label: "3".into(),
            value: "연대 · 19 (15.3%)".into(),
            width: "15.3%",
            color: "var(--c4)",
            group: "poll-q2",
            group_label: "중요 가치",
            filter_value: "연대".into(),
            correct: false,
            correct_text: false,
        },
    ];

    let free_responses = vec![
        "국민 다수가 합의하지 않은 상태에서 개헌을 강행해서는 안 된다고 생각합니다. 충분한 공론화 과정과 시민 참여 절차가 우선되어야 합니다. (8)",
        "기본권 조항을 강화하면서도 의무 조항과의 균형이 무너지지 않도록 신중하게 다뤄야 합니다. (6)",
        "지방분권을 명시할 때 재정 자립 대책도 함께 헌법에 담아야 실효성이 있다고 봅니다. (4)",
        "사법부 독립을 명문화하더라도, 사법 평가에 대한 국민 통제 장치도 함께 마련되어야 합니다. (3)",
        "권력구조 개편은 정파적 이해를 떠나 시민의 일상에 미치는 영향을 중심으로 논의되어야 합니다. (3)",
    ];

    rsx! {
        section { class: "panel", "data-panel": "poll", "data-active": "true",
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

            // Toolbar — filter select + download.
            div { class: "panel-toolbar",
                label { class: "filter-select-wrap",
                    select { class: "filter-select",
                        option { "{tr.detail_filter_all}" }
                        option { "{tr.detail_filter_gender}" }
                        option { "{tr.detail_filter_age}" }
                        option { "{tr.detail_filter_school}" }
                    }
                }
                button {
                    class: "btn btn--primary",
                    "data-testid": "download-poll-btn",
                    svg {
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
                    "{tr.detail_download_btn}"
                }
            }

            // Card 1 — bar chart with hint
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_poll_card1_title}" }
                    span { class: "card__count", "128명 {tr.detail_responses_unit}" }
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
                    "{tr.detail_card_hint_poll}"
                }
                div { class: "bar-chart",
                    for (idx, item) in q1.iter().enumerate() {
                        BarRow {
                            key: "poll-q1-{idx}",
                            source: "poll",
                            item: item.clone(),
                        }
                    }
                }
            }

            // Card 2 — bar chart, no hint
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_poll_card2_title}" }
                    span { class: "card__count", "124명 {tr.detail_responses_unit}" }
                }
                div { class: "bar-chart",
                    for (idx, item) in q2.iter().enumerate() {
                        BarRow {
                            key: "poll-q2-{idx}",
                            source: "poll",
                            item: item.clone(),
                        }
                    }
                }
            }

            // Card 3 — free text answers
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_poll_card3_title}" }
                    span { class: "card__count", "54명 {tr.detail_responses_unit}" }
                }
                div { class: "text-list",
                    for (idx, txt) in free_responses.iter().enumerate() {
                        div { key: "txt-{idx}", class: "text-item", "{txt}" }
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
