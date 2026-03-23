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
    subjective_input_placeholder: {
        en: "Write your answer...",
        ko: "답변을 입력하세요...",
    },
    other_option: {
        en: "Other",
        ko: "기타",
    },
    other_placeholder: {
        en: "Write your answer...",
        ko: "응답을 입력하세요...",
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
            Question::SingleChoice(_),
            Some(Answer::SingleChoice {
                other: Some(value),
                ..
            }),
        ) => !value.trim().is_empty(),
        (
            Question::MultipleChoice(_),
            Some(Answer::MultipleChoice {
                other: Some(value),
                ..
            }),
        ) => !value.trim().is_empty(),
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
    #[props(default = false)]
    pub enable_other_option: bool,
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
        enable_other_option,
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
                        enable_other_option,
                        on_change,
                    }
                },
                Question::MultipleChoice(q) => rsx! {
                    MultipleChoiceViewer {
                        index,
                        question: q,
                        answer: answer.clone(),
                        disabled,
                        enable_other_option,
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
        div {
            class: "mb-3 flex flex-col gap-1",
            "data-question-title-wrap": true,
            div { class: "flex items-center gap-1",
                span {
                    class: "text-lg font-semibold text-poll-question-title",
                    "data-question-title": true,
                    "{title}"
                }
                if is_required.unwrap_or(false) {
                    span { class: "text-red-500", "*" }
                }
            }
            if let Some(desc) = description {
                if !desc.is_empty() {
                    p {
                        class: "text-sm text-text-primary-muted",
                        "data-question-desc": true,
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
    enable_other_option: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: QuestionViewerTranslate = use_translate();
    let selected = match &answer {
        Some(Answer::SingleChoice { answer, .. }) => *answer,
        _ => None,
    };
    let other_value = match &answer {
        Some(Answer::SingleChoice { other: Some(other), .. }) => other.clone(),
        _ => String::new(),
    };
    let other_value_for_row_click = other_value.clone();
    let other_value_for_focus = other_value.clone();
    let other_selected = matches!(
        &answer,
        Some(Answer::SingleChoice {
            other: Some(_),
            ..
        })
    );

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "grid w-full grid-cols-1 gap-3 md:grid-cols-2",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_selected = selected == Some(opt_idx as i32);
                    let opt_idx = opt_idx as i32;
                    let on_change = on_change.clone();
                    rsx! {
                        button {
                            key: "single-{index}-{opt_idx}",
                            "aria-selected": is_selected,
                            class: "group relative flex min-h-[88px] w-full items-center overflow-hidden rounded-xl text-left transition-all bg-option-card-bg hover:bg-option-card-hover-bg aria-selected:bg-gradient-to-r aria-selected:from-primary/80 aria-selected:to-primary aria-selected:shadow-[0_8px_20px_rgba(0,0,0,0.2)] aria-selected:ring-2 aria-selected:ring-primary/90",
                            class: if disabled { "cursor-not-allowed opacity-60" } else { "cursor-pointer" },
                            disabled,
                            onclick: move |_| {
                                let next = if is_selected { None } else { Some(opt_idx) };
                                on_change
                                    .call(Answer::SingleChoice {
                                        answer: next,
                                        other: None,
                                    })
                            },
                            div { class: "absolute inset-y-0 left-0 w-[72px] bg-option-card-accent group-aria-selected:bg-primary" }
                            div { class: "relative z-10 flex w-full items-center justify-between px-5 py-4",
                                div { class: "w-10 shrink-0" }
                                span {
                                    class: "text-[20px] font-semibold tracking-[0.2px] text-text-primary group-aria-selected:text-text-third",
                                    "{option}"
                                }
                                div { class: "flex w-10 shrink-0 items-center justify-center",
                                    if is_selected {
                                        div { class: "flex size-6 items-center justify-center rounded-full bg-white",
                                            icons::validations::Check { class: "size-4 [&>path]:stroke-primary" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if enable_other_option && question.allow_other.unwrap_or(false) {
            div {
                "aria-selected": other_selected,
                class: "group relative flex min-h-[88px] w-full items-center overflow-hidden rounded-xl text-left transition-all bg-option-card-bg hover:bg-option-card-hover-bg aria-selected:bg-gradient-to-r aria-selected:from-primary/80 aria-selected:to-primary aria-selected:shadow-[0_8px_20px_rgba(0,0,0,0.2)]",
                class: if disabled { "cursor-not-allowed opacity-60" } else { "cursor-pointer" },
                onclick: move |_| {
                    if disabled {
                        return;
                    }

                    on_change.call(Answer::SingleChoice {
                        answer: None,
                        other: Some(other_value_for_row_click.clone()),
                    });
                },
                div { class: "absolute inset-y-0 left-0 w-[72px] bg-option-card-accent group-aria-selected:bg-primary" }
                    div { class: "relative z-10 flex w-full items-center px-5 py-4",
                        div { class: "w-[64px] shrink-0" }
                        div { class: "flex-1 pr-4",
                        crate::common::components::Input {
                            variant: crate::common::components::InputVariant::Plain,
                                class: "h-11 w-full appearance-none border-0 bg-transparent px-0 text-left text-[18px] text-text-primary shadow-none outline-none ring-0 placeholder:text-text-primary-muted focus:border-transparent focus:outline-none focus:ring-0 focus-visible:border-transparent focus-visible:outline-none focus-visible:ring-0",
                            placeholder: if other_selected {
                                tr.other_placeholder
                            } else {
                                ""
                            },
                            value: other_value.clone(),
                            disabled,
                            onfocus: move |_| {
                                on_change.call(Answer::SingleChoice {
                                    answer: None,
                                    other: Some(other_value_for_focus.clone()),
                                });
                            },
                            oninput: move |evt: Event<FormData>| {
                                on_change.call(Answer::SingleChoice {
                                    answer: None,
                                    other: Some(evt.value().to_string()),
                                });
                            },
                        }
                        }
                        div { class: "flex w-10 shrink-0 items-center justify-center",
                        if other_selected {
                            div { class: "flex size-6 items-center justify-center rounded-full bg-white",
                                icons::validations::Check { class: "size-4 [&>path]:stroke-primary" }
                            }
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
    enable_other_option: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: QuestionViewerTranslate = use_translate();
    let selected: Vec<i32> = match &answer {
        Some(Answer::MultipleChoice { answer, .. }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };
    let other_value = match &answer {
        Some(Answer::MultipleChoice { other: Some(other), .. }) => other.clone(),
        _ => String::new(),
    };
    let selected_for_row_click = selected.clone();
    let selected_for_focus = selected.clone();
    let selected_for_input = selected.clone();
    let selected_for_clear = selected.clone();
    let other_value_for_row_click = other_value.clone();
    let other_value_for_focus = other_value.clone();
    let other_selected = matches!(
        &answer,
        Some(Answer::MultipleChoice {
            other: Some(_),
            ..
        })
    );

    rsx! {
        QuestionTitle {
            title: question.title.clone(),
            description: question.description.clone(),
            is_required: question.is_required,
        }
        div { class: "grid w-full grid-cols-1 gap-3 md:grid-cols-2",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_selected = selected.contains(&(opt_idx as i32));
                    let opt_idx = opt_idx as i32;
                    let selected = selected.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        button {
                            key: "multi-{index}-{opt_idx}",
                            "aria-selected": is_selected,
                            class: "group relative flex min-h-[88px] w-full items-center overflow-hidden rounded-xl text-left transition-all bg-option-card-bg hover:bg-option-card-hover-bg aria-selected:bg-gradient-to-r aria-selected:from-primary/80 aria-selected:to-primary aria-selected:shadow-[0_8px_20px_rgba(0,0,0,0.2)] aria-selected:ring-2 aria-selected:ring-primary/90",
                            class: if disabled { "cursor-not-allowed opacity-60" } else { "cursor-pointer" },
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
                            div { class: "absolute inset-y-0 left-0 w-[72px] bg-option-card-accent group-aria-selected:bg-primary" }
                            div { class: "relative z-10 flex w-full items-center justify-between px-5 py-4",
                                div { class: "w-10 shrink-0" }
                                span {
                                    class: "text-[20px] font-semibold tracking-[0.2px] text-text-primary group-aria-selected:text-text-third",
                                    "{option}"
                                }
                                div { class: "flex w-10 shrink-0 items-center justify-center",
                                    if is_selected {
                                        div { class: "flex size-6 items-center justify-center rounded-full bg-white",
                                            icons::validations::Check { class: "size-4 [&>path]:stroke-primary" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if enable_other_option && question.allow_other.unwrap_or(false) {
            div {
                "aria-selected": other_selected,
                class: "group relative flex min-h-[88px] w-full items-center overflow-hidden rounded-xl text-left transition-all bg-option-card-bg hover:bg-option-card-hover-bg aria-selected:bg-gradient-to-r aria-selected:from-primary/80 aria-selected:to-primary aria-selected:shadow-[0_8px_20px_rgba(0,0,0,0.2)]",
                class: if disabled { "cursor-not-allowed opacity-60" } else { "cursor-pointer" },
                onclick: move |_| {
                    if disabled {
                        return;
                    }

                    on_change.call(Answer::MultipleChoice {
                        answer: Some(selected_for_row_click.clone()),
                        other: if other_selected {
                            None
                        } else {
                            Some(other_value_for_row_click.clone())
                        },
                    });
                },
                div { class: "absolute inset-y-0 left-0 w-[72px] bg-option-card-accent group-aria-selected:bg-primary" }
                div { class: "relative z-10 flex w-full items-center px-5 py-4",
                    div { class: "w-[64px] shrink-0" }
                    div { class: "flex-1 pr-4",
                        crate::common::components::Input {
                            variant: crate::common::components::InputVariant::Plain,
                            class: "h-11 w-full appearance-none border-0 bg-transparent px-0 text-left text-[18px] text-text-primary shadow-none outline-none ring-0 placeholder:text-text-primary-muted focus:border-transparent focus:outline-none focus:ring-0 focus-visible:border-transparent focus-visible:outline-none focus-visible:ring-0",
                            placeholder: if other_selected {
                                tr.other_placeholder
                            } else {
                                ""
                            },
                            value: other_value.clone(),
                            disabled,
                            onfocus: move |_| {
                                on_change.call(Answer::MultipleChoice {
                                    answer: Some(selected_for_focus.clone()),
                                    other: Some(other_value_for_focus.clone()),
                                });
                            },
                            oninput: move |evt: Event<FormData>| {
                                on_change.call(Answer::MultipleChoice {
                                    answer: Some(selected_for_input.clone()),
                                    other: Some(evt.value().to_string()),
                                });
                            },
                        }
                    }
                    div { class: "flex w-10 shrink-0 items-center justify-center",
                        if other_selected {
                            div { class: "flex size-6 items-center justify-center rounded-full bg-white",
                                icons::validations::Check { class: "size-4 [&>path]:stroke-primary" }
                            }
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
    let tr: QuestionViewerTranslate = use_translate();
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
        div { class: "flex w-full flex-col gap-[10px]",
            QuestionTitle {
                title: question.title.clone(),
                description: Some(question.description.clone()),
                is_required: question.is_required,
            }
            if is_short {
                crate::common::components::Input {
                    variant: crate::common::components::InputVariant::Plain,
                    r#type: crate::common::components::InputType::Text,
                    class: "px-4 py-3 text-base rounded-lg border focus:border-yellow-500 focus:outline-none bg-input-box-bg border-input-box-border text-text-primary placeholder:text-neutral-600",
                    placeholder: tr.subjective_input_placeholder,
                    disabled,
                    value: "{draft()}",
                    oninput: move |evt: Event<FormData>| {
                        let next = evt.value().to_string();
                        draft.set(next.clone());
                        on_change_answer(next);
                    },
                }
            } else {
                crate::common::components::TextArea {
                    class: "px-4 py-3 text-base rounded-lg border focus:border-yellow-500 focus:outline-none bg-input-box-bg border-input-box-border min-h-[185px] text-text-primary placeholder:text-neutral-600",
                    placeholder: tr.subjective_input_placeholder,
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
                            key: "checkbox-{index}-{opt_idx}",
                            "aria-selected": is_selected,
                            class: "group flex w-full items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors border-input-box-border hover:border-border-subtle aria-selected:border-primary aria-selected:bg-primary/10",
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
                                class: "w-4 h-4 rounded border-2 flex items-center justify-center border-foreground-muted group-aria-selected:border-primary group-aria-selected:bg-primary",
                                if is_selected {
                                    icons::validations::Check { class: "size-3 [&>path]:stroke-white" }
                                }
                            }
                            span { class: "text-sm text-foreground-muted group-aria-selected:text-text-primary", "{option}" }
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
        div { class: "flex w-full flex-col gap-5 select-none",
            div { class: "flex w-full items-center justify-between text-sm font-medium text-text-primary-muted",
                span { class: "max-w-[40%] truncate", "{question.min_label}" }
                span { class: "max-w-[40%] truncate text-right", "{question.max_label}" }
            }
            div { class: "w-full",
                div { class: "flex flex-wrap justify-center gap-2 px-1 pb-1",
                    for val in min..=max {
                        {
                            let is_selected = selected == Some(val as i32);
                            let on_change = on_change.clone();
                            rsx! {
                                button {
                                    class: "flex size-10 shrink-0 items-center justify-center rounded-lg text-[15px] font-normal transition-colors",
                                    class: if is_selected { "bg-primary text-white" } else { "bg-neutral-700 text-white" },
                                    class: if disabled { "cursor-not-allowed opacity-60" } else { "cursor-pointer" },
                                    disabled,
                                    onclick: move |_| {
                                        on_change
                                            .call(Answer::LinearScale {
                                                answer: Some(val as i32),
                                            });
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
}
