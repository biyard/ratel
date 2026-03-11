use super::*;
use crate::features::spaces::pages::actions::actions::poll::components::QuestionViewer;

#[component]
pub fn QuizPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    can_edit: bool,
    editing: Signal<bool>,
    questions: Signal<Vec<Question>>,
    answers: Signal<Vec<QuizCorrectAnswer>>,
    original_questions: Signal<Vec<Question>>,
    original_answers: Signal<Vec<QuizCorrectAnswer>>,
    current_section: Signal<QuizCreatorSection>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let show_editor = can_edit && editing();

    let on_save = {
        move |_| {
            let answer_key = {
                let mut k = space_page_actions_quiz_key(&space_id(), &quiz_id());
                k.push("answers".into());
                k
            };
            let mut toast = toast;
            spawn(async move {
                let req = UpdateQuizRequest {
                    questions: Some(questions()),
                    answers: Some(answers()),
                    ..Default::default()
                };
                if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                    error!("Failed to save quiz: {:?}", err);
                    toast.error(err);
                    return;
                }
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
                invalidate_query(&answer_key);
                original_questions.set(questions());
                original_answers.set(answers());
                editing.set(false);
            });
        }
    };

    let on_discard = move |_| {
        questions.set(original_questions());
        answers.set(original_answers());
        editing.set(false);
    };

    let on_edit = move |_| editing.set(true);

    rsx! {
        div { class: "flex w-full max-w-[1024px] flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h3 { class: "text-[24px]/[28px] font-bold tracking-[-0.24px] text-white",
                    {tr.quiz_section_title}
                }
                p { class: "text-[15px]/[22px] font-medium text-[#D4D4D4]",
                    {tr.quiz_section_description}
                }
            }

            if can_edit {
                div { class: "flex justify-end gap-2",
                    if show_editor {
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px]",
                            onclick: on_save,
                            {tr.btn_save}
                        }
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px]",
                            onclick: on_discard,
                            {tr.btn_discard}
                        }
                    } else {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px]",
                            onclick: on_edit,
                            {tr.btn_edit}
                        }
                    }
                }
            }

            if show_editor {
                QuizEditor { questions, answers }
            } else {
                if questions.read().is_empty() {
                    div { class: "flex justify-center items-center py-10 text-neutral-500",
                        "{tr.no_questions}"
                    }
                }
                for (idx , question) in questions.read().iter().enumerate() {
                    {
                        let question = question.clone();
                        let correct_answer = answers.read().get(idx).cloned();
                        let viewer_answer = correct_answer
                            .as_ref()
                            .and_then(|a| quiz_answer_to_viewer(&question, a));
                        rsx! {
                            div { class: "rounded-lg border border-neutral-700 bg-neutral-900 p-4",
                                QuestionViewer {
                                    index: idx,
                                    question,
                                    answer: viewer_answer,
                                    disabled: true,
                                    on_change: move |_ans: Answer| {},
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex w-full justify-end gap-3",
                Button {
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: "min-w-[110px]",
                    onclick: move |_| current_section.set(QuizCreatorSection::Upload),
                    {tr.btn_back}
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "min-w-[110px]",
                    onclick: move |_| current_section.set(QuizCreatorSection::Setting),
                    "{tr.btn_next} ->"
                }
            }
        }
    }
}
