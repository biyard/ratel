use crate::features::spaces::pages::actions::actions::poll::components::*;
use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Props, Clone, PartialEq)]
pub struct SurveyEditorProps {
    pub questions: Signal<Vec<Question>>,
    pub on_save: EventHandler<Vec<Question>>,
}

#[component]
pub fn SurveyEditor(props: SurveyEditorProps) -> Element {
    let mut questions = props.questions;

    rsx! {
        div { class: "flex flex-col gap-2 w-full bg-[#1A1A1A] rounded-[12px] px-4 pt-1 pb-5",
            for (idx , question) in questions.read().iter().enumerate() {
                {
                    let total = questions.read().len();
                    let is_last = idx + 1 == total;
                    let question = question.clone();
                    rsx! {
                        div { class: if is_last { "flex flex-col gap-3 pb-2" } else { "flex flex-col gap-3 pb-4 border-b border-[#262626]" },
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-neutral-400", "Question {idx + 1}" }
                            }
                            QuestionEditor {
                                question: question.clone(),
                                on_change: move |q: Question| {
                                    let mut qs = questions.read().clone();
                                    qs[idx] = q;
                                    questions.set(qs);
                                },
                            }
                            div { class: "flex justify-end",
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "flex items-center gap-1 text-[#8C8C8C] text-[15px] leading-[24px] tracking-[0.5px] font-medium",
                                    onclick: move |_| {
                                        let mut qs = questions.read().clone();
                                        qs.remove(idx);
                                        questions.set(qs);
                                    },
                                    "Delete"
                                    icons::edit::Delete2 { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                                }
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
    let button_class = "px-3 py-2 text-sm border border-neutral-600 rounded-lg hover:bg-neutral-800 text-neutral-300 flex items-center gap-1";
    rsx! {
        div { class: "flex flex-wrap gap-2",
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
            Button {
                size: ButtonSize::Small,
                style: ButtonStyle::Outline,
                shape: ButtonShape::Square,
                class: button_class,
                onclick: move |_| {
                    on_add
                        .call(
                            Question::Subjective(SubjectiveQuestion {
                                title: String::new(),
                                description: String::new(),
                                is_required: Some(false),
                            }),
                        );
                },
                icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                "Subjective"
            }
            Button {
                size: ButtonSize::Small,
                style: ButtonStyle::Outline,
                shape: ButtonShape::Square,
                class: button_class,
                onclick: move |_| {
                    on_add
                        .call(
                            Question::LinearScale(LinearScaleQuestion {
                                title: String::new(),
                                description: None,
                                image_url: None,
                                min_value: 1,
                                max_value: 5,
                                min_label: "Low".to_string(),
                                max_label: "High".to_string(),
                                is_required: Some(false),
                            }),
                        );
                },
                icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                "Linear Scale"
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
            for (opt_idx , option) in question.options.iter().enumerate() {
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
            for (opt_idx , option) in question.options.iter().enumerate() {
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
