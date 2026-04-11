use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::action_pages::quiz::CompletedActionCard;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::types::space_page_actions_key;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum ActionStatus {
    Active,
    Completed,
    Skipped,
}

pub(super) fn derive_action_status(action: &SpaceActionSummary) -> ActionStatus {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let ended = action.ended_at.map(|t| now >= t).unwrap_or(false);

    match action.action_type {
        SpaceActionType::Poll => {
            if action.user_participated {
                ActionStatus::Completed
            } else if ended {
                ActionStatus::Skipped
            } else {
                ActionStatus::Active
            }
        }
        SpaceActionType::TopicDiscussion => {
            if ended && action.user_participated {
                ActionStatus::Completed
            } else if ended {
                ActionStatus::Skipped
            } else {
                ActionStatus::Active
            }
        }
        SpaceActionType::Quiz => {
            if action.quiz_passed == Some(true) {
                ActionStatus::Completed
            } else if ended || action.quiz_passed == Some(false) {
                ActionStatus::Skipped
            } else {
                ActionStatus::Active
            }
        }
        SpaceActionType::Follow => {
            if action.user_participated {
                ActionStatus::Completed
            } else if ended {
                ActionStatus::Skipped
            } else {
                ActionStatus::Active
            }
        }
    }
}

#[component]
pub fn ActionDashboard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let actions_key = space_page_actions_key(&space_id());
    let actions_loader = use_query(&actions_key, move || list_actions(space_id()))?;
    let actions = actions_loader();
    let lang = use_language();
    let mut query = use_query_store();

    let active: Vec<_> = actions
        .iter()
        .filter(|a| derive_action_status(a) == ActionStatus::Active)
        .cloned()
        .collect();
    let completed: Vec<_> = actions
        .iter()
        .filter(|a| derive_action_status(a) == ActionStatus::Completed)
        .cloned()
        .collect();
    let skipped: Vec<_> = actions
        .iter()
        .filter(|a| derive_action_status(a) == ActionStatus::Skipped)
        .cloned()
        .collect();

    let total = actions.len();
    let done = completed.len();
    let skipped_count = skipped.len();
    let progress_pct = if total > 0 {
        (done as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let mut show_archive = use_signal(|| false);
    let mut completed_action: CompletedActionCard = use_context();
    let archive_action_id = completed_action.0();

    // After fly animation: clear signal and refresh the actions list
    use_effect(move || {
        if completed_action.0().is_some() {
            let actions_key = space_page_actions_key(&space_id());
            spawn(async move {
                #[cfg(feature = "web")]
                gloo_timers::future::sleep(std::time::Duration::from_millis(1500)).await;
                completed_action.0.set(None);
                query.invalidate(&actions_key);
            });
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "quest-label",
            span { class: "quest-label__title", "{tr.your_quests}" }
        }

        if active.is_empty() {
            div { class: "quest-empty",
                div { class: "quest-empty__icon",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                        polyline { points: "22 4 12 14.01 9 11.01" }
                    }
                }
                div { class: "quest-empty__text", "{tr.all_quests_done}" }
            }
        } else {
            div { class: "carousel-wrapper",
                div {
                    class: "carousel-track",
                    id: "carousel-track",
                    "data-archive-action": archive_action_id.clone().unwrap_or_default(),
                    for action in active.iter() {
                        {
                            let action = action.clone();
                            let key = action.action_id.clone();
                            match action.action_type {
                                SpaceActionType::Poll => rsx! {
                                    PollActionCard { key: "{key}", action, space_id }
                                },
                                SpaceActionType::TopicDiscussion => rsx! {
                                    DiscussionActionCard { key: "{key}", action, space_id }
                                },
                                SpaceActionType::Quiz => rsx! {
                                    QuizActionCard { key: "{key}", action, space_id }
                                },
                                SpaceActionType::Follow => rsx! {
                                    FollowActionCard { key: "{key}", action, space_id }
                                },
                            }
                        }
                    }
                }
            }

            div { class: "carousel-dots", id: "carousel-dots",
                for action in active.iter() {
                    button {
                        class: "carousel-dot",
                        "data-type": quest_type_css(&action.action_type),
                    }
                }
            }
        }

        // Bottom bar
        div { class: "bottom-bar",
            div { class: "quest-progress",
                span { class: "quest-progress__label", "{tr.quest_progress}" }
                div { class: "quest-progress__bar-wrap",
                    div {
                        class: "quest-progress__bar",
                        width: "{progress_pct}%",
                    }
                }
                span { class: "quest-progress__fraction", "{done} / {total}" }
            }

            button {
                class: "archive-btn",
                "data-testid": "btn-archive",
                aria_label: "Completed quests",
                onclick: move |_| {
                    show_archive.set(!show_archive());
                },
                svg {
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                }
                if done + skipped_count > 0 {
                    span { class: "archive-btn__count", "{done + skipped_count}" }
                }
            }
        }

        // Archive panel
        div {
            class: "archive-panel",
            "data-open": show_archive(),
            "data-testid": "archive-panel",
            div { class: "archive-panel__header",
                span { class: "archive-panel__title", "{tr.completed}" }
                button {
                    class: "archive-panel__close",
                    onclick: move |_| {
                        show_archive.set(false);
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        line {
                            x1: "18",
                            x2: "6",
                            y1: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            x2: "18",
                            y1: "6",
                            y2: "18",
                        }
                    }
                }
            }
            div { class: "archive-panel__list",
                if completed.is_empty() && skipped.is_empty() {
                    div { class: "archive-panel__empty", "{tr.no_completed_yet}" }
                } else {
                    for action in completed.iter() {
                        ArchiveItem {
                            action: action.clone(),
                            status: ActionStatus::Completed,
                            space_id: space_id(),
                        }
                    }
                    for action in skipped.iter() {
                        ArchiveItem {
                            action: action.clone(),
                            status: ActionStatus::Skipped,
                            space_id: space_id(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArchiveItem(action: SpaceActionSummary, status: ActionStatus, space_id: SpacePartition) -> Element {
    let lang = use_language();
    let tr: SpaceViewerTranslate = use_translate();
    let is_completed = status == ActionStatus::Completed;
    let is_poll = action.action_type == SpaceActionType::Poll;
    let can_reopen = is_completed && is_poll;
    let nav = navigator();
    let url = action.get_url(&space_id);

    rsx! {
        div {
            class: "archive-item",
            style: if can_reopen { "cursor: pointer;" } else { "" },
            onclick: move |_| {
                if can_reopen {
                    nav.push(url.clone());
                }
            },
            div { class: "archive-item__info",
                div { class: "archive-item__title", "{action.title}" }
                div { class: "archive-item__meta",
                    "{action.action_type.translate(&lang())} · {action.credits} CR"
                }
            }
            if is_completed {
                div { class: "archive-item__check",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                        polyline { points: "22 4 12 14.01 9 11.01" }
                    }
                }
            } else {
                div { class: "archive-item__skipped", "{tr.skipped_label}" }
            }
        }
    }
}

fn quest_type_css(t: &SpaceActionType) -> &'static str {
    match t {
        SpaceActionType::Poll => "poll",
        SpaceActionType::TopicDiscussion => "discuss",
        SpaceActionType::Quiz => "quiz",
        SpaceActionType::Follow => "follow",
    }
}
