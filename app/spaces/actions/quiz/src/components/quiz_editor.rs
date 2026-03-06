use crate::*;
use space_action_poll::components::ChoiceOptionRow;

#[derive(Props, Clone, PartialEq)]
pub struct QuizEditorProps {
    pub questions: Signal<Vec<Question>>,
    pub answers: Signal<Vec<QuizCorrectAnswer>>,
}

#[component]
pub fn QuizEditor(props: QuizEditorProps) -> Element {
    let mut questions = props.questions;
    let mut answers = props.answers;

    rsx! {
        div { class: "flex flex-col gap-2 w-full bg-[#1A1A1A] rounded-[12px] px-4 pt-1 pb-5",
            for (idx , question) in questions.read().iter().enumerate() {
                {
                    let total = questions.read().len();
                    let is_last = idx + 1 == total;
                    let question = question.clone();
                    let answer = answers
                        .read()
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| QuizCorrectAnswer::for_question(&question));
                    let mut questions = questions;
                    let mut answers = answers;
                    rsx! {
                        div { class: if is_last { "flex flex-col gap-3 pb-2" } else { "flex flex-col gap-3 pb-4 border-b border-[#262626]" },
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-neutral-400", "Question {idx + 1}" }
                            }
                            QuizQuestionEditor {
                                question,
                                answer,
                                on_change: move |(q, a): (Question, QuizCorrectAnswer)| {
                                    let mut qs = questions.read().clone();
                                    let mut ans = answers.read().clone();
                                    if idx < qs.len() {
                                        qs[idx] = q;
                                    }
                                    if idx < ans.len() {
                                        ans[idx] = a;
                                    }
                                    questions.set(qs);
                                    answers.set(ans);
                                },
                            }
                            div { class: "flex justify-end",
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "flex items-center gap-1 text-[#8C8C8C] text-[15px] leading-[24px] tracking-[0.5px] font-medium",
                                    onclick: move |_| {
                                        let mut qs = questions.read().clone();
                                        let mut ans = answers.read().clone();
                                        if idx < qs.len() {
                                            qs.remove(idx);
                                        }
                                        if idx < ans.len() {
                                            ans.remove(idx);
                                        }
                                        questions.set(qs);
                                        answers.set(ans);
                                    },
                                    "Delete"
                                    icons::edit::Delete2 { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                                }
                            }
                        }
                    }
                }
            }

            QuizQuestionTypeSelector {
                on_add: move |q: Question| {
                    let mut qs = questions.read().clone();
                    let mut ans = answers.read().clone();
                    let default_answer = QuizCorrectAnswer::for_question(&q);
                    qs.push(q);
                    ans.push(default_answer);
                    questions.set(qs);
                    answers.set(ans);
                },
            }
        }
    }
}

#[component]
fn QuizQuestionTypeSelector(on_add: EventHandler<Question>) -> Element {
    let button_class = "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300 flex items-center gap-1";
    rsx! {
        div { class: "flex flex-wrap gap-2",
            div { class: "flex flex-row justify-start items-center gap-2",
                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: button_class,
                    onclick: move |_| {
                        on_add
                            .call(
                                Question::SingleChoice(ChoiceQuestion {
                                    title: String::new(),
                                    description: None,
                                    image_url: None,
                                    options: vec!["Option 1".to_string(), "Option 2".to_string()],
                                    is_required: Some(false),
                                    allow_other: None,
                                }),
                            );
                    },
                    icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                    "Single Choice"
                }

                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: button_class,
                    onclick: move |_| {
                        on_add
                            .call(
                                Question::MultipleChoice(ChoiceQuestion {
                                    title: String::new(),
                                    description: None,
                                    image_url: None,
                                    options: vec!["Option 1".to_string(), "Option 2".to_string()],
                                    is_required: Some(false),
                                    allow_other: None,
                                }),
                            );
                    },
                    icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                    "Multiple Choice"
                }
            }
        }
    }
}

#[component]
fn QuizQuestionEditor(
    question: Question,
    answer: QuizCorrectAnswer,
    on_change: EventHandler<(Question, QuizCorrectAnswer)>,
) -> Element {
    match question {
        Question::SingleChoice(q) => rsx! {
            QuizChoiceEditor {
                question: q,
                is_single: true,
                answer,
                on_change,
            }
        },
        Question::MultipleChoice(q) => rsx! {
            QuizChoiceEditor {
                question: q,
                is_single: false,
                answer,
                on_change,
            }
        },
        _ => rsx! {
            div { class: "text-sm text-red-400", "Only choice questions are supported." }
        },
    }
}

#[component]
fn QuizChoiceEditor(
    question: ChoiceQuestion,
    is_single: bool,
    answer: QuizCorrectAnswer,
    on_change: EventHandler<(Question, QuizCorrectAnswer)>,
) -> Element {
    let q = question.clone();
    let answer = align_answer_with_options(answer, q.options.len(), is_single);
    let answer_for_title = answer.clone();
    let q_for_title = q.clone();

    rsx! {
        Input {
            variant: InputVariant::Plain,
            class: "w-full h-11 px-3 bg-[#262626] border border-[#737373] rounded-lg text-sm text-neutral-300 placeholder:text-neutral-500 focus:border-[#FCB300] focus-visible:border-[#FCB300] focus-visible:ring-0",
            placeholder: "Input",
            value: q.title.clone(),
            oninput: move |evt: Event<FormData>| {
                let mut next = q_for_title.clone();
                next.title = evt.value().to_string();
                let q_enum = if is_single {
                    Question::SingleChoice(next)
                } else {
                    Question::MultipleChoice(next)
                };
                on_change.call((q_enum, answer_for_title.clone()));
            },
        }
        div { class: "flex flex-col gap-2",
            for (opt_idx , option) in q.options.iter().enumerate() {
                {
                    let question_for_input = q.clone();
                    let question_for_remove = q.clone();
                    let current_answer = answer.clone();
                    let on_change_input = on_change.clone();
                    let on_change_remove = on_change.clone();
                    let question_for_toggle = q.clone();
                    let answer_for_toggle = current_answer.clone();
                    let answer_for_text = current_answer.clone();
                    let answer_for_remove = current_answer.clone();
                    let on_change_toggle = on_change.clone();
                    let checked = is_option_checked(&current_answer, opt_idx, is_single);
                    rsx! {
                        ChoiceOptionRow {
                            key: "{option}-{checked}",
                            value: option.clone(),
                            leading: rsx! {
                                div { class: "flex items-center gap-2.5",
                                    icons::security::DialPad { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked,
                                            class: "sr-only peer",
                                            onchange: move |e| {
                                                let next_checked = e.checked();
                                                let next_answer = toggle_answer(
                                                    &answer_for_toggle,
                                                    opt_idx,
                                                    is_single,
                                                    next_checked,
                                                );
                                                let q_enum = if is_single {
                                                    Question::SingleChoice(question_for_toggle.clone())
                                                } else {
                                                    Question::MultipleChoice(question_for_toggle.clone())
                                                };
                                                on_change_toggle.call((q_enum, next_answer));
                                            },
                                        }
                                        div { class: "w-6 h-6 rounded-[4px] border-2 border-[#737373] bg-[#101010] flex items-center justify-center peer-checked:bg-[#FCB300] peer-checked:border-[#FCB300] [&>svg]:opacity-0 peer-checked:[&>svg]:opacity-100",
                                            icons::validations::Check { class: "w-5 h-5 [&>path]:stroke-[#0A0A0A]" }
                                        }
                                    }
                                }
                            },
                            on_change: move |value: String| {
                                let mut next = question_for_input.clone();
                                next.options[opt_idx] = value;
                                let q_enum = if is_single {
                                    Question::SingleChoice(next)
                                } else {
                                    Question::MultipleChoice(next)
                                };
                                on_change_input.call((q_enum, answer_for_text.clone()));
                            },
                            on_remove: move |_| {
                                let mut next = question_for_remove.clone();
                                next.options.remove(opt_idx);
                                let next_answer = remove_option_from_answer(
                                    &answer_for_remove,
                                    opt_idx,
                                    is_single,
                                );
                                let q_enum = if is_single {
                                    Question::SingleChoice(next)
                                } else {
                                    Question::MultipleChoice(next)
                                };
                                on_change_remove.call((q_enum, next_answer));
                            },
                        }
                    }
                }
            }
            {
                let question = q.clone();
                let current_answer = answer.clone();
                rsx! {
                    Button {
                        size: ButtonSize::Small,
                        style: ButtonStyle::Text,
                        class: "text-sm text-neutral-500 justify-start px-0 flex items-center gap-2 w-full text-left",
                        onclick: move |_| {
                            let mut next = question.clone();
                            next.options.push(format!("Option {}", next.options.len() + 1));
                            let q_enum = if is_single {
                                Question::SingleChoice(next)
                            } else {
                                Question::MultipleChoice(next)
                            };
                            on_change.call((q_enum, current_answer.clone()));
                        },
                        icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                        "Add Option"
                    }
                }
            }
        }
    }
}

fn remove_option_from_answer(
    answer: &QuizCorrectAnswer,
    removed_idx: usize,
    is_single: bool,
) -> QuizCorrectAnswer {
    if is_single {
        match answer {
            QuizCorrectAnswer::Single { answer } => {
                let next = match answer {
                    Some(v) if *v as usize == removed_idx => None,
                    Some(v) if *v as usize > removed_idx => Some(v - 1),
                    Some(v) => Some(*v),
                    None => None,
                };
                QuizCorrectAnswer::Single { answer: next }
            }
            _ => QuizCorrectAnswer::Single { answer: None },
        }
    } else {
        let mut next = match answer {
            QuizCorrectAnswer::Multiple { answers } => answers.clone(),
            _ => vec![],
        };
        next = next
            .into_iter()
            .filter_map(|v| {
                let idx = v as usize;
                if idx == removed_idx {
                    None
                } else if idx > removed_idx {
                    Some(v - 1)
                } else {
                    Some(v)
                }
            })
            .collect();
        QuizCorrectAnswer::Multiple { answers: next }
    }
}

fn is_option_checked(answer: &QuizCorrectAnswer, opt_idx: usize, is_single: bool) -> bool {
    let checked = match answer {
        QuizCorrectAnswer::Single { answer } if is_single => {
            answer.map(|v| v as usize == opt_idx).unwrap_or(false)
        }
        QuizCorrectAnswer::Multiple { answers } if !is_single => {
            answers.iter().any(|v| *v as usize == opt_idx)
        }
        _ => false,
    };

    debug!("option checked: {:?}", checked);

    checked
}

fn toggle_answer(
    answer: &QuizCorrectAnswer,
    opt_idx: usize,
    is_single: bool,
    checked: bool,
) -> QuizCorrectAnswer {
    if is_single {
        if checked {
            QuizCorrectAnswer::Single {
                answer: Some(opt_idx as i32),
            }
        } else {
            QuizCorrectAnswer::Single { answer: None }
        }
    } else {
        let mut next = match answer {
            QuizCorrectAnswer::Multiple { answers } => answers.clone(),
            _ => vec![],
        };
        let target = opt_idx as i32;
        if checked {
            if !next.contains(&target) {
                next.push(target);
            }
        } else {
            next.retain(|v| *v != target);
        }
        QuizCorrectAnswer::Multiple { answers: next }
    }
}

fn align_answer_with_options(
    answer: QuizCorrectAnswer,
    options_len: usize,
    is_single: bool,
) -> QuizCorrectAnswer {
    if is_single {
        match answer {
            QuizCorrectAnswer::Single { answer } => {
                let next = answer.and_then(|v| {
                    if v < 0 || (v as usize) >= options_len {
                        None
                    } else {
                        Some(v)
                    }
                });
                QuizCorrectAnswer::Single { answer: next }
            }
            _ => QuizCorrectAnswer::Single { answer: None },
        }
    } else {
        match answer {
            QuizCorrectAnswer::Multiple { answers } => {
                let filtered = answers
                    .into_iter()
                    .filter(|v| *v >= 0 && (*v as usize) < options_len)
                    .collect();
                QuizCorrectAnswer::Multiple { answers: filtered }
            }
            _ => QuizCorrectAnswer::Multiple { answers: vec![] },
        }
    }
}
