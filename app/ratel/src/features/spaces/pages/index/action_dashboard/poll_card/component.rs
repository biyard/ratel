use crate::features::spaces::pages::actions::actions::poll::controllers::get_poll;
use crate::features::spaces::pages::actions::actions::poll::types::Question;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollActionCard(action: SpaceActionSummary, space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();

    let poll_id: SpacePollEntityType = action.action_id.clone().into();
    let poll_id = use_signal(move || poll_id);
    let poll = use_loader(move || async move { get_poll(space_id(), poll_id()).await })?;
    let poll = poll();

    // Extract first question's options for preview
    let options: Vec<String> = poll
        .questions
        .first()
        .map(|q| match q {
            Question::SingleChoice(c) => c.options.clone(),
            Question::MultipleChoice(c) => c.options.clone(),
            Question::Checkbox(c) => c.options.clone(),
            Question::Dropdown(c) => c.options.clone(),
            _ => vec![],
        })
        .unwrap_or_default();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "quest-card quest-card--poll",
            "data-type": "poll",
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
                path { d: "M18 20V10" }
                path { d: "M12 20V4" }
                path { d: "M6 20v-6" }
            }

            div { class: "quest-card__top",
                span { class: "quest-card__type quest-card__type--poll",
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
                    if !options.is_empty() {
                        div { class: "quest-poll-options",
                            for option in options.iter() {
                                div { class: "quest-poll-option",
                                    span { class: "quest-poll-option__dot" }
                                    " {option}"
                                }
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
                button { class: "quest-card__cta quest-card__cta--start", "{tr.vote_label}" }
            }
        }
    }
}
