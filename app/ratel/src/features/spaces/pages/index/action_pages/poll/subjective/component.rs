use crate::features::spaces::pages::actions::actions::poll::*;
use super::super::ActionPollTranslate;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollSubjective(
    idx: usize,
    question: SubjectiveQuestion,
    answer: Option<Answer>,
    disabled: bool,
    is_short: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let current_value = match &answer {
        Some(Answer::ShortAnswer { answer }) => answer.clone().unwrap_or_default(),
        Some(Answer::Subjective { answer }) => answer.clone().unwrap_or_default(),
        _ => String::new(),
    };
    let mut draft = use_signal(|| current_value.clone());
    let mut synced = use_signal(|| current_value.clone());
    use_effect(use_reactive((&current_value,), move |(cv,)| {
        if synced() != cv {
            synced.set(cv.clone());
            draft.set(cv);
        }
    }));
    let char_count = draft().len();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "subjective-wrap",
            if is_short {
                input {
                    class: "subjective-input",
                    r#type: "text",
                    placeholder: tr.subjective_placeholder,
                    disabled,
                    value: "{draft()}",
                    oninput: move |evt: Event<FormData>| {
                        let v = evt.value().to_string();
                        draft.set(v.clone());
                        on_change
                            .call(Answer::ShortAnswer {
                                answer: Some(v),
                            });
                    },
                }
            } else {
                textarea {
                    class: "subjective-textarea",
                    placeholder: tr.subjective_placeholder,
                    disabled,
                    value: "{draft()}",
                    maxlength: 2000,
                    oninput: move |evt: Event<FormData>| {
                        let v = evt.value().to_string();
                        draft.set(v.clone());
                        on_change
                            .call(Answer::Subjective {
                                answer: Some(v),
                            });
                    },
                }
                span { class: "subjective-counter", "{char_count} / 2000" }
            }
        }
    }
}
