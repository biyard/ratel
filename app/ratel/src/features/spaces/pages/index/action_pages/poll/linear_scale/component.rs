use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollLinearScale(
    idx: usize,
    question: LinearScaleQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::LinearScale { answer }) => *answer,
        _ => None,
    };

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "scale-wrap",
            div { class: "scale-labels",
                span { class: "scale-label scale-label--min", {question.min_label} }
                span { class: "scale-label scale-label--max", {question.max_label} }
            }
            div { class: "scale-track",
                for val in question.min_value..=question.max_value {
                    {
                        let is_sel = selected == Some(val as i32);
                        let in_range = selected
                            .map_or(
                                false,
                                |s| (val as i32) < s && (val as i32) >= (question.min_value as i32),
                            );
                        let on_change = on_change.clone();
                        rsx! {
                            button {
                                key: "sc-{idx}-{val}",
                                class: "scale-point",
                                "data-selected": is_sel,
                                "data-in-range": in_range,
                                "data-disabled": disabled,
                                disabled,
                                onclick: move |_| {
                                    on_change
                                        .call(Answer::LinearScale {
                                            answer: Some(val as i32),
                                        });
                                },
                                "{val}"
                            }
                        }
                    }
                }
            }
        }
    }
}
