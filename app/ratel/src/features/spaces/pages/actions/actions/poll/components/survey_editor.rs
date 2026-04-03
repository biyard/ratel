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
                                        on_save.call(qs.clone());
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
                            on_save.call(qs.clone());
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
                "data-testid": "poll-add-question",
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
    let button_class = "px-3 py-2 text-sm border rounded-lg flex items-center gap-1 border-input-box-border bg-transparent text-text-primary transition-colors duration-150 hover:bg-hover hover:text-text-primary";
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
                                options: vec![String::new(), String::new()],
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
                                options: vec![String::new(), String::new()],
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
                                min_label: "Min".to_string(),
                                max_label: "Max".to_string(),
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
            ChoiceQuestionEditor {
                question: q,
                is_single: true,
                on_change,
                on_save,
            }
        },
        Question::MultipleChoice(q) => rsx! {
            ChoiceQuestionEditor {
                question: q,
                is_single: false,
                on_change,
                on_save,
            }
        },
        Question::ShortAnswer(q) | Question::Subjective(q) => rsx! {
            SubjectiveQuestionEditor { question: q, on_change, on_save }
        },
        Question::Checkbox(q) => rsx! {
            CheckboxQuestionEditor { question: q, on_change, on_save }
        },
        Question::Dropdown(q) => rsx! {
            DropdownQuestionEditor { question: q, on_change, on_save }
        },
        Question::LinearScale(q) => rsx! {
            LinearScaleQuestionEditor { question: q, on_change, on_save }
        },
    }
}

#[component]
fn SubjectiveQuestionEditor(
    question: SubjectiveQuestion,
    on_change: EventHandler<Question>,
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let q = question.clone();
    let blur_save = on_save.clone();
    let confirm_save = on_save.clone();
    rsx! {
        crate::common::components::Input {
            variant: crate::common::components::InputVariant::Plain,
            class: "p-2 w-full bg-transparent border-b outline-none border-neutral-600 text-text-primary placeholder:text-muted-foreground focus:border-blue-500",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt: Event<FormData>| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Subjective(next));
            },
            onblur: move |_| {
                if let Some(on_save) = &blur_save {
                    on_save.call(());
                }
            },
            onconfirm: move |_| {
                if let Some(on_save) = &confirm_save {
                    on_save.call(());
                }
            },
        }
        span { class: "text-xs text-text-primary-muted", "Text answer field" }
    }
}

#[component]
fn CheckboxQuestionEditor(
    question: CheckboxQuestion,
    on_change: EventHandler<Question>,
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let q = question.clone();
    let title_blur_save = on_save.clone();
    let title_confirm_save = on_save.clone();
    rsx! {
        crate::common::components::Input {
            variant: crate::common::components::InputVariant::Plain,
            class: "p-2 w-full bg-transparent border-b outline-none border-neutral-600 text-text-primary placeholder:text-muted-foreground focus:border-blue-500",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt: Event<FormData>| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Checkbox(next));
            },
            onblur: move |_| {
                if let Some(on_save) = &title_blur_save {
                    on_save.call(());
                }
            },
            onconfirm: move |_| {
                if let Some(on_save) = &title_confirm_save {
                    on_save.call(());
                }
            },
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let question = question.clone();
                    let on_change = on_change.clone();
                    let option_blur_save = on_save.clone();
                    let option_confirm_save = on_save.clone();
                    rsx! {
                        div { class: "flex gap-2 items-center",
                            crate::common::components::Input {
                                variant: crate::common::components::InputVariant::Plain,
                                class: "flex-1 p-2 text-sm bg-transparent border-b outline-none border-neutral-700 text-text-primary placeholder:text-muted-foreground",
                                value: "{option}",
                                oninput: move |evt: Event<FormData>| {
                                    let mut next = question.clone();
                                    next.options[opt_idx] = evt.value().to_string();
                                    on_change.call(Question::Checkbox(next));
                                },
                                onblur: move |_| {
                                    if let Some(on_save) = &option_blur_save {
                                        on_save.call(());
                                    }
                                },
                                onconfirm: move |_| {
                                    if let Some(on_save) = &option_confirm_save {
                                        on_save.call(());
                                    }
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
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let q = question.clone();
    let title_blur_save = on_save.clone();
    let title_confirm_save = on_save.clone();
    rsx! {
        crate::common::components::Input {
            variant: crate::common::components::InputVariant::Plain,
            class: "p-2 w-full bg-transparent border-b outline-none border-neutral-600 text-text-primary placeholder:text-muted-foreground focus:border-blue-500",
            placeholder: "Question title",
            value: "{q.title}",
            oninput: move |evt: Event<FormData>| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                on_change.call(Question::Dropdown(next));
            },
            onblur: move |_| {
                if let Some(on_save) = &title_blur_save {
                    on_save.call(());
                }
            },
            onconfirm: move |_| {
                if let Some(on_save) = &title_confirm_save {
                    on_save.call(());
                }
            },
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let question = question.clone();
                    let on_change = on_change.clone();
                    let option_blur_save = on_save.clone();
                    let option_confirm_save = on_save.clone();
                    rsx! {
                        div { class: "flex gap-2 items-center",
                            crate::common::components::Input {
                                variant: crate::common::components::InputVariant::Plain,
                                class: "flex-1 p-2 text-sm bg-transparent border-b outline-none border-neutral-700 text-text-primary placeholder:text-muted-foreground",
                                value: "{option}",
                                oninput: move |evt: Event<FormData>| {
                                    let mut next = question.clone();
                                    next.options[opt_idx] = evt.value().to_string();
                                    on_change.call(Question::Dropdown(next));
                                },
                                onblur: move |_| {
                                    if let Some(on_save) = &option_blur_save {
                                        on_save.call(());
                                    }
                                },
                                onconfirm: move |_| {
                                    if let Some(on_save) = &option_confirm_save {
                                        on_save.call(());
                                    }
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
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let mut draft = use_signal(|| question.clone());
    use_effect(use_reactive((&question,), move |(next_question,)| {
        draft.set(next_question.clone());
    }));

    let current = draft();
    let title_blur_save = on_save.clone();
    let title_confirm_save = on_save.clone();
    let min_change_save = on_save.clone();
    let max_change_save = on_save.clone();
    let min_label_blur_save = on_save.clone();
    let min_label_confirm_save = on_save.clone();
    let max_label_blur_save = on_save.clone();
    let max_label_confirm_save = on_save.clone();

    let min_options = [0_i64, 1];
    let max_options = 2_i64..=10;

    rsx! {
        crate::common::components::Input {
            class: "w-full",
            placeholder: "Question title",
            value: "{current.title}",
            oninput: move |evt: Event<FormData>| {
                let mut next = draft();
                next.title = evt.value().to_string();
                draft.set(next.clone());
                on_change.call(Question::LinearScale(next));
            },
            onblur: move |_| {
                if let Some(on_save) = &title_blur_save {
                    on_save.call(());
                }
            },
            onconfirm: move |_| {
                if let Some(on_save) = &title_confirm_save {
                    on_save.call(());
                }
            },
        }
        div { class: "flex flex-wrap gap-3 items-center text-sm text-neutral-400",
            div { class: "flex gap-2 items-center",
                crate::common::components::Select::<i64> {
                    placeholder: "Min",
                    value: Some(current.min_value),
                    on_value_change: move |value: Option<i64>| {
                        let Some(selected) = value else {
                            return;
                        };
                        let mut next = draft();
                        next.min_value = selected;
                        if next.max_value <= next.min_value {
                            next.max_value = (selected + 1).clamp(2, 10);
                        }
                        draft.set(next.clone());
                        on_change.call(Question::LinearScale(next));
                        if let Some(on_save) = &min_change_save {
                            on_save.call(());
                        }
                    },
                    SelectTrigger { min_width: "4.5rem", aria_label: "Select min value", SelectValue {} }
                    SelectList { aria_label: "Select min value",
                        SelectGroup {
                            for (idx , value) in min_options.into_iter().enumerate() {
                                SelectOption::<i64> {
                                    index: idx,
                                    value,
                                    text_value: "{value}",
                                    "{value}"
                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }
                }
            }

            span { class: "text-neutral-500", "~" }

            div { class: "flex gap-2 items-center",
                crate::common::components::Select::<i64> {
                    placeholder: "Max",
                    value: Some(current.max_value),
                    on_value_change: move |value: Option<i64>| {
                        let Some(selected) = value else {
                            return;
                        };
                        let mut next = draft();
                        next.max_value = selected.max(next.min_value + 1);
                        draft.set(next.clone());
                        on_change.call(Question::LinearScale(next));
                        if let Some(on_save) = &max_change_save {
                            on_save.call(());
                        }
                    },
                    SelectTrigger { min_width: "4.5rem", aria_label: "Select max value", SelectValue {} }
                    SelectList { aria_label: "Select max value",
                        SelectGroup {
                            for (idx , value) in max_options.clone().enumerate() {
                                SelectOption::<i64> {
                                    index: idx,
                                    value,
                                    text_value: "{value}",
                                    "{value}"
                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "flex flex-col gap-3 w-full",
            div { class: "grid grid-cols-[48px_1fr] gap-3 items-center w-full",
                span { class: "text-sm font-medium text-neutral-400 text-center", "{current.min_value}" }
                crate::common::components::Input {
                    class: "w-full",
                    placeholder: "Label (optional)",
                    value: "{current.min_label}",
                    oninput: move |evt: Event<FormData>| {
                        let mut next = draft();
                        next.min_label = evt.value().to_string();
                        draft.set(next.clone());
                        on_change.call(Question::LinearScale(next));
                    },
                    onblur: move |_| {
                        if let Some(on_save) = &min_label_blur_save {
                            on_save.call(());
                        }
                    },
                    onconfirm: move |_| {
                        if let Some(on_save) = &min_label_confirm_save {
                            on_save.call(());
                        }
                    },
                }
            }

            div { class: "grid grid-cols-[48px_1fr] gap-3 items-center w-full",
                span { class: "text-sm font-medium text-neutral-400 text-center", "{current.max_value}" }
                crate::common::components::Input {
                    class: "w-full",
                    placeholder: "Label (optional)",
                    value: "{current.max_label}",
                    oninput: move |evt: Event<FormData>| {
                        let mut next = draft();
                        next.max_label = evt.value().to_string();
                        draft.set(next.clone());
                        on_change.call(Question::LinearScale(next));
                    },
                    onblur: move |_| {
                        if let Some(on_save) = &max_label_blur_save {
                            on_save.call(());
                        }
                    },
                    onconfirm: move |_| {
                        if let Some(on_save) = &max_label_confirm_save {
                            on_save.call(());
                        }
                    },
                }
            }
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
