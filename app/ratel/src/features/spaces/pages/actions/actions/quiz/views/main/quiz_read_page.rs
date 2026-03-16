use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::features::spaces::layout::use_space_layout_ui;
use crate::features::spaces::pages::actions::actions::poll::components::{
    has_answer_for_question, should_auto_next, QuestionViewer,
};
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::apps::apps::file::components::FileCard;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

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
    let mut hide_once = use_signal(|| false);
    let layout_ui = use_space_layout_ui();
    let sidebar_visible = layout_ui.sidebar_visible;
    let nav = navigator();

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
    let has_passed = quiz.passed.unwrap_or(false);
    let can_submit =
        can_respond && is_in_progress && !has_passed && quiz.attempt_count < quiz.retry_count;
    let remaining_submissions = quiz.retry_count.saturating_sub(quiz.attempt_count);
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
        let mut query = query;
        let mut toast = toast;
        let nav = nav.clone();
        spawn(async move {
            match respond_quiz(space_id(), quiz_id(), req).await {
                Ok(_) => {
                    let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                    query.invalidate(&keys);
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

    use_effect(move || {
        if hide_once() {
            return;
        }
        hide_once.set(true);
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(false);
    });

    use_drop(move || {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(true);
    });

    let on_cancel = move |_| {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(true);
        nav.push(format!("/spaces/{}/actions", space_id()));
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-4",
            if step() == QuizReadStep::Overview {
                div {
                    class: "mx-auto flex min-h-0 w-full flex-1 flex-col",
                    "data-testid": "quiz-read-overview",

                    div { class: "flex flex-row w-full justify-center items-center",
                        div { class: "flex flex-1 flex-col max-w-desktop gap-6 overflow-y-auto pb-6",
                            div { class: "text-[28px]/[34px] font-bold text-text-primary",
                                "{quiz.title}"
                            }

                            div { class: "flex items-center justify-end border-y border-card-border py-4",
                                div { class: "shrink-0 text-[14px] font-light text-text-primary",
                                    "{time_ago(quiz.created_at)}"
                                }
                            }

                            if !quiz.description.is_empty() {
                                div {
                                    class: "text-[15px]/[24px] tracking-[0.5px] text-[#D4D4D4] light:text-text-primary",
                                    dangerous_inner_html: quiz.description.clone(),
                                }
                            }

                            div { class: "flex flex-col gap-2",
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
                    }

                    div { class: "mt-auto -mx-5 -mb-5 max-tablet:-mx-3 max-tablet:-mb-3 max-mobile:-mx-2 max-mobile:-mb-2 flex items-center justify-between gap-3 border-t border-card-border bg-card-bg px-5 py-3",
                        div { class: "text-sm text-neutral-300 light:text-neutral-700",
                            "{i18n.remaining_submissions} {remaining_submissions}/{quiz.retry_count}"
                        }
                        div { class: "flex items-center gap-3",
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
                        }
                    }
                }
            } else {
                div {
                    class: "flex min-h-0 flex-1 flex-col w-full",
                    "data-testid": "quiz-read-quiz",

                    div { class: "flex flex-row w-full justify-center items-start min-h-0 flex-1",
                        div { class: "flex flex-1 flex-col gap-4 overflow-y-auto max-w-desktop pb-6",

                            if quiz.questions.is_empty() {
                                div { class: "flex items-center justify-center py-10 text-neutral-500 light:text-text-secondary",
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
                                                    disabled: !can_submit,
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
                    div { class: "-mx-5 -mb-5 max-tablet:-mx-3 max-tablet:-mb-3 max-mobile:-mx-2 max-mobile:-mb-2 flex items-center justify-between gap-3 border-t border-card-border bg-card-bg px-5 py-3",
                        div { class: "text-sm text-neutral-300 light:text-neutral-700",
                            "{i18n.remaining_submissions} {remaining_submissions}/{quiz.retry_count}"
                        }
                        div { class: "flex items-center gap-3",
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
                        }
                    }
                }
            }
        }
    }
}

fn time_ago(timestamp_millis: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_millis;

    if diff < 60 * 1000 {
        format!("{}s ago", diff / 1000)
    } else if diff < 3600 * 1000 {
        format!("{}m ago", diff / 1000 / 60)
    } else if diff < 86400 * 1000 {
        format!("{}h ago", diff / 1000 / 3600)
    } else if diff < 604800 * 1000 {
        format!("{}d ago", diff / 1000 / 86400)
    } else if diff < 31536000 * 1000 {
        format!("{}w ago", diff / 1000 / 604800)
    } else {
        format!("{}y ago", diff / 1000 / 31536000)
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
