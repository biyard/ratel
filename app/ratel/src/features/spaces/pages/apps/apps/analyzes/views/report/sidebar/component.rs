use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

/// One sidebar item under a group. JS owns active state — clicking
/// sets `aria-selected="true"` on this item, swaps `data-active` on
/// the matching `<section class="panel">`. Dioxus only renders the
/// initial state.
#[derive(Clone, PartialEq)]
pub struct SbItem {
    pub title: String,
    pub meta: String,
    pub target: &'static str,
    pub selected: bool,
}

/// One ANALYZES group (poll / quiz / discussion / follow).
#[derive(Clone, PartialEq)]
pub struct SbGroup {
    pub group: &'static str,
    pub label: String,
    pub items: Vec<SbItem>,
}

#[component]
pub fn ReportSidebar() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    // Mock content — verbatim from `analyze-detail-arena.html`.
    // Specialised per-action data (questions counts, comment counts) is
    // hardcoded here to match the design file 1:1; nothing comes from the
    // loaded `AnalyzeReport.filters`. Real data fetching is out of scope
    // for this Phase-3 visual port.
    let groups = vec![
        SbGroup {
            group: "poll",
            label: tr.detail_group_poll.to_string(),
            items: vec![
                SbItem {
                    title: "귀하는 현재 대한민국 헌법을 개정하는 것이 필요하다고 생각하십니까?".to_string(),
                    meta: format!("5 {} · 128명 {}", tr.detail_sb_item_meta_questions, tr.detail_sb_item_meta_responses),
                    target: "poll",
                    selected: true,
                },
                SbItem {
                    title: "공직 선거 연령 하향에 찬성하십니까?".to_string(),
                    meta: format!("3 {} · 94명 {}", tr.detail_sb_item_meta_questions, tr.detail_sb_item_meta_responses),
                    target: "poll",
                    selected: false,
                },
                SbItem {
                    title: "사법부 독립성 강화 방안 우선순위".to_string(),
                    meta: format!("7 {} · 71명 {}", tr.detail_sb_item_meta_questions, tr.detail_sb_item_meta_responses),
                    target: "poll",
                    selected: false,
                },
            ],
        },
        SbGroup {
            group: "quiz",
            label: tr.detail_group_quiz.to_string(),
            items: vec![
                SbItem {
                    title: "헌법 기본 상식 퀴즈".to_string(),
                    meta: format!("10 {} · 86명 {}", tr.detail_sb_item_meta_questions, tr.detail_sb_item_meta_attempts),
                    target: "quiz",
                    selected: false,
                },
                SbItem {
                    title: "법률 용어 이해도 테스트".to_string(),
                    meta: format!("8 {} · 47명 {}", tr.detail_sb_item_meta_questions, tr.detail_sb_item_meta_attempts),
                    target: "quiz",
                    selected: false,
                },
            ],
        },
        SbGroup {
            group: "discussion",
            label: tr.detail_group_discussion.to_string(),
            items: vec![
                SbItem {
                    title: "비동의 강간죄 도입에 대해서 어떻게 생각하십니까?".to_string(),
                    meta: format!("142 {} · 38명 {}", tr.detail_sb_item_meta_comments, tr.detail_sb_item_meta_participants),
                    target: "discussion",
                    selected: false,
                },
                SbItem {
                    title: "무고죄 형량 강화에 대한 시민 의견".to_string(),
                    meta: format!("87 {} · 24명 {}", tr.detail_sb_item_meta_comments, tr.detail_sb_item_meta_participants),
                    target: "discussion",
                    selected: false,
                },
            ],
        },
        SbGroup {
            group: "follow",
            label: tr.detail_group_follow.to_string(),
            items: vec![SbItem {
                title: "법률 전문가 팔로우 캠페인".to_string(),
                meta: format!("12 {} · 42명 {}", tr.detail_sb_item_meta_targets, tr.detail_sb_item_meta_participants),
                target: "follow",
                selected: false,
            }],
        },
    ];

    rsx! {
        aside { class: "sidebar", "aria-label": "Analyze items",
            div { class: "sidebar__head",
                span { class: "sidebar__head-label", "{tr.detail_sidebar_label}" }
            }
            for group in groups.iter() {
                ReportSidebarGroup { key: "{group.group}", group: group.clone() }
            }
        }
    }
}

#[component]
fn ReportSidebarGroup(group: SbGroup) -> Element {
    let count = group.items.len();
    let class = format!("sb-group sb-group--{}", group.group);
    rsx! {
        div {
            class: "{class}",
            "data-group": "{group.group}",
            "data-collapsed": "false",
            div { class: "sb-group__head", "data-group-toggle": true,
                div { class: "sb-group__head-left",
                    span { class: "sb-group__icon", {group_icon(group.group)} }
                    span { class: "sb-group__label", "{group.label}" }
                    span { class: "sb-group__count", "{count}" }
                }
                svg {
                    class: "sb-group__chevron",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            div { class: "sb-group__list",
                for (idx, item) in group.items.iter().enumerate() {
                    button {
                        key: "{group.group}-{idx}",
                        class: "sb-item",
                        r#type: "button",
                        "aria-selected": "{item.selected}",
                        "data-target-panel": "{item.target}",
                        "data-testid": "sb-item-{group.group}-{idx}",
                        span { class: "sb-item__indicator" }
                        div { class: "sb-item__body",
                            span { class: "sb-item__title", "{item.title}" }
                            span { class: "sb-item__meta", "{item.meta}" }
                        }
                    }
                }
            }
        }
    }
}

fn group_icon(group: &str) -> Element {
    match group {
        "poll" => rsx! {
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
        },
        "quiz" => rsx! {
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
        },
        "discussion" => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }
        },
        "follow" => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                circle { cx: "8.5", cy: "7", r: "4" }
                line {
                    x1: "20",
                    y1: "8",
                    x2: "20",
                    y2: "14",
                }
                line {
                    x1: "23",
                    y1: "11",
                    x2: "17",
                    y2: "11",
                }
            }
        },
        _ => rsx! {},
    }
}
