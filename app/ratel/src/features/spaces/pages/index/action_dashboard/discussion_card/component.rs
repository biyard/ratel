use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlay, ActiveActionOverlaySignal,
};
use crate::features::spaces::pages::index::*;

#[component]
pub fn DiscussionActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
    #[props(default)] is_admin: bool,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let mut overlay: ActiveActionOverlaySignal = use_context();
    let nav = use_navigator();

    let action_id = action.action_id.clone();
    let action_id_overlay = action_id.clone();
    let action_id_edit = action_id.clone();
    let prerequisite = action.prerequisite;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "quest-card quest-card--discuss",
            "data-prerequisite": prerequisite,
            "data-testid": "quest-card-{action_id}",
            "data-type": "discuss",
            onclick: move |_| {
                let discussion_id: SpacePostEntityType = action_id_overlay.clone().into();
                overlay.0.set(Some(ActiveActionOverlay::Discussion(space_id(), discussion_id)));
            },

            svg {
                class: "quest-card__hero",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "0.5",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }

            div { class: "quest-card__top",
                span { class: "quest-card__type quest-card__type--discussion",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                    }
                    "{action.action_type.translate(&lang())}"
                }
                div { class: "quest-card__top-actions",
                    if prerequisite {
                        span { class: "quest-card__badge quest-card__badge--prerequisite",
                            "{tr.required_label}"
                        }
                    }
                    if action.credits > 0 {
                        span { class: "quest-card__badge quest-card__badge--credits",
                            "+{action.credits} CR"
                        }
                    }
                    if is_admin {
                        button {
                            aria_label: "{tr.edit}",
                            class: "quest-card__edit-btn",
                            "data-testid": "quest-edit-btn-{action_id}",
                            onclick: move |e| {
                                e.stop_propagation();
                                let discussion_id: SpacePostEntityType = action_id_edit.clone().into();
                                nav.push(crate::Route::DiscussionActionEditorPage {
                                    space_id: space_id(),
                                    discussion_id,
                                });
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                                polyline { points: "14 2 14 8 20 8" }
                                line {
                                    x1: "16",
                                    x2: "8",
                                    y1: "13",
                                    y2: "13",
                                }
                                line {
                                    x1: "16",
                                    x2: "8",
                                    y1: "17",
                                    y2: "17",
                                }
                                line {
                                    x1: "10",
                                    x2: "8",
                                    y1: "9",
                                    y2: "9",
                                }
                            }
                        }
                    }
                }
            }

            div { class: "quest-card__body",
                div { class: "quest-card__title", "{action.title}" }
                if !action.description.is_empty() {
                    div {
                        class: "quest-card__desc",
                        dangerous_inner_html: "{action.description}",
                    }
                }
                div { class: "quest-card__detail",
                    div { class: "quest-detail-chip",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                        }
                        "{tr.join_discussion}"
                    }
                }
            }

            div { class: "quest-card__footer",
                div { class: "quest-card__reward",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M12 6v12" }
                        path { d: "M16 10H8" }
                    }
                    "{action.credits} CR"
                }
                button { class: "quest-card__cta quest-card__cta--start", "{tr.start_quest}" }
            }
        }
    }
}
