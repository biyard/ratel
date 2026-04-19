use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;
use super::super::ActionPollTranslate;

#[component]
pub fn PollSingleChoice(
    idx: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let selected = match &answer {
        Some(Answer::SingleChoice { answer, .. }) => *answer,
        _ => None,
    };
    let other_value = match &answer {
        Some(Answer::SingleChoice {
            other: Some(v), ..
        }) => v.clone(),
        _ => String::new(),
    };
    let other_selected = matches!(&answer, Some(Answer::SingleChoice { other: Some(_), .. }));
    let allow_other = question.allow_other.unwrap_or(false);
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let other_letter = letters.get(question.options.len()).copied().unwrap_or('?');

    rsx! {
        div { class: "options-single",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected == Some(opt_idx as i32);
                    let oi = opt_idx as i32;
                    let letter = letters.get(opt_idx).copied().unwrap_or('?');
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "sc-{idx}-{oi}",
                            class: "option-single",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    on_change
                                        .call(Answer::SingleChoice {
                                            answer: if is_sel { None } else { Some(oi) },
                                            other: None,
                                        });
                                }
                            },
                            span { class: "option-single__letter", "{letter}" }
                            div { class: "option-single__radio",
                                div { class: "option-single__radio-dot" }
                            }
                            span { class: "option-single__label", "{option}" }
                        }
                    }
                }
            }

            if allow_other {
                div {
                    key: "sc-{idx}-other",
                    class: "option-single",
                    "data-selected": other_selected,
                    "data-disabled": disabled,
                    onclick: {
                        let other_value = other_value.clone();
                        let on_change = on_change.clone();
                        move |_| {
                            if !disabled {
                                on_change
                                    .call(Answer::SingleChoice {
                                        answer: None,
                                        other: Some(other_value.clone()),
                                    });
                            }
                        }
                    },
                    span { class: "option-single__letter", "{other_letter}" }
                    div { class: "option-single__radio",
                        div { class: "option-single__radio-dot" }
                    }
                    input {
                        r#type: "text",
                        class: "option-single__other-input",
                        placeholder: tr.other_placeholder,
                        disabled,
                        value: "{other_value}",
                        onclick: move |e: Event<MouseData>| e.stop_propagation(),
                        oninput: move |evt: Event<FormData>| {
                            on_change
                                .call(Answer::SingleChoice {
                                    answer: None,
                                    other: Some(evt.value().to_string()),
                                });
                        },
                    }
                }
            }
        }
    }
}
