use crate::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceOptionRowProps {
    pub value: String,
    pub on_change: EventHandler<String>,
    pub on_remove: EventHandler<()>,
    #[props(default = rsx! { })]
    pub leading: Element,
}

#[component]
pub fn ChoiceOptionRow(props: ChoiceOptionRowProps) -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            {props.leading}
            Input {
                variant: InputVariant::Plain,
                class: "flex-1 p-2 bg-transparent border-b border-neutral-700 text-white placeholder-neutral-500 focus:border-blue-500 outline-none text-sm",
                value: props.value.clone(),
                oninput: move |evt: Event<FormData>| props.on_change.call(evt.value()),
            }
            Button {
                size: ButtonSize::Icon,
                style: ButtonStyle::Ghost,
                class: "text-red-400 text-xs hover:text-red-300",
                onclick: move |_| props.on_remove.call(()),
                icons::validations::Clear { class: "w-4 h-4 [&>path]:stroke-current" }
            }
        }
    }
}

#[component]
pub fn ChoiceQuestionEditor(
    question: ChoiceQuestion,
    is_single: bool,
    on_change: EventHandler<Question>,
) -> Element {
    let q = question.clone();
    rsx! {
        Input {
            variant: InputVariant::Plain,
            class: "w-full p-2 bg-transparent border-b border-neutral-600 text-white placeholder-neutral-500 focus:border-blue-500 outline-none",
            placeholder: "Question title",
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
        }
        div { class: "flex flex-col gap-1",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let question_for_input = question.clone();
                    let question_for_remove = question.clone();
                    let on_change_input = on_change.clone();
                    let on_change_remove = on_change.clone();
                    rsx! {
                        ChoiceOptionRow {
                            value: option.clone(),
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
                            },
                        }
                    }
                }
            }
            {
                let question = question.clone();
                rsx! {
                    Button {
                        size: ButtonSize::Small,
                        style: ButtonStyle::Ghost,
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
