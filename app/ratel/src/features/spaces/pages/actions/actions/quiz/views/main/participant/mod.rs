mod i18n;

use super::creator::{OverviewTab, QuizCreatorTranslate, UploadTab};
use crate::features::spaces::pages::actions::actions::poll::components::QuestionViewer;
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;
use i18n::QuizParticipantTranslate;

#[component]
pub fn QuizParticipantPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    Context::init(space_id, quiz_id)?;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            h3 { {tr.page_title} }
            Tabs { default_value: "overview-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "overview-tab", {tr.overview_title} }
                    TabTrigger { index: 1usize, value: "upload-tab", {tr.upload_title} }
                    TabTrigger { index: 2usize, value: "quiz-tab", {tr.quiz_section_title} }
                }
                TabContent { index: 0usize, value: "overview-tab",
                    OverviewTab { can_edit: false }
                }
                TabContent { index: 1usize, value: "upload-tab",
                    UploadTab { can_edit: false }
                }
                TabContent { index: 2usize, value: "quiz-tab", ParticipantQuizTab {} }
            }
        }
    }
}

#[component]
fn ParticipantQuizTab() -> Element {
    let ctx = use_space_quiz_context();
    let tr: QuizParticipantTranslate = use_translate();
    let mut toast = use_toast();
    let quiz = ctx.quiz.read().clone();
    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let initial_answers = quiz.my_response.clone().unwrap_or_else(|| {
        quiz.questions
            .iter()
            .map(default_answer_for_question)
            .collect()
    });
    let mut answers = use_signal(|| initial_answers);

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let is_in_progress = now >= quiz.started_at && now <= quiz.ended_at;
    let has_passed = quiz.passed.unwrap_or(false);
    let is_failed = quiz.attempt_count > 0 && !has_passed;
    let can_submit = is_in_progress && !has_passed && quiz.attempt_count < quiz.retry_count;
    let remaining_submissions = quiz.retry_count.saturating_sub(quiz.attempt_count);
    let total_questions = quiz.questions.len();
    let score_text = quiz
        .my_score
        .map(|score| score.to_string())
        .unwrap_or_else(|| "-".to_string());

    let on_submit = move |_| {
        let mut toast = toast;
        spawn(async move {
            let req = RespondQuizRequest { answers: answers() };
            match respond_quiz(space_id(), quiz_id(), req).await {
                Ok(_) => {
                    let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                    invalidate_query(&keys);
                    toast.info(tr.submit_success);
                }
                Err(err) => {
                    error!("Failed to submit quiz response: {:?}", err);
                    toast.error(err);
                }
            }
        });
    };

    rsx! {
        div { class: "flex w-full flex-col gap-4",
            if !is_in_progress {
                Card { class: "bg-neutral-800 p-3 text-sm text-neutral-400 light:bg-input-box-bg light:text-text-secondary",
                    if now < quiz.started_at {
                        {tr.quiz_not_started}
                    } else {
                        {tr.quiz_ended}
                    }
                }
            }

            div { class: "grid gap-3 sm:grid-cols-2",
                Card { class: "border border-neutral-700 bg-neutral-900 p-4 light:border-input-box-border light:bg-input-box-bg",
                    div { class: "text-xs text-neutral-500 light:text-text-secondary",
                        "{tr.retries_label}"
                    }
                    div { class: "mt-1 text-lg font-semibold text-white light:text-text-primary",
                        "{quiz.retry_count}"
                    }
                }
                Card { class: "border border-neutral-700 bg-neutral-900 p-4 light:border-input-box-border light:bg-input-box-bg",
                    div { class: "text-xs text-neutral-500 light:text-text-secondary",
                        "{tr.remaining_submissions}"
                    }
                    div { class: "mt-1 text-lg font-semibold text-white light:text-text-primary",
                        "{remaining_submissions}"
                    }
                }
            }

            Card { class: "border border-neutral-700 bg-neutral-900 p-4 light:border-input-box-border light:bg-input-box-bg",
                div { class: "text-xs text-neutral-500 light:text-text-secondary", "{tr.score_label}" }
                div { class: "mt-1 flex items-center gap-2",
                    div { class: "text-lg font-semibold text-white light:text-text-primary",
                        if total_questions > 0 {
                            "{score_text} / {total_questions}"
                        } else {
                            "{score_text}"
                        }
                    }
                    if has_passed {
                        span { class: "inline-flex items-center rounded-full border border-green-600 bg-green-500/10 px-2 py-0.5 text-xs font-semibold text-green-400",
                            "{tr.status_pass}"
                        }
                    } else if is_failed {
                        span { class: "inline-flex items-center rounded-full border border-red-600 bg-red-500/10 px-2 py-0.5 text-xs font-semibold text-red-400",
                            "{tr.status_failed}"
                        }
                    }
                }
            }

            if quiz.questions.is_empty() {
                div { class: "flex items-center justify-center py-10 text-neutral-500 light:text-text-secondary",
                    "{tr.no_questions}"
                }
            }

            for (idx , question) in quiz.questions.iter().enumerate() {
                {
                    let question = question.clone();
                    let current_answer = answers.read().get(idx).cloned();
                    rsx! {
                        Card { class: "border border-neutral-700 bg-neutral-900 w-full p-4 light:border-input-box-border light:bg-input-box-bg",
                            QuestionViewer {
                                index: idx,
                                question,
                                answer: current_answer,
                                disabled: !can_submit,
                                on_change: move |next_answer: Answer| {
                                    let mut next = answers();
                                    if idx < next.len() {
                                        next[idx] = next_answer;
                                    }
                                    answers.set(next);
                                },
                            }
                        }
                    }
                }
            }

            if can_submit {
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "w-full",
                    onclick: on_submit,
                    {tr.btn_submit}
                }
            }
        }
    }
}

fn default_answer_for_question(question: &Question) -> Answer {
    match question {
        Question::SingleChoice(_) => Answer::SingleChoice {
            answer: None,
            other: None,
        },
        Question::MultipleChoice(_) => Answer::MultipleChoice {
            answer: Some(vec![]),
            other: None,
        },
        Question::ShortAnswer(_) => Answer::ShortAnswer { answer: None },
        Question::Subjective(_) => Answer::Subjective { answer: None },
        Question::Checkbox(_) => Answer::Checkbox {
            answer: Some(vec![]),
        },
        Question::Dropdown(_) => Answer::Dropdown { answer: None },
        Question::LinearScale(_) => Answer::LinearScale { answer: None },
    }
}
