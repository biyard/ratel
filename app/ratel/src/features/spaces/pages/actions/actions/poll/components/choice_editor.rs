use crate::features::spaces::pages::actions::actions::poll::*;

translate! {
    ChoiceEditorTranslate;

    allow_other: {
        en: "Allow Other",
        ko: "기타 허용",
    },
}

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionRowProps {
    pub value: String,
    pub on_change: EventHandler<String>,
    pub on_remove: EventHandler<()>,
    #[props(default = rsx! { })]
    pub leading: Element,
    #[props(default)]
    pub on_save: Option<EventHandler<()>>,
    #[props(default)]
    pub placeholder: String,
}

#[component]
pub fn ChoiceOptionRow(props: ChoiceOptionRowProps) -> Element {
    let ChoiceOptionRowProps {
        value,
        on_change,
        on_remove,
        leading,
        on_save,
        placeholder,
    } = props;

    rsx! {
        div { class: "flex items-center gap-2.5 w-full",
            {leading}
            Input {
                variant: InputVariant::Plain,
                class: "flex-1 w-full h-11 px-3 rounded-lg border bg-choice-editor-field-bg border-choice-editor-field-border text-sm text-choice-editor-field-text placeholder:text-choice-editor-field-placeholder focus:border-primary focus-visible:border-primary focus-visible:ring-0",
                value: value.clone(),
                placeholder,
                oninput: move |evt: Event<FormData>| on_change.call(evt.value()),
                onblur: move |_| {
                    if let Some(on_save) = &on_save {
                        on_save.call(());
                    }
                },
                onconfirm: move |_| {
                    if let Some(on_save) = &on_save {
                        on_save.call(());
                    }
                },
            }
            Button {
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                class: "text-choice-editor-action hover:text-choice-editor-field-text",
                onclick: move |_| on_remove.call(()),
                icons::validations::Clear { class: "w-5 h-5 [&>path]:stroke-current" }
            }
        }
    }
}

#[component]
pub fn ChoiceQuestionEditor(
    question: ChoiceQuestion,
    is_single: bool,
    on_change: EventHandler<Question>,
    #[props(default = true)]
    show_allow_other: bool,
    #[props(default)] selected_options: Vec<i32>,
    #[props(default)] on_toggle_option: Option<EventHandler<(usize, bool)>>,
    #[props(default)] on_save: Option<EventHandler<()>>,
) -> Element {
    let tr: ChoiceEditorTranslate = use_translate();
    let q = question.clone();
    let title_save = on_save.clone();
    let confirm_save = on_save.clone();
    let option_rows = question
        .options
        .iter()
        .enumerate()
        .map(|(opt_idx, option)| {
            let question_for_input = question.clone();
            let question_for_remove = question.clone();
            let on_change_input = on_change.clone();
            let on_change_remove = on_change.clone();
            let checked = selected_options.contains(&(opt_idx as i32));
            let on_toggle_option = on_toggle_option.clone();
            let remove_save = on_save.clone();
            let leading = if let Some(on_toggle_option) = on_toggle_option {
                rsx! {
                    div { class: "flex items-center gap-2.5",
                        icons::security::DialPad { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                        label { class: "flex items-center cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked,
                                class: "sr-only peer",
                                onchange: move |e| on_toggle_option.call((opt_idx, e.checked())),
                            }
                            div { class: "flex w-6 h-6 items-center justify-center rounded-[4px] border-2 border-choice-editor-field-border bg-choice-editor-selector-bg peer-checked:bg-primary peer-checked:border-primary [&>svg]:opacity-0 peer-checked:[&>svg]:opacity-100",
                                icons::validations::Check { class: "w-5 h-5 [&>path]:stroke-[#0A0A0A]" }
                            }
                        }
                    }
                }
            } else {
                rsx! {
                    icons::security::DialPad { class: "w-6 h-6 [&>path]:stroke-[#737373]" }
                }
            };

            rsx! {
                ChoiceOptionRow {
                    value: option.clone(),
                    placeholder: format!("Option {}", opt_idx + 1),
                    leading,
                    on_save: on_save.clone(),
                    on_change: move |value: String| {
                        let mut next = question_for_input.clone();
                        next.options[opt_idx] = value;
                        if is_single {
                            on_change_input.call(Question::SingleChoice(next));
                        } else {
                            on_change_input.call(Question::MultipleChoice(next));
                        }
                    },
                    on_remove: move |_| {
                        let mut next = question_for_remove.clone();
                        next.options.remove(opt_idx);
                        if is_single {
                            on_change_remove.call(Question::SingleChoice(next));
                        } else {
                            on_change_remove.call(Question::MultipleChoice(next));
                        }
                        if let Some(on_save) = &remove_save {
                            on_save.call(());
                        }
                    },
                }
            }
        })
        .collect::<Vec<_>>();

    let add_option = {
        let question = question.clone();
        let add_save = on_save.clone();
        rsx! {
            Button {
                size: ButtonSize::Small,
                style: ButtonStyle::Text,
                class: "flex w-full items-center justify-start gap-2 px-0 text-left text-sm text-choice-editor-action",
                onclick: move |_| {
                    let mut next = question.clone();
                    next.options.push(String::new());
                    if is_single {
                        on_change.call(Question::SingleChoice(next));
                    } else {
                        on_change.call(Question::MultipleChoice(next));
                    }
                    if let Some(on_save) = &add_save {
                        on_save.call(());
                    }
                },
                icons::validations::Add { class: "w-4 h-4 [&>path]:stroke-current" }
                "Add Option"
            }
        }
    };

    rsx! {
        Input {
            variant: InputVariant::Plain,
            class: "w-full h-11 px-3 rounded-lg border bg-choice-editor-field-bg border-choice-editor-field-border text-sm text-choice-editor-field-text placeholder:text-choice-editor-field-placeholder focus:border-primary focus-visible:border-primary focus-visible:ring-0",
            placeholder: "Input",
            value: q.title.clone(),
            oninput: move |evt: Event<FormData>| {
                let mut next = q.clone();
                next.title = evt.value().to_string();
                if is_single {
                    on_change.call(Question::SingleChoice(next));
                } else {
                    on_change.call(Question::MultipleChoice(next));
                }
            },
            onblur: move |_| {
                if let Some(on_save) = &title_save {
                    on_save.call(());
                }
            },
            onconfirm: move |_| {
                if let Some(on_save) = &confirm_save {
                    on_save.call(());
                }
            },
        }
        div { class: "flex flex-col gap-1",
            for row in option_rows {
                {row}
            }
            {add_option}
        }
        if show_allow_other {
            div { class: "flex w-full items-center justify-between pt-2",
                span { class: "text-sm text-choice-editor-action", {tr.allow_other} }
                Switch {
                    active: question.allow_other.unwrap_or(false),
                    on_toggle: move |_| {
                        let mut next = question.clone();
                        next.allow_other = Some(!next.allow_other.unwrap_or(false));
                        if is_single {
                            on_change.call(Question::SingleChoice(next));
                        } else {
                            on_change.call(Question::MultipleChoice(next));
                        }
                        if let Some(on_save) = &on_save {
                            on_save.call(());
                        }
                    },
                }
            }
        }
    }
}
