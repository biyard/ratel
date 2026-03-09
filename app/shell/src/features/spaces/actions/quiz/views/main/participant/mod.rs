use std::collections::HashMap;

mod i18n;
use crate::features::spaces::actions::quiz::components::*;
use crate::features::spaces::actions::quiz::controllers::*;
use crate::features::spaces::actions::quiz::*;
use i18n::QuizParticipantTranslate;
use crate::features::spaces::actions::poll::components::QuestionViewer;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

#[component]
pub fn QuizParticipantPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let tr: QuizParticipantTranslate = use_translate();
    let nav = navigator();
    let mut toast = use_toast();
    let key = space_page_actions_quiz_key(&space_id, &quiz_id);

    let quiz_loader = use_query(&key, {
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        move || get_quiz(space_id.clone(), quiz_id.clone())
    })?;

    let quiz = quiz_loader.read();

    let mut answers: Signal<HashMap<usize, Answer>> = use_signal(|| {
        let mut map = HashMap::new();
        if let Some(ref my_resp) = quiz.my_response {
            for (i, ans) in my_resp.iter().enumerate() {
                map.insert(i, ans.clone());
            }
        }
        map
    });

    let now = common::utils::time::get_now_timestamp_millis();
    let is_in_progress = now >= quiz.started_at && now <= quiz.ended_at;
    let is_not_started = now < quiz.started_at;
    let is_finished = now > quiz.ended_at;

    let attempt_count = quiz.attempt_count;
    let remaining_retries = (quiz.retry_count - attempt_count).max(0);
    let has_passed = quiz.passed.unwrap_or(false);

    let can_submit = is_in_progress && attempt_count == 0 && !has_passed;
    let can_update = is_in_progress && attempt_count > 0 && remaining_retries > 0 && !has_passed;
    let can_edit = can_submit || can_update;

    let total = quiz.questions.len();

    let submit_handler = {
        let questions = quiz.questions.clone();
        let space_id = space_id.clone();
        let quiz_id = quiz_id.clone();
        let mut toast = toast;
        move |_: MouseEvent| {
            let space_id = space_id.clone();
            let quiz_id = quiz_id.clone();
            let questions = questions.clone();
            let mut toast = toast;
            spawn(async move {
                let answers_map = answers.read().clone();
                let payload: Vec<Answer> = (0..questions.len())
                    .map(|i| {
                        answers_map
                            .get(&i)
                            .cloned()
                            .unwrap_or_else(|| default_answer(&questions[i]))
                    })
                    .collect();

                let req = RespondQuizRequest { answers: payload };

                match respond_quiz(space_id.clone(), quiz_id.clone(), req).await {
                    Ok(_) => {
                        let keys = space_page_actions_quiz_key(&space_id, &quiz_id);
                        invalidate_query(&keys);
                    }
                    Err(err) => {
                        error!("Failed to submit quiz response: {:?}", err);
                        toast.error(crate::features::spaces::actions::quiz::Error::QuizResponseFailed);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            Button {
                size: ButtonSize::Inline,
                style: ButtonStyle::Text,
                class: "flex items-center gap-1 text-sm text-neutral-400 hover:text-white transition-colors w-fit",
                onclick: move |_| nav.go_back(),
                "← {tr.btn_back}"
            }

            div { class: "flex items-center justify-between gap-3",
                TimeRangeDisplay { started_at: quiz.started_at, ended_at: quiz.ended_at }
                if let Some(passed) = quiz.passed {
                    if passed || remaining_retries == 0 {
                        div { class: if passed { "px-2 py-0.5 rounded-full text-xs font-semibold border text-badge-green bg-badge-green/20 border-badge-green/30" } else { "px-2 py-0.5 rounded-full text-xs font-semibold border text-badge-red bg-badge-red/20 border-badge-red/30" },
                            if passed {
                                {tr.status_pass}
                            } else {
                                {tr.status_failed}
                            }
                        }
                    }
                }
            }

            div { class: "flex items-center gap-2 text-sm text-neutral-400",
                span { class: "font-medium", "{tr.remaining_submissions}:" }
                span { "{remaining_retries}" }
            }

            if let Some(score) = quiz.my_score {
                div { class: "flex items-center gap-3 text-sm text-neutral-400",
                    span { class: "font-medium", "{tr.score_label}" }
                    span { "{score}/{total}" }
                }
            }

            if is_finished {
                div { class: "p-3 rounded-lg bg-neutral-800 text-neutral-400 text-sm",
                    {tr.quiz_ended}
                }
            }
            if is_not_started {
                div { class: "p-3 rounded-lg bg-neutral-800 text-neutral-400 text-sm",
                    {tr.quiz_not_started}
                }
            }

            if total == 0 {
                div { class: "flex justify-center items-center py-10 text-neutral-500",
                    "No questions yet."
                }
            }

            for (idx , question) in quiz.questions.iter().enumerate() {
                {
                    let question = question.clone();
                    let current_answer = answers.read().get(&idx).cloned();
                    let display_idx = idx + 1;
                    rsx! {
                        div { class: "p-4 rounded-lg border border-neutral-700 bg-neutral-900",
                            div { class: "text-xs text-neutral-500 mb-2", "{display_idx} / {total}" }
                            QuestionViewer {
                                index: idx,
                                question,
                                answer: current_answer,
                                disabled: !can_edit,
                                on_change: move |ans: Answer| {
                                    answers.write().insert(idx, ans);
                                },
                            }
                        }
                    }
                }
            }

            if can_submit || can_update {
                div { class: "flex flex-row w-full justify-end items-end",
                    Button {
                        style: ButtonStyle::Primary,
                        class: "max-w-[200px] py-3 text-sm",
                        onclick: submit_handler,
                        if can_update {
                            {tr.btn_update}
                        } else {
                            {tr.btn_submit}
                        }
                    }
                }
            }
        }
    }
}

fn default_answer(question: &Question) -> Answer {
    match question {
        Question::SingleChoice(_) => Answer::SingleChoice {
            answer: None,
            other: None,
        },
        Question::MultipleChoice(_) => Answer::MultipleChoice {
            answer: Some(vec![]),
            other: None,
        },
        Question::Checkbox(_) => Answer::Checkbox {
            answer: Some(vec![]),
        },
        Question::Dropdown(_) => Answer::Dropdown { answer: None },
        Question::LinearScale(_) => Answer::LinearScale { answer: None },
        Question::ShortAnswer(_) => Answer::ShortAnswer { answer: None },
        Question::Subjective(_) => Answer::Subjective { answer: None },
    }
}
