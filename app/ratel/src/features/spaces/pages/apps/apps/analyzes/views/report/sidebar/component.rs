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
    /// Stable identifier for discussion items so the click handler
    /// can stamp the selected discussion on the controller. Empty
    /// for non-discussion targets.
    pub discussion_id: String,
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
    let ctrl = use_context::<UseAnalyzeReportDetail>();
    let detail = ctrl.detail.read().clone();
    let result = detail.result.unwrap_or_default();

    let respondent = result.respondent_count;

    // Poll items: one per (poll, question) the matched users
    // answered. Title is the question, meta is "<N options> · <M명>
    // 응답".
    let poll_items: Vec<SbItem> = result
        .poll_aggregates
        .iter()
        .map(|q| SbItem {
            title: q.question_title.clone(),
            meta: format!(
                "{} {} · {}명 {}",
                q.options.len(),
                tr.detail_sb_item_meta_options,
                q.respondent_count,
                tr.detail_sb_item_meta_responses,
            ),
            target: "poll",
            selected: false,
            discussion_id: String::new(),
        })
        .collect();

    let quiz_items: Vec<SbItem> = result
        .quiz_aggregates
        .iter()
        .map(|q| SbItem {
            title: q.question_title.clone(),
            meta: format!(
                "{} {} · {}/{}명 {}",
                q.options.len(),
                tr.detail_sb_item_meta_options,
                q.correct_count,
                q.respondent_count,
                tr.detail_sb_item_meta_correct,
            ),
            target: "quiz",
            selected: false,
            discussion_id: String::new(),
        })
        .collect();

    let selected_discussion = ctrl.selected_discussion.read().clone();
    let discussion_items: Vec<SbItem> = detail
        .discussions
        .iter()
        .map(|d| {
            let did = d.discussion_id.to_string();
            let is_selected = selected_discussion.as_deref() == Some(did.as_str());
            SbItem {
                title: d.title.clone(),
                meta: format!(
                    "{} {} · {}명 {}",
                    d.comment_count,
                    tr.detail_sb_item_meta_comments,
                    respondent,
                    tr.detail_sb_item_meta_participants,
                ),
                target: "discussion",
                selected: is_selected,
                discussion_id: did,
            }
        })
        .collect();

    let follow_items: Vec<SbItem> = result
        .follow_aggregates
        .iter()
        .map(|f| {
            let label = if f.display_name.is_empty() {
                f.username.clone()
            } else {
                f.display_name.clone()
            };
            SbItem {
                title: label,
                meta: format!(
                    "{} {}",
                    f.count, tr.detail_sb_item_meta_followers,
                ),
                target: "follow",
                selected: false,
                discussion_id: String::new(),
            }
        })
        .collect();

    let groups = vec![
        SbGroup {
            group: "poll",
            label: tr.detail_group_poll.to_string(),
            items: poll_items,
        },
        SbGroup {
            group: "quiz",
            label: tr.detail_group_quiz.to_string(),
            items: quiz_items,
        },
        SbGroup {
            group: "discussion",
            label: tr.detail_group_discussion.to_string(),
            items: discussion_items,
        },
        SbGroup {
            group: "follow",
            label: tr.detail_group_follow.to_string(),
            items: follow_items,
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
    let mut ctrl = use_context::<UseAnalyzeReportDetail>();

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
                    {
                        let did = item.discussion_id.clone();
                        let is_discussion = item.target == "discussion";
                        rsx! {
                            button {
                                key: "{group.group}-{idx}",
                                class: "sb-item",
                                r#type: "button",
                                "aria-selected": "{item.selected}",
                                "data-target-panel": "{item.target}",
                                "data-testid": "sb-item-{group.group}-{idx}",
                                onclick: move |_| {
                                    if is_discussion && !did.is_empty() {
                                        ctrl.selected_discussion.set(Some(did.clone()));
                                    }
                                },
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
