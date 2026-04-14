use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::common::utils::time::time_ago;
use crate::features::spaces::pages::actions::actions::poll::components::{
    has_answer_for_question, should_auto_next, QuestionViewer,
};
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::pages::apps::apps::file::components::FileCard;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;

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
    let mut ctx = Context::init(space_id, quiz_id)?;
    let quiz = ctx.quiz.read().clone();
    let mut space_ctx = use_space_context();
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
    let can_execute_action = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        quiz.space_action.prerequisite,
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
                        ctx.quiz.restart();
                    ctx.answer.restart();
                    space_ctx.ranking.restart();
                    space_ctx.my_score.restart();
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
            FullActionLayover {
                "data-testid": "quiz-read-overview",
                content_class: "gap-6".to_string(),
                bottom_left: rsx! {
                    div { class: "text-sm text-foreground-muted",
                        "{i18n.remaining_submissions} {remaining_submissions}/{total_allowed}"
                    }
                },
                bottom_right: rsx! {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        onclick: on_cancel,
                        {i18n.btn_cancel}
                    }
                    Button {
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        disabled: quiz.questions.is_empty(),
                        "data-testid": "quiz-read-next",
                        onclick: move |_| {
                            question_index.set(0);
                            step.set(QuizReadStep::Quiz);
                        },
                        {i18n.btn_next}
                    }
                },
                div { class: "w-full",
                    div { class: "text-[28px]/[34px] font-bold text-text-primary", "{quiz.title}" }

                    div { class: "flex items-center justify-end border-y border-card-border py-4",
                        div { class: "shrink-0 text-[14px] font-light text-text-primary",
                            "{time_ago(quiz.created_at)}"
                        }
                    }

                    if !quiz.description.is_empty() {
                        div {
                            class: "text-[15px]/[24px] tracking-[0.5px] text-foreground-muted",
                            dangerous_inner_html: quiz.description.clone(),
                        }
                    }

                    if !quiz.files.is_empty() {
                        div { class: "grid grid-cols-4 gap-2.5 max-desktop:grid-cols-3 max-tablet:grid-cols-2 max-mobile:grid-cols-1",
                            for file in quiz.files.iter() {
                                FileCard {
                                    key: "{file.id}",
                                    file: file.clone(),
                                    editable: false,
                                    on_delete: None,
                                }
                            }
                        }
                    }
                }
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
                            div { class: "rounded-lg bg-banner-bg p-3 text-sm text-banner-text",
                                {i18n.quiz_not_started}
                            }
                        } else {
                            div { class: "rounded-lg bg-banner-bg p-3 text-sm text-banner-text",
                                {i18n.quiz_ended}
                            }
                        }
                    }

                    if is_in_progress && !can_execute_action {
                        div { class: "rounded-lg bg-banner-bg p-3 text-sm text-banner-text",
                            {i18n.no_permission}
                        }
                    }

                    if is_in_progress && can_execute_action && has_passed {
                        div { class: "rounded-lg bg-banner-success-bg p-3 text-sm text-banner-success-text",
                            {i18n.already_passed}
                        }
                    }

                    if is_in_progress && can_execute_action && !has_passed
                        && quiz.attempt_count >= total_allowed && can_respond
                    {
                        div { class: "rounded-lg bg-banner-bg p-3 text-sm text-banner-text",
                            {i18n.no_remaining_attempts}
                        }
                    }

                    if quiz.questions.is_empty() {
                        div { class: "flex items-center justify-center py-10 text-foreground-muted",
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
                                    div { class: "mb-5 flex items-center justify-end text-[16px] font-normal text-text-primary",
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

                                                if can_submit
                                                    && can_next
                                                    && should_auto_next(&question, &next_answer)
                                                {
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
