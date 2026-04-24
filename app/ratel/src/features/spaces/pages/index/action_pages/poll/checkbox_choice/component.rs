use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollCheckbox(
    idx: usize,
    question: CheckboxQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected: Vec<i32> = match &answer {
        Some(Answer::Checkbox { answer }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };

    rsx! {
        // Uses multi_choice style.css (same CSS classes)
        document::Stylesheet { href: asset!("/src/features/spaces/pages/index/action_pages/poll/multi_choice/style.css") }
        div { class: "options-multi",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected.contains(&(opt_idx as i32));
                    let oi = opt_idx as i32;
                    let selected = selected.clone();
                    let is_multi = question.is_multi;
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "cb-{idx}-{oi}",
                            class: "option-multi",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    let mut next = selected.clone();
                                    if is_multi {
                                        if next.contains(&oi) {
                                            next.retain(|&x| x != oi);
                                        } else {
                                            next.push(oi);
                                        }
                                    } else if is_sel {
                                        next.clear();
                                    } else {
                                        next = vec![oi];
                                    }
                                    on_change
                                        .call(Answer::Checkbox {
                                            answer: Some(next),
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
        }
    }
}
