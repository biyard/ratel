use crate::features::spaces::pages::actions::actions::meet::MeetActionCard;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlay, ActiveActionOverlaySignal, CompletedActionCard,
};
use crate::features::spaces::pages::index::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum ActionStatus {
    Active,
    Completed,
    Skipped,
}

pub(super) fn derive_action_status(action: &SpaceActionSummary) -> ActionStatus {
    let ended = matches!(
        action.status,
        Some(crate::features::spaces::pages::actions::types::SpaceActionStatus::Finish)
    );

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
            // Any attempt — passing or failing — counts as "completed" so
            // the quiz moves into the archive where the user can review
            // their score. The archive row itself distinguishes passed
            // vs. failed-with-retries-left via a score badge.
            if action.user_participated {
                ActionStatus::Completed
            } else if ended {
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
        SpaceActionType::Meet => {
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
pub fn ActionDashboard(
    space_id: ReadSignal<SpacePartition>,
    #[props(default)] is_admin: bool,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut space_ctx = crate::features::spaces::space_common::providers::use_space_context();
    let actions = space_ctx.actions();
    let lang = use_language();
    let mut type_picker_open = use_signal(|| false);
    let nav = use_navigator();
    let mut toast = use_toast();

    let lookup = super::dependency_lock::build_action_lookup(&actions);
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
            spawn(async move {
                #[cfg(feature = "web")]
                gloo_timers::future::sleep(std::time::Duration::from_millis(1500)).await;
                completed_action.0.set(None);
                space_ctx.actions.restart();
            });
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "quest-label",
            span { class: "quest-label__title", "{tr.your_quests}" }
            span {
                class: "quest-label__info",
                aria_label: "{tr.your_quests_tooltip}",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    circle { cx: "12", cy: "12", r: "10" }
                    line {
                        x1: "12",
                        y1: "16",
                        x2: "12",
                        y2: "12",
                    }
                    line {
                        x1: "12",
                        y1: "8",
                        x2: "12.01",
                        y2: "8",
                    }
                }
                span { class: "quest-label__info-tip", "{tr.your_quests_tooltip}" }
            }
        }

        if active.is_empty() && !is_admin {
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
                button {
                    class: "carousel-arrow carousel-arrow--prev",
                    "data-testid": "carousel-prev",
                    aria_label: "{tr.prev_quest}",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                button {
                    class: "carousel-arrow carousel-arrow--next",
                    "data-testid": "carousel-next",
                    aria_label: "{tr.next_quest}",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
                div {
                    class: "carousel-track",
                    id: "carousel-track",
                    "data-archive-action": archive_action_id.clone().unwrap_or_default(),
                    for action in active.iter() {
                        {
                            let action = action.clone();
                            let key = action.action_id.clone();
                            let lock = super::dependency_lock::resolve_dependency_lock(&action, &lookup);
                            match action.action_type {
                                SpaceActionType::Poll => rsx! {
                                    PollActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                        lock: lock.clone(),
                                    }
                                },
                                SpaceActionType::TopicDiscussion => rsx! {
                                    DiscussionActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                        lock: lock.clone(),
                                    }
                                },
                                SpaceActionType::Quiz => rsx! {
                                    QuizActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                        lock: lock.clone(),
                                    }
                                },
                                SpaceActionType::Follow => rsx! {
                                    FollowActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                        lock: lock.clone(),
                                    }
                                },
                                SpaceActionType::Meet => rsx! {
                                    MeetActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                    }
                                },
                            }
                        }
                    }
                    if is_admin {
                        AddActionCard {
                            on_click: move |_| {
                                type_picker_open.set(true);
                            },
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
                if is_admin {
                    button { class: "carousel-dot", "data-type": "add" }
                }
            }
        }

        // Type picker modal (admin only)
        if is_admin {
            TypePickerModal {
                open: type_picker_open(),
                on_close: move |_| {
                    type_picker_open.set(false);
                },
                on_pick: move |ty: SpaceActionType| async move {
                    type_picker_open.set(false);
                    match ty.create(space_id()).await {
                        Ok(route) => {
                            space_ctx.current_role.set(SpaceUserRole::Creator);
                            // Refresh the actions list so the newly-created
                            // action appears when the user returns to the
                            // dashboard from the editor.
                            space_ctx.actions.restart();
                            nav.push(route);
                        }
                        Err(err) => {
                            toast.error(err);
                        }
                    }
                },
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
    // Completed actions reopen their overlay on click so participants can
    // revisit their entry. Skipped quizzes are also clickable in read-only
    // mode — the submit button inside the overlay stays disabled because
    // `can_submit` checks `attempt_count < total_allowed && !has_passed`.
    // Skipped discussions are likewise clickable so users can read the
    // thread after it ended; `can_comment` inside the overlay already
    // disables posting for viewers or ended discussions.
    let can_reopen = is_completed
        || (status == ActionStatus::Skipped
            && matches!(
                action.action_type,
                SpaceActionType::Quiz
                    | SpaceActionType::TopicDiscussion
                    | SpaceActionType::Poll
            ));
    let mut overlay: ActiveActionOverlaySignal = use_context();
    let nav = use_navigator();
    let action_id = action.action_id.clone();
    let space_id_for_click = space_id.clone();
    let action_type = action.action_type.clone();

    rsx! {
        div {
            class: "archive-item",
            style: if can_reopen { "cursor: pointer;" } else { "" },
            onclick: move |_| {
                if !can_reopen {
                    return;
                }
                let sid = space_id_for_click.clone();
                let aid = action_id.clone();
                match action_type {
                    SpaceActionType::Poll => {
                        let pid: SpacePollEntityType = aid.into();
                        overlay.0.set(Some(ActiveActionOverlay::Poll(sid, pid)));
                    }
                    SpaceActionType::Quiz => {
                        let qid: SpaceQuizEntityType = aid.into();
                        overlay.0.set(Some(ActiveActionOverlay::Quiz(sid, qid)));
                    }
                    SpaceActionType::TopicDiscussion => {
                        let did: SpacePostEntityType = aid.into();
                        nav.push(crate::Route::SpaceDiscussionPage {
                            space_id: sid,
                            discussion_id: did,
                        });
                    }
                    SpaceActionType::Follow => {} // Follow has no in-overlay detail view yet; skip.
                    SpaceActionType::Meet => {} // Meet has no in-overlay detail view yet; skip.
                }
            },
            div { class: "archive-item__info",
                div { class: "archive-item__title", "{action.title}" }
                div { class: "archive-item__meta",
                    "{action.action_type.translate(&lang())} · {action.credits} CR"
                }
            }
            {
                let is_quiz = action.action_type == SpaceActionType::Quiz;
                let quiz_passed = action.quiz_passed == Some(true);
                if is_quiz && is_completed && !quiz_passed {
                    // Failed attempt — show the score (e.g. "1 / 4").
                    let score = action.quiz_score.unwrap_or(0);
                    let total = action.quiz_total_score.unwrap_or(0);
                    rsx! {
                        div { class: "archive-item__score", "{score} / {total}" }
                    }
                } else if is_completed {
                    rsx! {
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
                    }
                } else {
                    rsx! {
                        div { class: "archive-item__skipped", "{tr.skipped_label}" }
                    }
                }
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
        SpaceActionType::Meet => "meet",
    }
}
