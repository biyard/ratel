use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use std::collections::BTreeMap;

/// Aggregated parent (poll / quiz) summary for the sidebar.
struct ParentGroup {
    id: String,
    title: String,
    question_count: usize,
    /// Max respondent_count across the parent's questions. A single
    /// scalar is more honest as a "people who touched this parent"
    /// number than a sum (which would multi-count the same user when
    /// they answered multiple questions of the same poll).
    respondent_count: u32,
}

/// Group poll question aggregates by their parent poll. Order: by
/// first appearance in the `poll_aggregates` vec — the auto-analysis
/// service already orders them deterministically by (poll_id,
/// question_idx), so this preserves that ordering.
fn group_poll_aggregates(aggs: &[PollQuestionAggregate]) -> Vec<ParentGroup> {
    let mut order: Vec<String> = Vec::new();
    let mut by_id: BTreeMap<String, ParentGroup> = BTreeMap::new();
    for q in aggs {
        let entry = by_id.entry(q.poll_id.clone()).or_insert_with(|| {
            order.push(q.poll_id.clone());
            ParentGroup {
                id: q.poll_id.clone(),
                title: if q.poll_title.trim().is_empty() {
                    q.poll_id.clone()
                } else {
                    q.poll_title.clone()
                },
                question_count: 0,
                respondent_count: 0,
            }
        });
        entry.question_count += 1;
        if q.respondent_count > entry.respondent_count {
            entry.respondent_count = q.respondent_count;
        }
    }
    order.into_iter().filter_map(|id| by_id.remove(&id)).collect()
}

fn group_quiz_aggregates(aggs: &[QuizQuestionAggregate]) -> Vec<ParentGroup> {
    let mut order: Vec<String> = Vec::new();
    let mut by_id: BTreeMap<String, ParentGroup> = BTreeMap::new();
    for q in aggs {
        let entry = by_id.entry(q.quiz_id.clone()).or_insert_with(|| {
            order.push(q.quiz_id.clone());
            ParentGroup {
                id: q.quiz_id.clone(),
                title: if q.quiz_title.trim().is_empty() {
                    q.quiz_id.clone()
                } else {
                    q.quiz_title.clone()
                },
                question_count: 0,
                respondent_count: 0,
            }
        });
        entry.question_count += 1;
        if q.respondent_count > entry.respondent_count {
            entry.respondent_count = q.respondent_count;
        }
    }
    order.into_iter().filter_map(|id| by_id.remove(&id)).collect()
}

/// One sidebar item under a group. JS still owns visible-panel
/// switching (via `data-target-panel`), but Dioxus owns *which item
/// inside that panel is active* through the controller's
/// `selected_*` signals — clicking an item stamps its `action_id`
/// onto the matching signal so the panel re-renders filtered.
#[derive(Clone, PartialEq)]
pub struct SbItem {
    pub title: String,
    pub meta: String,
    pub target: &'static str,
    pub selected: bool,
    /// Action id this item refers to. Set for poll / quiz / discussion
    /// items; empty for follow (the data model has no per-target
    /// follow-action id today, so the panel shows everything).
    pub action_id: String,
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

    let selected_poll = ctrl.selected_poll.read().clone();
    let selected_quiz = ctrl.selected_quiz.read().clone();
    let selected_discussion = ctrl.selected_discussion.read().clone();
    let active_panel = ctrl.active_panel.read().clone();

    // Group poll questions under their parent poll. The detail-page
    // sidebar surfaces one entry per *poll* (parent action), not per
    // question — clicking a poll filters the poll panel to that
    // poll's questions only. Same for quiz, discussion, follow.
    //
    // `selected` flag is set when the controller's selection signal
    // matches this group's id; falls back to "first item" so the
    // panel always has *something* to render — see panel components
    // for the matching default-selection logic.
    let poll_groups = group_poll_aggregates(&result.poll_aggregates);
    let first_poll_id = poll_groups.first().map(|g| g.id.clone());
    let active_poll = selected_poll.clone().or(first_poll_id);
    let poll_items: Vec<SbItem> = poll_groups
        .into_iter()
        .map(|g| {
            // Highlight the active poll only when the *poll* panel is
            // the visible one. Otherwise the dot would also light up
            // when the user is on the quiz / discussion panel.
            let is_selected =
                active_panel == "poll" && active_poll.as_deref() == Some(g.id.as_str());
            SbItem {
                title: g.title,
                meta: format!(
                    "{} {} · {}명 {}",
                    g.question_count,
                    tr.detail_sb_item_meta_questions,
                    g.respondent_count,
                    tr.detail_sb_item_meta_responses,
                ),
                target: "poll",
                selected: is_selected,
                action_id: g.id,
            }
        })
        .collect();

    let quiz_groups = group_quiz_aggregates(&result.quiz_aggregates);
    let first_quiz_id = quiz_groups.first().map(|g| g.id.clone());
    let active_quiz = selected_quiz.clone().or(first_quiz_id);
    let quiz_items: Vec<SbItem> = quiz_groups
        .into_iter()
        .map(|g| {
            let is_selected =
                active_panel == "quiz" && active_quiz.as_deref() == Some(g.id.as_str());
            SbItem {
                title: g.title,
                meta: format!(
                    "{} {} · {}명 {}",
                    g.question_count,
                    tr.detail_sb_item_meta_questions,
                    g.respondent_count,
                    tr.detail_sb_item_meta_attempts,
                ),
                target: "quiz",
                selected: is_selected,
                action_id: g.id,
            }
        })
        .collect();

    let first_discussion_id = detail
        .discussions
        .first()
        .map(|d| d.discussion_id.to_string());
    let active_discussion = selected_discussion.clone().or(first_discussion_id);
    let discussion_items: Vec<SbItem> = detail
        .discussions
        .iter()
        .map(|d| {
            let did = d.discussion_id.to_string();
            let is_selected =
                active_panel == "discussion" && active_discussion.as_deref() == Some(did.as_str());
            SbItem {
                title: d.title.clone(),
                meta: format!(
                    // Both numbers are cross-filter aware:
                    //   matched_comment_count    = comments authored by
                    //                              users in the matched
                    //                              set on this post
                    //   matched_participant_count = distinct authors
                    //                              from that set
                    "{} {} · {}명 {}",
                    d.matched_comment_count,
                    tr.detail_sb_item_meta_comments,
                    d.matched_participant_count,
                    tr.detail_sb_item_meta_participants,
                ),
                target: "discussion",
                selected: is_selected,
                action_id: did,
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
                action_id: String::new(),
            }
        })
        .collect();

    // Groups with zero items are hidden entirely — when a report has
    // no poll/quiz/discussion/follow data, the matching sidebar bucket
    // has no purpose. Showing an empty header just creates dead
    // chrome.
    let groups: Vec<SbGroup> = vec![
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
    ]
    .into_iter()
    .filter(|g| !g.items.is_empty())
    .collect();

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
                        let action_id = item.action_id.clone();
                        let target = item.target;
                        rsx! {
                            button {
                                key: "{group.group}-{idx}",
                                class: "sb-item",
                                r#type: "button",
                                "aria-selected": "{item.selected}",
                                "data-target-panel": "{item.target}",
                                "data-testid": "sb-item-{group.group}-{idx}",
                                onclick: move |_| {
                                    // Always switch the active panel so the
                                    // sidebar's selected dot follows the panel
                                    // currently in view (matches the JS-driven
                                    // `data-active` swap).
                                    ctrl.active_panel.set(target.to_string());
                                    if action_id.is_empty() {
                                        return;
                                    }
                                    match target {
                                        "poll" => ctrl.selected_poll.set(Some(action_id.clone())),
                                        "quiz" => ctrl.selected_quiz.set(Some(action_id.clone())),
                                        "discussion" => {
                                            ctrl.selected_discussion.set(Some(action_id.clone()))
                                        }
                                        _ => {}
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
