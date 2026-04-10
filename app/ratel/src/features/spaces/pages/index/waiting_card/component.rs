use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::*;

#[component]
pub fn WaitingCard(prereqs: Vec<SpaceActionSummary>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "waiting-card",
            "data-testid": "card-waiting",

            // Success icon
            div { class: "waiting-card__icon",
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

            span { class: "waiting-card__heading", "{tr.waiting_heading}" }
            p { class: "waiting-card__desc", "{tr.waiting_desc}" }

            // Completed checklist summary
            if !prereqs.is_empty() {
                div { class: "waiting-card__list",
                    for action in prereqs.iter() {
                        div {
                            key: "{action.action_id}",
                            class: "waiting-item",
                            div { class: "waiting-item__icon",
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
                            div { class: "waiting-item__info",
                                span { class: "waiting-item__title", "{action.title}" }
                                span { class: "waiting-item__type",
                                    "{action.action_type.translate(&lang())}"
                                }
                            }
                        }
                    }
                }
            }

            // Status badge
            div { class: "waiting-card__status",
                div { class: "waiting-card__pulse" }
                "{tr.waiting_status}"
            }
        }
    }
}
