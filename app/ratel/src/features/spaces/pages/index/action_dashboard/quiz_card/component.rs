use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::*;

#[component]
pub fn QuizActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "quest-card quest-card--quiz",
            "data-type": "quiz",
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
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                line {
                    x1: "12",
                    x2: "12.01",
                    y1: "17",
                    y2: "17",
                }
            }

            div { class: "quest-card__top",
                span { class: "quest-card__type quest-card__type--quiz",
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
                    div { class: "quest-quiz-stats",
                        if let Some(total_q) = action.quiz_total_score {
                            div { class: "quest-quiz-stat",
                                span { class: "quest-quiz-stat__value", "{total_q}" }
                                span { class: "quest-quiz-stat__label", "{tr.questions}" }
                            }
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
