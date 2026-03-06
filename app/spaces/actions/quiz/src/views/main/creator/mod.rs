use crate::components::*;
use crate::controllers::*;
use crate::*;
mod i18n;
use i18n::QuizCreatorTranslate;
use space_action_poll::components::QuestionViewer;
use space_common::types::space_page_actions_quiz_key;

#[component]
pub fn QuizCreatorPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let nav = navigator();
    let key = space_page_actions_quiz_key(&space_id, &quiz_id);
    let quiz_loader = use_query(&key, {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move || get_quiz(space_id.clone(), quiz_id.clone())
    })?;
    let answer_key = {
        let mut k = key.clone();
        k.push("answers".into());
        k
    };
    let answer_loader = use_query(&answer_key, {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move || get_quiz_answer(space_id.clone(), quiz_id.clone())
    })?;

    let quiz = quiz_loader.read().clone();
    let answer = answer_loader.read().clone();

    let mut editing = use_signal(|| false);
    let mut questions = use_signal(|| quiz.questions.clone());
    let mut answers = use_signal(|| align_answers(&quiz.questions, &answer.answers));
    let mut title = use_signal(|| quiz.title.clone());
    let mut started_at = use_signal(|| quiz.started_at);
    let mut ended_at = use_signal(|| quiz.ended_at);
    let mut retry_count = use_signal(|| quiz.retry_count);

    let can_edit = quiz.user_response_count == 0;

    let on_back = move |_| {
        nav.go_back();
    };

    let on_title_save = {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move |_| {
            let t = title();
            let space_id = space_id.clone();
            let quiz_id = quiz_id.clone();
            spawn(async move {
                let req = UpdateQuizRequest {
                    title: Some(t),
                    ..Default::default()
                };
                if let Err(e) = update_quiz(space_id.clone(), quiz_id.clone(), req).await {
                    error!("Failed to update title: {:?}", e);
                } else {
                    let keys = space_page_actions_quiz_key(&space_id, &quiz_id);
                    invalidate_query(&keys);
                }
            });
        }
    };

    let on_time_save = {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move |(start, end): (i64, i64)| {
            let space_id = space_id.clone();
            let quiz_id = quiz_id.clone();
            spawn(async move {
                let req = UpdateQuizRequest {
                    started_at: Some(start),
                    ended_at: Some(end),
                    ..Default::default()
                };
                if let Err(e) = update_quiz(space_id.clone(), quiz_id.clone(), req).await {
                    error!("Failed to update time range: {:?}", e);
                } else {
                    let keys = space_page_actions_quiz_key(&space_id, &quiz_id);
                    invalidate_query(&keys);
                }
            });
        }
    };

    let on_retry_save = {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move |_| {
            let retry = retry_count();
            let space_id = space_id.clone();
            let quiz_id = quiz_id.clone();
            spawn(async move {
                let req = UpdateQuizRequest {
                    retry_count: Some(retry),
                    ..Default::default()
                };
                if let Err(e) = update_quiz(space_id.clone(), quiz_id.clone(), req).await {
                    error!("Failed to update retry count: {:?}", e);
                } else {
                    let keys = space_page_actions_quiz_key(&space_id, &quiz_id);
                    invalidate_query(&keys);
                }
            });
        }
    };

    let on_save = {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move |_| {
            let qs = questions.read().clone();
            let ans = answers.read().clone();
            let space_id = space_id.clone();
            let quiz_id = quiz_id.clone();
            let answer_key = {
                let mut k = space_page_actions_quiz_key(&space_id, &quiz_id);
                k.push("answers".into());
                k
            };
            spawn(async move {
                let req = UpdateQuizRequest {
                    questions: Some(qs),
                    answers: Some(ans),
                    ..Default::default()
                };
                if let Err(e) = update_quiz(space_id.clone(), quiz_id.clone(), req).await {
                    error!("Failed to save quiz: {:?}", e);
                    return;
                }
                let keys = space_page_actions_quiz_key(&space_id, &quiz_id);
                invalidate_query(&keys);
                invalidate_query(&answer_key);
                editing.set(false);
            });
        }
    };

    let on_discard = {
        let quiz_questions = quiz.questions.clone();
        let quiz_answers = answer.answers.clone();
        move |_| {
            questions.set(quiz_questions.clone());
            answers.set(align_answers(&quiz_questions, &quiz_answers));
            editing.set(false);
        }
    };

    let on_edit = move |_| {
        editing.set(true);
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            Button {
                size: ButtonSize::Inline,
                style: ButtonStyle::Ghost,
                class: "flex items-center gap-1 text-sm text-neutral-400 hover:text-white transition-colors w-fit",
                onclick: on_back,
                "← {tr.btn_back}"
            }

            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                    "{tr.title_label}"
                }
                Input {
                    class: "text-base",
                    placeholder: tr.title_placeholder,
                    value: title(),
                    oninput: move |e: Event<FormData>| title.set(e.value()),
                    onblur: on_title_save,
                }
            }

            TimeRangeSetting {
                started_at: started_at(),
                ended_at: ended_at(),
                on_change: move |(start, end)| {
                    started_at.set(start);
                    ended_at.set(end);
                    on_time_save((start, end));
                },
            }

            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                    "{tr.retry_label}"
                }
                Input {
                    r#type: "number".to_string(),
                    class: "text-base",
                    placeholder: tr.retry_placeholder,
                    value: retry_count().to_string(),
                    attributes: vec![Attribute::new("min", "0", None, false)],
                    oninput: move |e: Event<FormData>| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            retry_count.set(v);
                        }
                    },
                    onblur: on_retry_save,
                }
            }

            if can_edit {
                div { class: "flex justify-end gap-2",
                    if editing() {
                        Button {
                            style: ButtonStyle::Primary,
                            class: "text-sm",
                            onclick: on_save,
                            "{tr.btn_save}"
                        }
                        Button {
                            style: ButtonStyle::Outline,
                            class: "text-sm",
                            onclick: on_discard,
                            "{tr.btn_discard}"
                        }
                    } else {
                        Button {
                            style: ButtonStyle::Outline,
                            class: "text-sm",
                            onclick: on_edit,
                            "{tr.btn_edit}"
                        }
                    }
                }
            }

            if editing() {
                QuizEditor { questions, answers }
            } else {
                if quiz.questions.is_empty() {
                    div { class: "flex justify-center items-center py-10 text-neutral-500",
                        "{tr.no_questions}"
                    }
                }
                for (idx , question) in quiz.questions.iter().enumerate() {
                    {
                        let question = question.clone();
                        let correct_answer = answer.answers.get(idx).cloned();
                        let viewer_answer = correct_answer
                            .as_ref()
                            .and_then(|a| quiz_answer_to_viewer(&question, a));
                        rsx! {
                            div { class: "p-4 rounded-lg border border-neutral-700 bg-neutral-900",
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
        }
    }
}

fn align_answers(questions: &[Question], answers: &[QuizCorrectAnswer]) -> Vec<QuizCorrectAnswer> {
    let mut next = Vec::with_capacity(questions.len());
    for (idx, question) in questions.iter().enumerate() {
        let answer = answers
            .get(idx)
            .cloned()
            .unwrap_or_else(|| QuizCorrectAnswer::for_question(question));
        let aligned = match (question, answer) {
            (Question::MultipleChoice(_), QuizCorrectAnswer::Multiple { answers }) => {
                QuizCorrectAnswer::Multiple { answers }
            }
            (Question::SingleChoice(_), QuizCorrectAnswer::Single { answer }) => {
                QuizCorrectAnswer::Single { answer }
            }
            (Question::MultipleChoice(_), _) => QuizCorrectAnswer::Multiple { answers: vec![] },
            _ => QuizCorrectAnswer::Single { answer: None },
        };
        next.push(aligned);
    }
    next
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
