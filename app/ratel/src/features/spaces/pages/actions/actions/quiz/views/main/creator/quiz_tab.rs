use super::*;
use crate::features::spaces::pages::actions::actions::poll::components::QuestionViewer;
use crate::features::spaces::pages::actions::actions::poll::components::TimeRangeDisplay;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

#[component]
pub fn QuizTab(can_edit: bool) -> Element {
    let ctx = use_space_quiz_context();
    let tr: QuizCreatorTranslate = use_translate();
    let toast = use_toast();
    let mut query = use_query_store();
    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let mut questions = ctx.questions;
    let mut answers = ctx.answers;
    let mut retry_count = ctx.retry_count;
    let mut pass_score = ctx.pass_score;

    rsx! {
        div { class: "flex w-full flex-col gap-6",
            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-text-secondary",
                    "{tr.survey_time_label}"
                }
                TimeRangeDisplay {
                    started_at: ctx.quiz.read().started_at,
                    ended_at: ctx.quiz.read().ended_at,
                }
            }
            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-text-secondary",
                    "{tr.pass_score_label}"
                }
                Input {
                    r#type: InputType::Number,
                    class: "text-base",
                    placeholder: tr.pass_score_placeholder,
                    value: pass_score().to_string(),
                    disabled: !can_edit,
                    attributes: vec![Attribute::new("min", "0", None, false)],
                    oninput: move |e: Event<FormData>| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            pass_score.set(v);
                        }
                    },
                    onblur: move |_| {
                        save_quiz(space_id, quiz_id, questions, answers, pass_score, retry_count, toast, query);
                    },
                    onconfirm: move |_| {
                        save_quiz(space_id, quiz_id, questions, answers, pass_score, retry_count, toast, query);
                    },
                }
            }
            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-text-secondary",
                    "{tr.retry_label}"
                }
                Input {
                    r#type: InputType::Number,
                    class: "text-base",
                    placeholder: tr.retry_placeholder,
                    value: retry_count().to_string(),
                    disabled: !can_edit,
                    attributes: vec![Attribute::new("min", "0", None, false)],
                    oninput: move |e: Event<FormData>| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            retry_count.set(v);
                        }
                    },
                    onblur: move |_| {
                        save_quiz(space_id, quiz_id, questions, answers, pass_score, retry_count, toast, query);
                    },
                    onconfirm: move |_| {
                        save_quiz(space_id, quiz_id, questions, answers, pass_score, retry_count, toast, query);
                    },
                }
            }

            if can_edit {
                QuizEditor {
                    questions,
                    answers,
                    on_save: move |_| {
                        save_quiz(space_id, quiz_id, questions, answers, pass_score, retry_count, toast, query);
                    },
                }
            } else {
                if questions.read().is_empty() {
                    div { class: "flex justify-center items-center py-10 text-neutral-500 light:text-text-secondary",
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
                            div { class: "rounded-lg border border-neutral-700 bg-neutral-900 p-4 light:border-input-box-border light:bg-input-box-bg",
                                QuestionViewer {
                                    index: idx,
                                    total: questions.read().len(),
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

        }
    }
}

fn save_quiz(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    questions: Signal<Vec<Question>>,
    answers: Signal<Vec<QuizCorrectAnswer>>,
    pass_score: Signal<i64>,
    retry_count: Signal<i64>,
    mut toast: ToastService,
    mut query: QueryStore,
) {
    let answer_key = {
        let mut k = space_page_actions_quiz_key(&space_id(), &quiz_id());
        k.push("answers".into());
        k
    };
    spawn(async move {
        let req = UpdateQuizRequest {
            questions: Some(questions()),
            answers: Some(answers()),
            pass_score: Some(pass_score()),
            retry_count: Some(retry_count()),
            ..Default::default()
        };
        if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
            error!("Failed to save quiz: {:?}", err);
            toast.error(err);
            return;
        }
        let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
        query.invalidate(&keys);
        query.invalidate(&answer_key);
    });
}

fn quiz_answer_to_viewer(question: &Question, answer: &QuizCorrectAnswer) -> Option<Answer> {
    match (question, answer) {
        (Question::SingleChoice(_), QuizCorrectAnswer::Single { answer }) => {
            Some(Answer::SingleChoice {
                answer: *answer,
                other: None,
            })
        }
        (Question::MultipleChoice(_), QuizCorrectAnswer::Multiple { answers }) => {
            Some(Answer::MultipleChoice {
                answer: Some(answers.clone()),
                other: None,
            })
        }
        _ => None,
    }
}
