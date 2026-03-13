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
    let mut selecting_question_type = use_signal(|| false);
    let tr: SurveyEditorTranslate = use_translate();
    let on_save = props.on_save;

    rsx! {
        div { class: "flex flex-col gap-2 pt-1 pb-5 w-full rounded-[12px]",
            for (idx , question) in questions.read().iter().enumerate() {
                {
                    let total = questions.read().len();
                    let is_last = idx + 1 == total;
                    let question = question.clone();
                    rsx! {
                        Card { class: "flex flex-col gap-3",
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
                                on_save: move |_| {
                                    on_save.call(questions());
                                },
                            }
                            div { class: "flex justify-end",
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "flex gap-1 items-center font-medium text-text-secondary text-[15px] leading-[24px] tracking-[0.5px]",
                                    onclick: move |_| {
                                        let mut qs = questions.read().clone();
                                        qs.remove(idx);
                                        questions.set(qs);
                                    },
                                    {tr.btn_delete}
                                    icons::edit::Delete2 { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                                }
                            }
                        }
                    }
                }
            }

            if selecting_question_type() {
                div { class: "flex justify-center items-center w-full",
                    QuestionTypeSelector {
                        on_add: move |q: Question| {
                            let mut qs = questions.read().clone();
                            qs.push(q);
                            questions.set(qs);
                            selecting_question_type.set(false);
                        },
                    }
                }
            } else {
                AddQuestionButton {
                    on_add: move |_| {
                        selecting_question_type.set(true);
                    },
                }
            }

        }
    }
}

#[component]
fn AddQuestionButton(on_add: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex relative justify-center items-center w-full",
            Button {
                style: ButtonStyle::Outline,
                onclick: move |_| on_add.call(()),
                class: "flex justify-center items-center w-10 h-10 !p-0 z-2 !bg-background",
                icons::validations::Add { class: "w-[13.3px] h-[13.3px] [&>path]:stroke-current" }
            }
            Separator {
                variant: SeparatorVariant::Dashed,
                class: "absolute left-0 top-1/2 w-full z-1",
                horizontal: true,
                decorative: false,
            }
        }
    }
}

#[component]
fn PlusIcon() -> Element {
    rsx! {

        svg {
            class: "z-3 [&>path]:stoke-icon-primary",
            fill: "none",
            height: "13.3",
            view_box: "0 0 40 40",
            width: "13.3",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                fill: "#262626",
                height: "39",
                rx: "19.5",
                width: "39",
                x: "0.5",
                y: "0.5",
            }
            rect {
                height: "39",
                rx: "19.5",
                stroke: "#A1A1A1",
                width: "39",
                x: "0.5",
                y: "0.5",
            }
            path {
                d: "M13.3334 20.0002L20 20.0002M20 20.0002L26.6667 20.0002M20 20.0002V13.3335M20 20.0002L20 26.6668",
                stroke: "#A1A1A1",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
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
fn QuestionEditor(
    question: Question,
    on_change: EventHandler<Question>,
    on_save: EventHandler<()>,
) -> Element {
    match question {
        Question::SingleChoice(q) => rsx! {
            ChoiceQuestionEditor { question: q, is_single: true, on_change, on_save: Some(on_save) }
        },
        Question::MultipleChoice(q) => rsx! {
            ChoiceQuestionEditor { question: q, is_single: false, on_change, on_save: Some(on_save) }
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
            class: "p-2 w-full text-white bg-transparent border-b outline-none focus:border-blue-500 border-neutral-600 placeholder-neutral-500",
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
            class: "p-2 w-full text-white bg-transparent border-b outline-none focus:border-blue-500 border-neutral-600 placeholder-neutral-500",
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
                        div { class: "flex gap-2 items-center",
                            input {
                                class: "flex-1 p-2 text-sm text-white bg-transparent border-b outline-none border-neutral-700",
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
            class: "p-2 w-full text-white bg-transparent border-b outline-none focus:border-blue-500 border-neutral-600 placeholder-neutral-500",
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
                        div { class: "flex gap-2 items-center",
                            input {
                                class: "flex-1 p-2 text-sm text-white bg-transparent border-b outline-none border-neutral-700",
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
            class: "p-2 w-full text-white bg-transparent border-b outline-none focus:border-blue-500 border-neutral-600 placeholder-neutral-500",
            r#type: "text",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::LinearScale(next));
            },
        }
        div { class: "flex gap-4 items-center text-sm text-neutral-400",
            span { "Min: {min_val}" }
            span { "Max: {max_val}" }
        }
    }
}

translate! {
    SurveyEditorTranslate;

    btn_delete: {
        en: "Delete",
        ko: "삭제하기",
    },
}
