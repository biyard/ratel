use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;
use super::super::ActionPollTranslate;

#[component]
pub fn PollMultipleChoice(
    idx: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let selected: Vec<i32> = match &answer {
        Some(Answer::MultipleChoice { answer, .. }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };
    let other_value = match &answer {
        Some(Answer::MultipleChoice {
            other: Some(v), ..
        }) => v.clone(),
        _ => String::new(),
    };
    let other_selected = matches!(&answer, Some(Answer::MultipleChoice { other: Some(_), .. }));
    let allow_other = question.allow_other.unwrap_or(false);

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "options-multi",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected.contains(&(opt_idx as i32));
                    let oi = opt_idx as i32;
                    let selected_inner = selected.clone();
                    let other_value_inner = other_value.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "mc-{idx}-{oi}",
                            class: "option-multi",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    let mut next = selected_inner.clone();
                                    if next.contains(&oi) {
                                        next.retain(|&x| x != oi);
                                    } else {
                                        next.push(oi);
                                    }
                                    on_change
                                        .call(Answer::MultipleChoice {
                                            answer: Some(next),
                                            other: if other_value_inner.trim().is_empty() {
                                                None
                                            } else {
                                                Some(other_value_inner.clone())
                                            },
                                        });
                                }
                            },
                            div { class: "option-multi__checkbox",
                                span { class: "option-multi__check",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "3",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        polyline { points: "20 6 9 17 4 12" }
                                    }
                                }
                            }
                            span { class: "option-multi__label", "{option}" }
                        }
                    }
                }
            }

            if allow_other {
                div {
                    key: "mc-{idx}-other",
                    class: "option-multi",
                    "data-selected": other_selected,
                    "data-disabled": disabled,
                    onclick: {
                        let selected = selected.clone();
                        let other_value = other_value.clone();
                        let on_change = on_change.clone();
                        move |_| {
                            if !disabled {
                                on_change
                                    .call(Answer::MultipleChoice {
                                        answer: Some(selected.clone()),
                                        other: if other_selected {
                                            None
                                        } else {
                                            Some(other_value.clone())
                                        },
                                    });
                            }
                        }
                    },
                    div { class: "option-multi__checkbox",
                        span { class: "option-multi__check",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "3",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                    input {
                        r#type: "text",
                        class: "option-multi__other-input",
                        placeholder: tr.other_placeholder,
                        disabled,
                        value: "{other_value}",
                        onclick: move |e: Event<MouseData>| e.stop_propagation(),
                        oninput: {
                            let selected = selected.clone();
                            move |evt: Event<FormData>| {
                                on_change
                                    .call(Answer::MultipleChoice {
                                        answer: Some(selected.clone()),
                                        other: Some(evt.value().to_string()),
                                    });
                            }
                        },
                    }
                }
            }
        }
    }
}
