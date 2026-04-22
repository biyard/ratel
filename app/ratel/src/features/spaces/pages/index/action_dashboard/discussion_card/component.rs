use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlay, ActiveActionOverlaySignal,
};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::providers::use_space_context;

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
    let mut space_ctx = use_space_context();

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
                        QuestEditButton {
                            action_id: action_id.clone(),
                            on_edit: move |_| {
                                space_ctx.current_role.set(SpaceUserRole::Creator);
                                let discussion_id: SpacePostEntityType = action_id_edit.clone().into();
                                nav.push(crate::Route::DiscussionActionEditorPage {
                                    space_id: space_id(),
                                    discussion_id,
                                });
                            },
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
                    if let Some(count) = action.comment_count {
                        div { class: "quest-detail-chip",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" }
                            }
                            "{count} {tr.comments_count}"
                        }
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
