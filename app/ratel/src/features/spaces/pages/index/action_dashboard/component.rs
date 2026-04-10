use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::use_space;

#[component]
pub fn ActionDashboard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let actions = actions();
    let lang = use_language();

    let incomplete: Vec<_> = actions
        .iter()
        .filter(|a| !a.user_participated)
        .cloned()
        .collect();
    let completed: Vec<_> = actions
        .iter()
        .filter(|a| a.user_participated)
        .cloned()
        .collect();
    let total = actions.len();
    let done = completed.len();
    let progress_pct = if total > 0 {
        (done as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let mut show_archive = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "quest-label",
            span { class: "quest-label__title", "{tr.your_quests}" }
        }

        if incomplete.is_empty() {
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
                div { class: "carousel-track", id: "carousel-track",
                    for action in incomplete.iter() {
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
                for action in incomplete.iter() {
                    button {
                        class: "carousel-dot",
                        "data-type": quest_type_css(&action.action_type),
                    }
                }
            }
        }

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
                if done > 0 {
                    span { class: "archive-btn__count", "{done}" }
                }
            }
        }

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
                if completed.is_empty() {
                    div { class: "archive-panel__empty", "{tr.no_completed_yet}" }
                } else {
                    for action in completed.iter() {
                        div { class: "archive-item",
                            div { class: "archive-item__info",
                                div { class: "archive-item__title", "{action.title}" }
                                div { class: "archive-item__meta",
                                    "{action.action_type.translate(&lang())} · {action.credits} CR"
                                }
                            }
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
    }
}
