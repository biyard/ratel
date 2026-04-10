use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollSingleChoice(
    idx: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::SingleChoice { answer, .. }) => *answer,
        _ => None,
    };
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
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
        }
    }
}
