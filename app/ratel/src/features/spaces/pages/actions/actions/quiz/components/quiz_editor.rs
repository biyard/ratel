use crate::features::spaces::pages::actions::actions::poll::components::ChoiceQuestionEditor;
use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Props, Clone, PartialEq)]
pub struct QuizEditorProps {
    pub questions: Signal<Vec<Question>>,
    pub answers: Signal<Vec<QuizCorrectAnswer>>,
    #[props(default)]
    pub on_save: Option<EventHandler<()>>,
}

#[component]
pub fn QuizEditor(props: QuizEditorProps) -> Element {
    let mut questions = props.questions;
    let mut answers = props.answers;
    let on_save = props.on_save;
    let mut selecting_question_type = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-2 pt-1 pb-5 w-full rounded-[12px]",
            for (idx , question) in questions.read().iter().enumerate() {
                {
                    let question = question.clone();
                    let answer = answers
                        .read()
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| QuizCorrectAnswer::for_question(&question));
                    let mut questions = questions;
                    let mut answers = answers;
                    let row_save = on_save.clone();
                    let delete_save = on_save.clone();
                    rsx! {
                        Card {
                            key: "quiz-editor-question-{idx}",
                            class: "flex flex-col gap-3",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-neutral-400", "Question {idx + 1}" }
                            }
                            QuizQuestionEditor {
                                question,
                                answer,
                                on_save: row_save,
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
                                    class: "flex items-center gap-1 text-quiz-editor-action text-[15px] leading-[24px] tracking-[0.5px] font-medium",
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
                                        if let Some(on_save) = &delete_save {
                                            on_save.call(());
                                        }
                                    },
                                    "Delete"
                                    icons::edit::Delete2 { class: "w-6 h-6 [&>path]:stroke-quiz-editor-icon [&>path]:fill-none" }
                                }
                            }
                        }
                    }
                }
            }

            if selecting_question_type() {
                div { class: "flex justify-center items-center w-full",
                    QuizQuestionTypeSelector {
                        on_save: on_save.clone(),
                        on_add: move |q: Question| {
                            let mut qs = questions.read().clone();
                            let mut ans = answers.read().clone();
                            let default_answer = QuizCorrectAnswer::for_question(&q);
                            qs.push(q);
                            ans.push(default_answer);
                            questions.set(qs);
                            answers.set(ans);
                            selecting_question_type.set(false);
                        },
                    }
                }
            } else {
                QuizAddQuestionButton { on_add: move |_| selecting_question_type.set(true) }
            }
        }
    }
}

#[component]
fn QuizAddQuestionButton(on_add: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex relative justify-center items-center w-full",
            Button {
                "data-testid": "quiz-add-question",
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
fn QuizQuestionTypeSelector(
    on_add: EventHandler<Question>,
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let button_class = "px-3 py-2 text-sm border rounded-lg flex items-center gap-1 border-input-box-border bg-transparent text-text-primary transition-colors duration-150 hover:bg-hover hover:text-text-primary";
    let single_save = on_save.clone();
    let multi_save = on_save.clone();
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
                        if let Some(on_save) = &single_save {
                            on_save.call(());
                        }
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
                        if let Some(on_save) = &multi_save {
                            on_save.call(());
                        }
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
    #[props(default)] on_save: Option<EventHandler<()>>,
    on_change: EventHandler<(Question, QuizCorrectAnswer)>,
) -> Element {
    match question {
        Question::SingleChoice(q) => rsx! {
            QuizChoiceEditor {
                question: q,
                is_single: true,
                answer,
                on_save,
                on_change,
            }
        },
        Question::MultipleChoice(q) => rsx! {
            QuizChoiceEditor {
                question: q,
                is_single: false,
                answer,
                on_save,
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
    #[props(default)] on_save: Option<EventHandler<()>>,
    on_change: EventHandler<(Question, QuizCorrectAnswer)>,
) -> Element {
    let normalized_answer = align_answer_with_options(answer, question.options.len(), is_single);
    let selected_options = match normalized_answer.clone() {
        QuizCorrectAnswer::Single { answer } => answer.into_iter().collect(),
        QuizCorrectAnswer::Multiple { answers } => answers,
    };
    let answer_for_change = normalized_answer.clone();
    let answer_for_toggle = normalized_answer;
    let question_for_toggle = question.clone();
    let toggle_save = on_save.clone();

    rsx! {
        ChoiceQuestionEditor {
            question: question.clone(),
            is_single,
            show_allow_other: false,
            selected_options,
            on_save,
            on_change: move |next_question: Question| {
                let options_len = match &next_question {
                    Question::SingleChoice(q) | Question::MultipleChoice(q) => q.options.len(),
                    _ => 0,
                };
                let next_answer = align_answer_with_options(
                    answer_for_change.clone(),
                    options_len,
                    is_single,
                );
                on_change.call((next_question, next_answer));
            },
            on_toggle_option: move |(opt_idx, checked): (usize, bool)| {
                let next_answer = toggle_answer(&answer_for_toggle, opt_idx, is_single, checked);
                let q_enum = if is_single {
                    Question::SingleChoice(question_for_toggle.clone())
                } else {
                    Question::MultipleChoice(question_for_toggle.clone())
                };
                on_change.call((q_enum, next_answer));
                if let Some(on_save) = &toggle_save {
                    on_save.call(());
                }
            },
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
