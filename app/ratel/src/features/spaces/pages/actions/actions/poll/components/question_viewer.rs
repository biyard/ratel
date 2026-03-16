use crate::features::spaces::pages::actions::actions::poll::*;

translate! {
    QuestionViewerTranslate;

    btn_back: {
        en: "Back",
        ko: "뒤로",
    },
    btn_next: {
        en: "Next",
        ko: "다음",
    },
}

pub fn should_auto_next(question: &Question, answer: &Answer) -> bool {
    match (question, answer) {
        (
            Question::SingleChoice(_),
            Answer::SingleChoice {
                answer: Some(_), ..
            },
        ) => true,
        (Question::Dropdown(_), Answer::Dropdown { answer: Some(_) }) => true,
        (Question::LinearScale(_), Answer::LinearScale { answer: Some(_) }) => true,
        _ => false,
    }
}

pub fn has_answer_for_question(question: &Question, answer: Option<&Answer>) -> bool {
    match (question, answer) {
        (
            Question::SingleChoice(_),
            Some(Answer::SingleChoice {
                answer: Some(_), ..
            }),
        ) => true,
        (
            Question::MultipleChoice(_),
            Some(Answer::MultipleChoice {
                answer: Some(selected),
                ..
            }),
        ) => !selected.is_empty(),
        (
            Question::ShortAnswer(_),
            Some(Answer::ShortAnswer {
                answer: Some(value),
            }),
        ) => !value.trim().is_empty(),
        (
            Question::Subjective(_),
            Some(Answer::Subjective {
                answer: Some(value),
            }),
        ) => !value.trim().is_empty(),
        (
            Question::Checkbox(_),
            Some(Answer::Checkbox {
                answer: Some(selected),
            }),
        ) => !selected.is_empty(),
        (Question::Dropdown(_), Some(Answer::Dropdown { answer: Some(_) })) => true,
        (Question::LinearScale(_), Some(Answer::LinearScale { answer: Some(_) })) => true,
        _ => false,
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct QuestionViewerProps {
    pub index: usize,
    pub total: usize,
    pub question: Question,
    pub answer: Option<Answer>,
    pub disabled: bool,
    pub on_change: EventHandler<Answer>,
    #[props(default)]
    pub on_prev: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub on_next: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub next_disabled: bool,
}

#[component]
pub fn QuestionViewer(props: QuestionViewerProps) -> Element {
    let QuestionViewerProps {
        index,
        total,
        question,
        answer,
        disabled,
        on_change,
        on_prev,
        on_next,
        next_disabled,
    } = props;
    let tr: QuestionViewerTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            match question {
                Question::SingleChoice(q) => rsx! {
                    SingleChoiceViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                    }
                },
                Question::MultipleChoice(q) => rsx! {
                    MultipleChoiceViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                    }
                },
                Question::ShortAnswer(q) => rsx! {
                    SubjectiveQuestionViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                        is_short: true,
                    }
                },
                Question::Subjective(q) => rsx! {
                    SubjectiveQuestionViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                        is_short: false,
                    }
                },
                Question::Checkbox(q) => rsx! {
                    CheckboxViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                    }
                },
                Question::Dropdown(q) => rsx! {
                    DropdownViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                    }
                },
                Question::LinearScale(q) => rsx! {
                    LinearScaleViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        on_change,
                    }
                },
            }

            if on_prev.is_some() || on_next.is_some() {
                div { class: "flex w-full items-center justify-end gap-3 pt-2",
                    if index > 0 {
                        if let Some(on_prev) = on_prev {
                            crate::common::components::Button {
                                style: crate::common::components::ButtonStyle::Outline,
                                shape: crate::common::components::ButtonShape::Square,
                                class: "min-w-[120px]",
                                onclick: on_prev,
                                {tr.btn_back}
                            }
                        }
                    }
                    if index + 1 < total {
                        crate::common::components::Button {
                            style: crate::common::components::ButtonStyle::Outline,
                            shape: crate::common::components::ButtonShape::Square,
                            class: "min-w-[120px]",
                            disabled: next_disabled,
                            onclick: move |e| {
                                if let Some(handler) = &on_next {
                                    handler.call(e);
                                }
                            },
                            {tr.btn_next}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionTitle(title: String, description: Option<String>, is_required: Option<bool>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 mb-3",
            div { class: "flex items-center gap-1",
                span { class: "font-semibold text-lg text-white light:text-text-primary",
                    "{title}"
                }
                if is_required.unwrap_or(false) {
                    span { class: "text-red-500", "*" }
                }
            }
            if let Some(desc) = description {
                if !desc.is_empty() {
                    p { class: "text-sm text-neutral-400 light:text-text-secondary",
                        "{desc}"
                    }
                }
            }
        }
    }
}

#[component]
fn SingleChoiceViewer(
    index: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::SingleChoice { answer, .. }) => *answer,
        _ => None,
    };

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "flex flex-col w-full gap-2",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_selected = selected == Some(opt_idx as i32);
                    let opt_idx = opt_idx as i32;
                    let on_change = on_change.clone();
                    rsx! {
                        button {
                            class: "flex w-full items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors",
                            class: if is_selected { "border-blue-500 bg-blue-500/10" } else { "border-neutral-700 hover:border-neutral-500 light:border-input-box-border light:hover:border-input-box-border" },
                            disabled,
                            onclick: move |_| {
                                let next = if is_selected { None } else { Some(opt_idx) };
                                on_change
                                    .call(Answer::SingleChoice {
                                        answer: next,
                                        other: None,
                                    })
                            },
                            div {
                                class: "w-4 h-4 rounded-full border-2 flex items-center justify-center",
                                class: if is_selected { "border-blue-500" } else { "border-neutral-500 light:border-input-box-border" },
                                if is_selected {
                                    div { class: "w-2 h-2 rounded-full bg-blue-500" }
                                }
                            }
                            span { class: "text-sm text-neutral-300 light:text-text-primary", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MultipleChoiceViewer(
    index: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected: Vec<i32> = match &answer {
        Some(Answer::MultipleChoice { answer, .. }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "flex flex-col w-full gap-2",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_selected = selected.contains(&(opt_idx as i32));
                    let opt_idx = opt_idx as i32;
                    let selected = selected.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        button {
                            class: "flex w-full items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors",
                            class: if is_selected { "border-blue-500 bg-blue-500/10" } else { "border-neutral-700 hover:border-neutral-500 light:border-input-box-border light:hover:border-input-box-border" },
                            disabled,
                            onclick: move |_| {
                                let mut next = selected.clone();
                                if next.contains(&opt_idx) {
                                    next.retain(|&x| x != opt_idx);
                                } else {
                                    next.push(opt_idx);
                                }
                                on_change
                                    .call(Answer::MultipleChoice {
                                        answer: Some(next),
                                        other: None,
                                    }
                                    )
                            },
                            div {
                                class: "w-4 h-4 rounded border-2 flex items-center justify-center",
                                class: if is_selected { "border-blue-500 bg-blue-500" } else { "border-neutral-500 light:border-input-box-border" },
                                if is_selected {
                                    span { class: "text-white text-xs", "v" }
                                }
                            }
                            span { class: "text-sm text-neutral-300 light:text-text-primary", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SubjectiveQuestionViewer(
    index: usize,
    question: SubjectiveQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
    is_short: bool,
) -> Element {
    let current_value = match &answer {
        Some(Answer::ShortAnswer { answer }) => answer.clone().unwrap_or_default(),
        Some(Answer::Subjective { answer }) => answer.clone().unwrap_or_default(),
        _ => String::new(),
    };

    let mut draft = use_signal(|| current_value.clone());
    let mut synced_value = use_signal(|| current_value.clone());

    use_effect(use_reactive((&current_value,), move |(current_value,)| {
        let next = current_value.clone();
        if synced_value() != next {
            synced_value.set(next.clone());
            draft.set(next);
        }
    }));

    let on_change_answer = move |value: String| {
        if is_short {
            on_change.call(Answer::ShortAnswer {
                answer: Some(value),
            });
        } else {
            on_change.call(Answer::Subjective {
                answer: Some(value),
            });
        }
    };

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: Some(question.description.clone()),
            is_required: question.is_required,
        }
        if is_short {
            crate::common::components::Input {
                variant: crate::common::components::InputVariant::Plain,
                class: "w-full p-3 rounded-lg border border-neutral-700 bg-transparent text-white placeholder-neutral-500 focus:border-blue-500 outline-none light:border-input-box-border light:text-text-primary light:placeholder:text-text-secondary",
                disabled,
                value: "{draft()}",
                oninput: move |evt: Event<FormData>| {
                    let next = evt.value().to_string();
                    draft.set(next.clone());
                    on_change_answer(next);
                },
            }
        } else {
            textarea {
                class: "w-full p-3 rounded-lg border border-neutral-700 bg-transparent text-white placeholder-neutral-500 focus:border-blue-500 outline-none min-h-[100px] light:border-input-box-border light:text-text-primary light:placeholder:text-text-secondary",
                disabled,
                value: "{draft()}",
                oninput: move |evt: Event<FormData>| {
                    let next = evt.value().to_string();
                    draft.set(next.clone());
                    on_change_answer(next);
                },
            }
        }
    }
}

#[component]
fn CheckboxViewer(
    index: usize,
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
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "flex flex-col gap-2",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_selected = selected.contains(&(opt_idx as i32));
                    let opt_idx = opt_idx as i32;
                    let selected = selected.clone();
                    let is_multi = question.is_multi;
                    let on_change = on_change.clone();
                    rsx! {
                        button {
                            class: "flex w-full items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors",
                            class: if is_selected { "border-blue-500 bg-blue-500/10" } else { "border-neutral-700 hover:border-neutral-500 light:border-input-box-border light:hover:border-input-box-border" },
                            disabled,
                            onclick: move |_| {
                                let mut next = selected.clone();
                                if is_multi {
                                    if next.contains(&opt_idx) {
                                        next.retain(|&x| x != opt_idx);
                                    } else {
                                        next.push(opt_idx);
                                    }
                                } else if is_selected {
                                    next.clear();
                                } else {
                                    next = vec![opt_idx];
                                }
                                on_change
                                    .call(Answer::Checkbox {
                                        answer: Some(next),
                                    });
                            },
                            div {
                                class: "w-4 h-4 rounded border-2 flex items-center justify-center",
                                class: if is_selected { "border-blue-500 bg-blue-500" } else { "border-neutral-500 light:border-input-box-border" },
                                if is_selected {
                                    span { class: "text-white text-xs", "v" }
                                }
                            }
                            span { class: "text-sm text-neutral-300 light:text-text-primary", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownViewer(
    index: usize,
    question: DropdownQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::Dropdown { answer }) => *answer,
        _ => None,
    };

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        select {
            class: "w-full p-3 rounded-lg border border-neutral-700 bg-neutral-900 text-white focus:border-blue-500 outline-none",
            disabled,
            onchange: move |evt| {
                let val: String = evt.value().to_string();
                let idx: Option<i32> = val.parse().ok();
                on_change.call(Answer::Dropdown { answer: idx });
            },
            option { value: "", selected: selected.is_none(), "Select..." }
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let opt_val = format!("{opt_idx}");
                    let is_sel = selected == Some(opt_idx as i32);
                    rsx! {
                        option { value: "{opt_val}", selected: is_sel, "{option}" }
                    }
                }
            }
        }
    }
}

#[component]
fn LinearScaleViewer(
    index: usize,
    question: LinearScaleQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::LinearScale { answer }) => *answer,
        _ => None,
    };

    let min = question.min_value;
    let max = question.max_value;

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "flex flex-col gap-2",
            div { class: "flex justify-between text-xs text-neutral-400",
                span { "{question.min_label}" }
                span { "{question.max_label}" }
            }
            div { class: "flex gap-1 justify-center flex-wrap",
                for val in min..=max {
                    {
                        let is_selected = selected == Some(val as i32);
                        let on_change = on_change.clone();
                        rsx! {
                            button {
                                class: "w-10 h-10 rounded-full border-2 flex items-center justify-center text-sm transition-colors",
                                class: if is_selected { "border-blue-500 bg-blue-500 text-white" } else { "border-neutral-600 text-neutral-400 hover:border-neutral-400" },
                                disabled,
                                onclick: move |_| {
                                    on_change
                                        .call(Answer::LinearScale {
                                            answer: Some(val as i32),
                                        })
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
