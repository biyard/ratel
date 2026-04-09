use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::common::utils::time::time_ago;
use crate::features::spaces::pages::actions::actions::poll::components::{
    has_answer_for_question, should_auto_next, QuestionViewer,
};
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::pages::actions::gamification::components::quest_briefing::QuestBriefing;
use crate::features::spaces::pages::actions::gamification::types::{
    QuestNodeStatus, QuestNodeView,
};
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::pages::apps::apps::file::components::FileCard;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::types::{
    space_my_score_key, space_page_actions_quiz_key, space_ranking_key,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum QuizReadStep {
    Overview,
    Quiz,
}

translate! {
    QuizReadTranslate;

    btn_next: {
        en: "Next",
        ko: "다음",
    },
    btn_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    page_title: {
        en: "Quiz",
        ko: "퀴즈",
    },
    btn_back: {
        en: "Back",
        ko: "뒤로",
    },
    btn_submit: {
        en: "Submit",
        ko: "제출",
    },
    quiz_ended: {
        en: "This quiz has ended.",
        ko: "이 퀴즈가 종료되었습니다.",
    },
    quiz_not_started: {
        en: "This quiz has not started yet.",
        ko: "이 퀴즈는 아직 시작되지 않았습니다.",
    },
    space_not_active: {
        en: "This action is available only when the space is started or in progress.",
        ko: "이 액션은 스페이스가 시작/진행중 상태일 때만 참여할 수 있습니다.",
    },
    submit_success: {
        en: "Response submitted successfully.",
        ko: "응답이 성공적으로 제출되었습니다.",
    },
    remaining_submissions: {
        en: "Remaining submissions",
        ko: "남은 제출 횟수",
    },
    score_label: {
        en: "Score",
        ko: "점수",
    },
    no_questions: {
        en: "No questions yet.",
        ko: "아직 질문이 없습니다.",
    },
    status_pass: {
        en: "PASS",
        ko: "PASS",
    },
    status_failed: {
        en: "FAILED",
        ko: "FAILED",
    },
    question_label: {
        en: "Question",
        ko: "질문",
    },
    no_permission: {
        en: "You do not have permission to participate in this quiz.",
        ko: "이 퀴즈에 참여할 권한이 없습니다.",
    },
    already_passed: {
        en: "You have already passed this quiz.",
        ko: "이미 이 퀴즈를 통과했습니다.",
    },
    no_remaining_attempts: {
        en: "You have no remaining attempts for this quiz.",
        ko: "남은 참여 횟수가 없습니다.",
    },
}

#[component]
pub fn QuizReadPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    can_respond: bool,
) -> Element {
    let i18n: QuizReadTranslate = use_translate();
    let ctx = Context::init(space_id, quiz_id)?;
    let quiz = ctx.quiz.read().clone();
    let mut query = use_query_store();
    let mut toast = use_toast();
    let mut step = use_signal(|| QuizReadStep::Overview);
    let mut question_index = use_signal(|| 0usize);
    let nav = navigator();
    let space = use_space().read().clone();

    let initial_answers = quiz.my_response.clone().unwrap_or_else(|| {
        quiz.questions
            .iter()
            .map(default_answer_for_question)
            .collect()
    });
    let mut answers = use_signal(|| initial_answers);
    let all_answered = use_memo({
        let quiz = quiz.clone();
        move || {
            if quiz.questions.len() == 0 {
                return false;
            }
            let answers_read = answers.read();
            quiz.questions
                .iter()
                .enumerate()
                .all(|(idx, question)| has_answer_for_question(question, answers_read.get(idx)))
        }
    });

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let is_in_progress = now >= quiz.started_at && now <= quiz.ended_at;
    let role = use_space_role()();
    let chapter = crate::features::spaces::pages::actions::default_chapter_for_legacy_action(role);
    let can_execute_action = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        &chapter,
        true,
        true,
        space.status,
        space.join_anytime,
    );
    let has_passed = quiz.passed.unwrap_or(false);
    let total_allowed = quiz.retry_count.saturating_add(1);
    let can_submit = can_respond
        && can_execute_action
        && is_in_progress
        && !has_passed
        && quiz.attempt_count < total_allowed;
    let remaining_submissions = total_allowed.saturating_sub(quiz.attempt_count);
    let total_questions = quiz.questions.len();
    let current_idx = question_index().min(total_questions.saturating_sub(1));
    let current_question = quiz.questions.get(current_idx).cloned();
    let current_answer = answers.read().get(current_idx).cloned();
    let has_current_answer = current_question
        .as_ref()
        .map(|q| has_answer_for_question(q, current_answer.as_ref()))
        .unwrap_or(false);
    let is_first_question = total_questions == 0 || current_idx == 0;
    let is_last_question = total_questions == 0 || current_idx + 1 >= total_questions;
    let quiz_next_disabled = can_submit && !has_current_answer;

    let on_submit = move |_| {
        let req = RespondQuizRequest { answers: answers() };
        let nav = nav.clone();
        spawn(async move {
            match respond_quiz(space_id(), quiz_id(), req).await {
                Ok(_) => {
                    let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                    query.invalidate(&keys);
                    query.invalidate(&space_ranking_key(&space_id()));
                    query.invalidate(&space_my_score_key(&space_id()));
                    toast.info(i18n.submit_success);
                    nav.push(format!("/spaces/{}/actions", space_id()));
                }
                Err(err) => {
                    error!("Failed to submit quiz response: {:?}", err);
                    toast.error(err);
                }
            }
        });
    };

    let on_cancel = move |_| {
        nav.push(format!("/spaces/{}/actions", space_id()));
    };

    rsx! {
        if step() == QuizReadStep::Overview {
            QuestBriefing {
                node: QuestNodeView {
                    id: quiz_id().to_string(),
                    action_type: SpaceActionType::Quiz,
                    title: quiz.title.clone(),
                    base_points: 0,
                    projected_xp: 0,
                    status: QuestNodeStatus::Active,
                    depends_on: vec![],
                    chapter_id: String::new(),
                    started_at: None,
                    ended_at: None,
                    quiz_result: None,
                },
                on_begin: move |_| {
                    question_index.set(0);
                    step.set(QuizReadStep::Quiz);
                },
                on_cancel,
            }
        } else {
            FullActionLayover {
                "data-testid": "quiz-read-quiz",
                bottom_left: rsx! {
                    div { class: "text-sm text-foreground-muted",
                        "{i18n.remaining_submissions} {remaining_submissions}/{total_allowed}"
                    }
                },
                bottom_right: rsx! {
                    if !is_first_question && total_questions > 0 {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            onclick: move |_| {
                                let current = question_index();
                                if current > 0 {
                                    question_index.set(current - 1);
                                }
                            },
                            {i18n.btn_back}
                        }
                    }



                    if !is_last_question && total_questions > 0 {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            disabled: quiz_next_disabled,
                            onclick: move |_| {
                                let current = question_index();
                                if current + 1 < total_questions {
                                    question_index.set(current + 1);
                                }
                            },
                            {i18n.btn_next}
                        }
                    }

                    if is_last_question && can_respond && total_questions > 0 {
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            disabled: !can_submit || !all_answered(),
                            onclick: on_submit,
                            {i18n.btn_submit}
                        }
                    }
                },
                div { class: "w-full",
                    if !is_in_progress {
                        if now < quiz.started_at {
                            div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                                {i18n.quiz_not_started}
                            }
                        } else {
                            div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                                {i18n.quiz_ended}
                            }
                        }
                    }

                    if is_in_progress && !can_execute_action {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.no_permission}
                        }
                    }

                    if is_in_progress && can_execute_action && has_passed {
                        div { class: "p-3 text-sm rounded-lg bg-banner-success-bg text-banner-success-text",
                            {i18n.already_passed}
                        }
                    }

                    if is_in_progress && can_execute_action && !has_passed
                        && quiz.attempt_count >= total_allowed && can_respond
                    {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.no_remaining_attempts}
                        }
                    }

                    if quiz.questions.is_empty() {
                        div { class: "flex justify-center items-center py-10 text-foreground-muted",
                            {i18n.no_questions}
                        }
                    } else {
                        {
                            let idx = question_index().min(total_questions.saturating_sub(1));
                            let question = quiz.questions[idx].clone();
                            let answer = answers.read().get(idx).cloned();
                            let can_next = idx + 1 < total_questions;
                            rsx! {
                                div { key: "read-question-{idx}", class: "w-full",
                                    div { class: "flex justify-end items-center mb-5 font-normal text-[16px] text-text-primary",
                                        "{i18n.question_label}: {idx + 1}/{total_questions}"
                                    }
                                    div { class: "w-full [&_[data-question-title-wrap]]:mb-5 [&_[data-question-title-wrap]>div]:justify-center [&_[data-question-title]]:text-center [&_[data-question-title]]:text-[21px] [&_[data-question-desc]]:text-center",
                                        QuestionViewer {
                                            index: idx,
                                            total: total_questions,
                                            question: question.clone(),
                                            answer,
                                            disabled: !can_respond || !can_execute_action || !is_in_progress || !can_submit,
                                            on_change: move |next_answer: Answer| {
                                                let mut next = answers();
                                                if idx < next.len() {
                                                    next[idx] = next_answer.clone();
                                                }
                                                answers.set(next);


                                                if can_submit && can_next && should_auto_next(&question, &next_answer) {
                                                    question_index.set(idx + 1);
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
