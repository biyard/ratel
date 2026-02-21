use crate::*;

#[derive(Props, Clone, PartialEq)]
pub struct SurveyEditorProps {
    pub questions: Signal<Vec<Question>>,
    pub on_save: EventHandler<Vec<Question>>,
}

#[component]
pub fn SurveyEditor(props: SurveyEditorProps) -> Element {
    let mut questions = props.questions;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            for (idx, question) in questions.read().iter().enumerate() {
                {
                    let question = question.clone();
                    rsx! {
                        div { class: "flex flex-col gap-2 p-4 border border-neutral-700 rounded-lg",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-neutral-400", "Question {idx + 1}" }
                                button {
                                    class: "text-red-400 text-sm hover:text-red-300",
                                    onclick: move |_| {
                                        let mut qs = questions.read().clone();
                                        qs.remove(idx);
                                        questions.set(qs);
                                    },
                                    "Remove"
                                }
                            }
                            QuestionEditor {
                                question: question.clone(),
                                on_change: move |q: Question| {
                                    let mut qs = questions.read().clone();
                                    qs[idx] = q;
                                    questions.set(qs);
                                },
                            }
                        }
                    }
                }
            }

            QuestionTypeSelector {
                on_add: move |q: Question| {
                    let mut qs = questions.read().clone();
                    qs.push(q);
                    questions.set(qs);
                },
            }
        }
    }
}

#[component]
fn QuestionTypeSelector(on_add: EventHandler<Question>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            button {
                class: "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300",
                onclick: move |_| {
                    on_add.call(Question::SingleChoice(ChoiceQuestion {
                        title: String::new(),
                        description: None,
                        image_url: None,
                        options: vec!["Option 1".to_string(), "Option 2".to_string()],
                        is_required: Some(false),
                        allow_other: None,
                    }));
                },
                "+ Single Choice"
            }
            button {
                class: "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300",
                onclick: move |_| {
                    on_add.call(Question::MultipleChoice(ChoiceQuestion {
                        title: String::new(),
                        description: None,
                        image_url: None,
                        options: vec!["Option 1".to_string(), "Option 2".to_string()],
                        is_required: Some(false),
                        allow_other: None,
                    }));
                },
                "+ Multiple Choice"
            }
            button {
                class: "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300",
                onclick: move |_| {
                    on_add.call(Question::Subjective(SubjectiveQuestion {
                        title: String::new(),
                        description: String::new(),
                        is_required: Some(false),
                    }));
                },
                "+ Subjective"
            }
            button {
                class: "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300",
                onclick: move |_| {
                    on_add.call(Question::LinearScale(LinearScaleQuestion {
                        title: String::new(),
                        description: None,
                        image_url: None,
                        min_value: 1,
                        max_value: 5,
                        min_label: "Low".to_string(),
                        max_label: "High".to_string(),
                        is_required: Some(false),
                    }));
                },
                "+ Linear Scale"
            }
        }
    }
}

#[component]
fn QuestionEditor(question: Question, on_change: EventHandler<Question>) -> Element {
    match question {
        Question::SingleChoice(q) => rsx! {
            ChoiceQuestionEditor { question: q, is_single: true, on_change }
        },
        Question::MultipleChoice(q) => rsx! {
            ChoiceQuestionEditor { question: q, is_single: false, on_change }
        },
        Question::ShortAnswer(q) | Question::Subjective(q) => rsx! {
            SubjectiveQuestionEditor { question: q, on_change }
        },
        Question::Checkbox(q) => rsx! {
            CheckboxQuestionEditor { question: q, on_change }
        },
        Question::Dropdown(q) => rsx! {
            DropdownQuestionEditor { question: q, on_change }
        },
        Question::LinearScale(q) => rsx! {
            LinearScaleQuestionEditor { question: q, on_change }
        },
    }
}

#[component]
fn ChoiceQuestionEditor(
    question: ChoiceQuestion,
    is_single: bool,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    rsx! {
        input {
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                if is_single {
                    on_change.call(Question::SingleChoice(next));
                } else {
                    on_change.call(Question::MultipleChoice(next));
                }
            },
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx, option) in question.options.iter().enumerate() {
                {
                    let question_for_input = question.clone();
                    let question_for_remove = question.clone();
                    let on_change_input = on_change.clone();
                    let on_change_remove = on_change.clone();
                    rsx! {
                        div { class: "flex items-center gap-2",
                            input {
                                class: "flex-1 p-2 bg-transparent border-b border-neutral-700 text-white placeholder-neutral-500 focus:border-blue-500 outline-none text-sm",
                                r#type: "text",
                                value: "{option}",
                                oninput: move |evt| {
                                    let mut next = question_for_input.clone();
                                    next.options[opt_idx] = evt.value().to_string();
                                    if is_single {
                                        on_change_input.call(Question::SingleChoice(next));
                                    } else {
                                        on_change_input.call(Question::MultipleChoice(next));
                                    }
                                },
                            }
                            button {
                                class: "text-red-400 text-xs hover:text-red-300",
                                onclick: move |_| {
                                    let mut next = question_for_remove.clone();
                                    next.options.remove(opt_idx);
                                    if is_single {
                                        on_change_remove.call(Question::SingleChoice(next));
                                    } else {
                                        on_change_remove.call(Question::MultipleChoice(next));
                                    }
                                },
                                "x"
                            }
                        }
                    }
                }
            }
            {
                let question = question.clone();
                rsx! {
                    button {
                        class: "text-sm text-blue-400 hover:text-blue-300 mt-1",
                        onclick: move |_| {
                            let mut next = question.clone();
                            next.options.push(format!("Option {}", next.options.len() + 1));
                            if is_single {
                                on_change.call(Question::SingleChoice(next));
                            } else {
                                on_change.call(Question::MultipleChoice(next));
                            }
                        },
                        "+ Add Option"
                    }
                }
            }
        }
    }
}

#[component]
fn SubjectiveQuestionEditor(
    question: SubjectiveQuestion,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    rsx! {
        input {
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Subjective(next));
            },
        }
        span { class: "text-xs text-neutral-500", "Text answer field" }
    }
}

#[component]
fn CheckboxQuestionEditor(
    question: CheckboxQuestion,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    rsx! {
        input {
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Checkbox(next));
            },
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx, option) in question.options.iter().enumerate() {
                {
                    let question = question.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        div { class: "flex items-center gap-2",
                            input {
                                class: "flex-1 p-2 bg-transparent border-b border-neutral-700 text-white text-sm outline-none",
                                r#type: "text",
                                value: "{option}",
                                oninput: move |evt| {
                                    let mut next = question.clone();
                                    next.options[opt_idx] = evt.value().to_string();
                                    on_change.call(Question::Checkbox(next));
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownQuestionEditor(
    question: DropdownQuestion,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    rsx! {
        input {
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Dropdown(next));
            },
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx, option) in question.options.iter().enumerate() {
                {
                    let question = question.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        div { class: "flex items-center gap-2",
                            input {
                                class: "flex-1 p-2 bg-transparent border-b border-neutral-700 text-white text-sm outline-none",
                                r#type: "text",
                                value: "{option}",
                                oninput: move |evt| {
                                    let mut next = question.clone();
                                    next.options[opt_idx] = evt.value().to_string();
                                    on_change.call(Question::Dropdown(next));
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LinearScaleQuestionEditor(
    question: LinearScaleQuestion,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    let min_val = question.min_value;
    let max_val = question.max_value;
    rsx! {
        input {
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::LinearScale(next));
            },
        }
        div { class: "flex items-center gap-4 text-sm text-neutral-400",
            span { "Min: {min_val}" }
            span { "Max: {max_val}" }
        }
    }
}
