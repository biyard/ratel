use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlay, ActiveActionOverlaySignal,
};
use crate::features::spaces::pages::index::*;

#[component]
pub fn PrerequisiteCard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();
    let mut overlay: ActiveActionOverlaySignal = use_context();

    let actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let actions = actions();

    let prereqs: Vec<SpaceActionSummary> = actions
        .iter()
        .filter(|a| a.prerequisite)
        .cloned()
        .collect();

    let done_count = prereqs.iter().filter(|a| a.user_participated).count();
    let total_count = prereqs.len();
    let progress_pct = if total_count > 0 {
        (done_count as f64 / total_count as f64 * 100.0) as u32
    } else {
        0
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "prereq-card", "data-testid": "card-prerequisite",
            span { class: "prereq-card__heading", "{tr.prereq_heading}" }
            p { class: "prereq-card__desc", "{tr.prereq_desc}" }

            // Progress
            div { class: "prereq-card__progress",
                div { class: "prereq-card__progress-bar-wrap",
                    div {
                        class: "prereq-card__progress-bar",
                        style: "width: {progress_pct}%",
                    }
                }
                span { class: "prereq-card__progress-text", "{done_count} / {total_count}" }
            }

            // Action checklist
            div { class: "prereq-card__list",
                for action in prereqs.iter() {
                    {
                        let action = action.clone();
                        let is_done = action.user_participated;
                        rsx! {
                            div {
                                key: "{action.action_id}",
                                class: "prereq-item",
                                "data-done": is_done,
                                "data-testid": "prereq-item-{action.action_id}",
                                onclick: {
                                    let action = action.clone();
                                    move |_| {
                                        match action.action_type {
                                            // Poll is always clickable — the poll page handles
                                            // response editability internally.
                                            SpaceActionType::Poll => {
                                                let poll_id: SpacePollEntityType = action
                                                    .action_id
                                                    .clone()
                                                    .into();
                                                overlay
                                                    .0
                                                    .set(Some(ActiveActionOverlay::Poll(space_id(), poll_id)));
                                            }
                                            // Others stay gated by is_done.
                                            SpaceActionType::Quiz if !is_done => {
                                                let quiz_id: SpaceQuizEntityType = action
                                                    .action_id
                                                    .clone()
                                                    .into();
                                                overlay
                                                    .0
                                                    .set(Some(ActiveActionOverlay::Quiz(space_id(), quiz_id)));
                                            }
                                            SpaceActionType::TopicDiscussion if !is_done => {
                                                let discussion_id: SpacePostEntityType = action
                                                    .action_id
                                                    .clone()
                                                    .into();
                                                overlay
                                                    .0
                                                    .set(
                                                        Some(
                                                            ActiveActionOverlay::Discussion(space_id(), discussion_id),
                                                        ),
                                                    );
                                            }
                                            SpaceActionType::Follow if !is_done => {
                                                let route = action.get_url(&space_id());
                                                nav.push(route);
                                            }
                                            _ => {}
                                        }
                                    }
                                },

                                div { class: "prereq-item__icon", {action_type_icon(&action.action_type)} }

                                div { class: "prereq-item__info",
                                    span { class: "prereq-item__title", "{action.title}" }
                                    span { class: "prereq-item__type", "{action.action_type.translate(&lang())}" }
                                }

                                if is_done {
                                    div { class: "prereq-item__status prereq-item__status--done",
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
                                    div { class: "prereq-item__status prereq-item__status--pending",
                                        "{tr.prereq_pending}"
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn action_type_icon(action_type: &SpaceActionType) -> Element {
    match action_type {
        SpaceActionType::Poll => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M18 20V10" }
                path { d: "M12 20V4" }
                path { d: "M6 20v-6" }
            }
        },
        SpaceActionType::Quiz => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                line {
                    x1: "12",
                    x2: "12.01",
                    y1: "17",
                    y2: "17",
                }
            }
        },
        SpaceActionType::TopicDiscussion => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }
        },
        SpaceActionType::Follow => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                line {
                    x1: "19",
                    x2: "19",
                    y1: "8",
                    y2: "14",
                }
                line {
                    x1: "22",
                    x2: "16",
                    y1: "11",
                    y2: "11",
                }
            }
        },
    }
}
