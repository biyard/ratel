use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::*;

#[component]
pub fn DiscussionActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "quest-card quest-card--discuss",
            "data-type": "discuss",
            "data-prerequisite": action.prerequisite,
            "data-testid": "quest-card-{action.action_id}",
            onclick: move |_| {
                let route = action.get_url(&space_id());
                nav.push(route);
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
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                    }
                    "{action.action_type.translate(&lang())}"
                }
                div { class: "quest-card__badges",
                    if action.prerequisite {
                        span { class: "quest-card__badge quest-card__badge--prerequisite",
                            "{tr.required_label}"
                        }
                    }
                    if action.credits > 0 {
                        span { class: "quest-card__badge quest-card__badge--credits",
                            "+{action.credits} CR"
                        }
                    }
                }
            }

            div { class: "quest-card__body",
                div { class: "quest-card__title", "{action.title}" }
                if !action.description.is_empty() {
                    div { class: "quest-card__desc", "{action.description}" }
                }
                div { class: "quest-card__detail",
                    div { class: "quest-detail-chip",
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
