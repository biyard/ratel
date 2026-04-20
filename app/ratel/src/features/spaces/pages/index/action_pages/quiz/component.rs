use crate::features::spaces::pages::actions::actions::poll::{Answer, ChoiceQuestion, Question};
use crate::features::spaces::pages::actions::actions::quiz::controllers::{
    get_quiz, respond_quiz, RespondQuizRequest,
};
use crate::features::spaces::pages::actions::actions::quiz::{QuizCorrectAnswer, QuizResponse};
use crate::features::spaces::pages::index::action_pages::quiz::*;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;

/// Generic overlay for prerequisite/action pages (poll + quiz + discussion).
#[derive(Clone, PartialEq)]
pub enum ActiveActionOverlay {
    Poll(SpacePartition, SpacePollEntityType),
    Quiz(SpacePartition, SpaceQuizEntityType),
    Discussion(SpacePartition, SpacePostEntityType),
}

/// Context signal wrapping the overlay state.
#[derive(Clone, Copy)]
pub struct ActiveActionOverlaySignal(pub Signal<Option<ActiveActionOverlay>>);

/// Context signal set when an action card is completed (quiz passed, all followed, etc.).
/// Holds the action_id so the dashboard can animate the card into the archive.
#[derive(Clone, Copy)]
pub struct CompletedActionCard(pub Signal<Option<String>>);

const LETTERS: &[&str] = &["A", "B", "C", "D", "E", "F", "G", "H"];
const RING_CIRCUMFERENCE: f64 = 2.0 * std::f64::consts::PI * 20.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum QuizStep {
    Overview,
    Quiz,
}

#[component]
pub fn QuizArenaPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizArenaTranslate = use_translate();
    let nav = use_navigator();
    let mut toast = use_toast();
    let mut space_ctx = use_space_context();
    let role = use_space_role()();
    let space = use_space()();

    let mut quiz_loader = use_loader(move || get_quiz(space_id(), quiz_id()))?;
    let quiz = quiz_loader();
    let questions = quiz.questions.clone();

    let total_questions = questions.len();
    let total_allowed = quiz.retry_count.saturating_add(1);
    let remaining = total_allowed.saturating_sub(quiz.attempt_count);
    let has_passed = quiz.passed.unwrap_or(false);
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let is_in_progress = now >= quiz.started_at && now <= quiz.ended_at;
    // Candidates must be allowed through here so they can take a prerequisite
    // quiz before the space starts. The actual role+prerequisite+status gate
    // below (`can_execute`) still blocks non-prerequisite quizzes / Viewers.
    let can_respond = matches!(
        role,
        SpaceUserRole::Creator | SpaceUserRole::Participant | SpaceUserRole::Candidate
    );
    let can_execute = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        quiz.space_action.prerequisite,
        space.status,
        space.join_anytime,
    );
    let can_submit = can_respond
        && can_execute
        && is_in_progress
        && !has_passed
        && quiz.attempt_count < total_allowed;
    let pass_pct = if total_questions > 0 {
        (quiz.pass_score as f64 / total_questions as f64 * 100.0) as i64
    } else {
        0
    };

    let mut step = use_signal(|| QuizStep::Overview);
    let mut question_index = use_signal(|| 0usize);

    let initial_answers: Vec<Answer> = quiz
        .my_response
        .clone()
        .unwrap_or_else(|| questions.iter().map(default_answer).collect());
    let mut answers = use_signal(|| initial_answers);

    let questions_for_memo = questions.clone();
    let answered_count = use_memo(move || {
        let ans = answers.read();
        questions_for_memo
            .iter()
            .enumerate()
            .filter(|(i, q)| has_answer(q, ans.get(*i)))
            .count()
    });

    let all_answered = use_memo(move || {
        if total_questions == 0 {
            return false;
        }
        answered_count() == total_questions
    });

    let ring_offset = use_memo(move || {
        if total_questions == 0 {
            return RING_CIRCUMFERENCE;
        }
        RING_CIRCUMFERENCE - (answered_count() as f64 / total_questions as f64) * RING_CIRCUMFERENCE
    });

    let mut overlay: ActiveActionOverlaySignal = use_context();
    let mut completed: CompletedActionCard = use_context();

    let mut close_overlay = move || {
        overlay.0.set(None);
    };

    let on_back = move |_| {
        if step() == QuizStep::Quiz && question_index() == 0 {
            step.set(QuizStep::Overview);
        } else {
            close_overlay();
        }
    };

    let on_submit = move |_| async move {
        let req = RespondQuizRequest { answers: answers() };
        match respond_quiz(space_id(), quiz_id(), req).await {
            Ok(_) => {
                quiz_loader.restart();
                space_ctx.ranking.restart();
                space_ctx.my_score.restart();
                // Restart actions list so dashboard refreshes after animation
                space_ctx.actions.restart();
                toast.info(tr.submit_success);
                // Signal the dashboard to animate this card into archive
                completed.0.set(Some(quiz_id().to_string()));
                close_overlay();
            }
            Err(err) => {
                tracing::error!("Failed to submit quiz: {:?}", err);
                toast.error(err);
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "quiz-arena",

            // ── Top bar ──────────────────────────────
            div { class: "quiz-topbar",
                div { class: "quiz-topbar__left",
                    button {
                        class: "quiz-topbar__back",
                        "data-testid": "quiz-arena-back",
                        onclick: on_back,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M19 12H5" }
                            path { d: "m12 19-7-7 7-7" }
                        }
                    }
                    span { class: "quiz-topbar__title", "{quiz.title}" }
                    span { class: "quiz-topbar__badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                            line {
                                x1: "12",
                                y1: "17",
                                x2: "12.01",
                                y2: "17",
                            }
                        }
                        "{tr.quiz_label}"
                    }
                }
                div { class: "quiz-topbar__right",
                    if step() == QuizStep::Quiz {
                        div { class: "progress-ring",
                            svg {
                                class: "progress-ring__svg",
                                view_box: "0 0 48 48",
                                circle {
                                    class: "progress-ring__bg",
                                    cx: "24",
                                    cy: "24",
                                    r: "20",
                                }
                                circle {
                                    class: "progress-ring__fill",
                                    cx: "24",
                                    cy: "24",
                                    r: "20",
                                    stroke_dasharray: "{RING_CIRCUMFERENCE}",
                                    stroke_dashoffset: "{ring_offset()}",
                                }
                            }
                            div { class: "progress-ring__label", "{answered_count()}/{total_questions}" }
                        }
                    }
                    div { class: "attempts-chip",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                            path { d: "M3 3v5h5" }
                            path { d: "M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" }
                            path { d: "M16 16h5v5" }
                        }
                        span { "{remaining} / {total_allowed} {tr.attempts_left}" }
                    }
                }
            }

            // ── Banners ──────────────────────────────
            if !is_in_progress && now > quiz.ended_at {
                div {
                    class: "quiz-banner quiz-banner--warning",
                    "{tr.quiz_ended}"
                }
            }
            if !is_in_progress && now < quiz.started_at {
                div {
                    class: "quiz-banner quiz-banner--info",
                    "{tr.quiz_not_started}"
                }
            }
            // Viewer role, space is Processing/Finished, or prerequisite
            // missing → show the permission banner (matches poll's arena
            // behavior). `can_execute` already folds in space status +
            // prerequisite + join_anytime.
            if is_in_progress && (!can_respond || !can_execute) {
                div {
                    class: "quiz-banner quiz-banner--warning",
                    "{tr.no_permission}"
                }
            }
            if is_in_progress && has_passed {
                div {
                    class: "quiz-banner quiz-banner--success",
                    "{tr.already_passed}"
                }
            }
            if is_in_progress && !has_passed && quiz.attempt_count >= total_allowed && can_respond {
                div {
                    class: "quiz-banner quiz-banner--warning",
                    "{tr.no_remaining_attempts}"
                }
            }

            // ── SCREEN 1: Overview ───────────────────
            if step() == QuizStep::Overview {
                div {
                    class: "quiz-overview",
                    "data-testid": "quiz-arena-overview",

                    div { class: "overview-ring",
                        svg {
                            class: "overview-ring__svg",
                            view_box: "0 0 140 140",
                            circle {
                                class: "overview-ring__bg",
                                cx: "70",
                                cy: "70",
                                r: "60",
                            }
                            circle {
                                class: "overview-ring__fill",
                                cx: "70",
                                cy: "70",
                                r: "60",
                                stroke_dasharray: "376.99",
                                stroke_dashoffset: "0",
                            }
                        }
                        div { class: "overview-ring__center",
                            span { class: "overview-ring__number", "{total_questions}" }
                            span { class: "overview-ring__label", "{tr.questions_label}" }
                        }
                    }

                    div { class: "overview-card",
                        div { class: "overview-card__title", "{quiz.title}" }
                        if !quiz.description.is_empty() {
                            div {
                                class: "overview-card__desc",
                                dangerous_inner_html: "{quiz.description}",
                            }
                        }

                        div { class: "overview-stats",
                            div { class: "overview-stat",
                                span { class: "overview-stat__value", "{total_questions}" }
                                span { class: "overview-stat__label", "{tr.questions_label}" }
                            }
                            div { class: "overview-stat",
                                span { class: "overview-stat__value", "{pass_pct}%" }
                                span { class: "overview-stat__label", "{tr.pass_score_label}" }
                            }
                            div { class: "overview-stat",
                                span { class: "overview-stat__value", "{quiz.space_action.credits}" }
                                span { class: "overview-stat__label", "{tr.credits_label}" }
                            }
                        }

                        if !quiz.files.is_empty() {
                            div { class: "overview-files",
                                for file in quiz.files.iter() {
                                    a {
                                        class: "overview-file",
                                        key: "{file.id}",
                                        href: file.url.clone().unwrap_or_default(),
                                        target: "_blank",
                                        download: "{file.name}",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                            polyline { points: "7 10 12 15 17 10" }
                                            line {
                                                x1: "12",
                                                y1: "15",
                                                x2: "12",
                                                y2: "3",
                                            }
                                        }
                                        "{file.name}"
                                    }
                                }
                            }
                        }

                        button {
                            class: "quiz-begin-btn",
                            "data-testid": "quiz-arena-begin",
                            disabled: total_questions == 0,
                            onclick: move |_| {
                                question_index.set(0);
                                step.set(QuizStep::Quiz);
                            },
                            if can_submit {
                                "{tr.begin_quiz}"
                            } else {
                                "{tr.review_quiz}"
                            }
                        }
                    }
                }
            }

            // ── SCREEN 2: Question solving ───────────
            if step() == QuizStep::Quiz {
                div {
                    class: "quiz-question-area",
                    "data-testid": "quiz-arena-questions",
                    {
                        let idx = question_index().min(total_questions.saturating_sub(1));
                        let question = questions.get(idx).cloned();
                        rsx! {
                            if let Some(question) = question {
                                QuestionCardView {
                                    key: "{idx}",
                                    index: idx,
                                    total: total_questions,
                                    question,
                                    answer: answers.read().get(idx).cloned(),
                                    correct_answer: quiz
                                                                            .correct_answers
                                                                            .as_ref()
                                                                            .and_then(|a| a.get(idx))
                                                                            .cloned(),
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

                // ── Bottom bar ───────────────────────
                div { class: "quiz-bottom",
                    div { class: "quiz-bottom__left",
                        div { class: "question-dots",
                            for i in 0..total_questions {
                                div {
                                    key: "dot-{i}",
                                    class: {
                                        let base = "question-dot";
                                        if i == question_index() {
                                            format!("{base} question-dot--current")
                                        } else if has_answer(&questions[i], answers.read().get(i)) {
                                            format!("{base} question-dot--answered")
                                        } else {
                                            base.to_string()
                                        }
                                    },
                                }
                            }
                        }
                    }
                    div { class: "quiz-bottom__right",
                        if question_index() > 0 {
                            button {
                                class: "quiz-nav-btn",
                                "data-testid": "quiz-arena-prev",
                                onclick: move |_| {
                                    let cur = question_index();
                                    if cur > 0 {
                                        question_index.set(cur - 1);
                                    }
                                },
                                "{tr.btn_back}"
                            }
                        }

                        if question_index() + 1 < total_questions {
                            button {
                                class: "quiz-nav-btn quiz-nav-btn--primary",
                                "data-testid": "quiz-arena-next",
                                onclick: move |_| {
                                    let cur = question_index();
                                    if cur + 1 < total_questions {
                                        question_index.set(cur + 1);
                                    }
                                },
                                "{tr.btn_next}"
                            }
                        }

                        if question_index() + 1 >= total_questions {
                            button {
                                class: "quiz-nav-btn quiz-nav-btn--submit",
                                "data-testid": "quiz-arena-submit",
                                disabled: !can_submit || !all_answered(),
                                onclick: on_submit,
                                "{tr.btn_submit}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Question card ────────────────────────────────────
#[component]
fn QuestionCardView(
    index: usize,
    total: usize,
    question: Question,
    answer: Option<Answer>,
    #[props(default)] correct_answer: Option<QuizCorrectAnswer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: QuizArenaTranslate = use_translate();

    let (title, description, options, is_required, is_multi) = match &question {
        Question::SingleChoice(q) => (
            q.title.clone(),
            q.description.clone(),
            q.options.clone(),
            q.is_required,
            false,
        ),
        Question::MultipleChoice(q) => (
            q.title.clone(),
            q.description.clone(),
            q.options.clone(),
            q.is_required,
            true,
        ),
        _ => return rsx! {},
    };

    let use_single_col = options.len() > 4;

    rsx! {
        div { class: "question-card",
            div { class: "question-header",
                span { class: "question-number", "{tr.question_of} {index + 1} {tr.of_label} {total}" }
                if is_required.unwrap_or(false) {
                    span { class: "question-required", "{tr.required_label}" }
                }
            }

            div { class: "question-title", "{title}" }
            {
                let has_custom_desc = description.as_ref().is_some_and(|d| !d.is_empty());
                let hint = if is_multi {
                    tr.multiple_choice_hint
                } else {
                    tr.single_choice_hint
                };
                rsx! {
                    if has_custom_desc {
                        div { class: "question-desc", "{description.as_ref().unwrap()}" }
                    } else {
                        div { class: "question-desc", "{hint}" }
                    }
                }
            }

            div { class: if use_single_col { "option-grid option-grid--single" } else { "option-grid" },
                for (opt_idx, option_text) in options.iter().enumerate() {
                    {
                        let is_selected = check_selected(&answer, opt_idx as i32, is_multi);
                        let is_correct =
                            check_correct(correct_answer.as_ref(), opt_idx as i32, is_multi);
                        let on_change = on_change.clone();
                        let answer_clone = answer.clone();
                        rsx! {
                            button {
                                key: "opt-{index}-{opt_idx}",
                                class: "option-tile",
                                "aria-selected": is_selected,
                                "data-correct": is_correct,
                                "data-user-wrong": is_selected && !is_correct && correct_answer.is_some(),
                                disabled,
                                onclick: move |_| {
                                    let next = if is_multi {
                                        toggle_multi(&answer_clone, opt_idx as i32)
                                    } else {
                                        toggle_single(&answer_clone, opt_idx as i32)
                                    };
                                    on_change.call(next);
                                },
                                div { class: "option-tile__accent" }
                                div { class: "option-tile__content",
                                    if is_multi {
                                        div { class: "option-tile__checkbox",
                                            svg {
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "3",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                polyline { points: "20 6 9 17 4 12" }
                                            }
                                        }
                                    } else {
                                        span { class: "option-tile__letter", "{LETTERS.get(opt_idx).unwrap_or(&\"\")}" }
                                    }
                                    span { class: "option-tile__text", "{option_text}" }
                                }
                                div { class: "option-tile__check",
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "3",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        polyline { points: "20 6 9 17 4 12" }
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

// ── Helpers ──────────────────────────────────────────

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
        _ => Answer::SingleChoice {
            answer: None,
            other: None,
        },
    }
}

fn has_answer(question: &Question, answer: Option<&Answer>) -> bool {
    match (question, answer) {
        (
            Question::SingleChoice(_),
            Some(Answer::SingleChoice {
                answer: Some(_), ..
            }),
        ) => true,
        (
            Question::MultipleChoice(_),
            Some(Answer::MultipleChoice {
                answer: Some(selected),
                ..
            }),
        ) => !selected.is_empty(),
        _ => false,
    }
}

fn check_selected(answer: &Option<Answer>, opt_idx: i32, is_multi: bool) -> bool {
    match answer {
        Some(Answer::SingleChoice {
            answer: Some(sel), ..
        }) if !is_multi => *sel == opt_idx,
        Some(Answer::MultipleChoice {
            answer: Some(selected),
            ..
        }) if is_multi => selected.contains(&opt_idx),
        _ => false,
    }
}

fn check_correct(
    correct: Option<&QuizCorrectAnswer>,
    opt_idx: i32,
    is_multi: bool,
) -> bool {
    match correct {
        Some(QuizCorrectAnswer::Single { answer: Some(v) }) if !is_multi => *v == opt_idx,
        Some(QuizCorrectAnswer::Multiple { answers }) if is_multi => answers.contains(&opt_idx),
        _ => false,
    }
}

fn toggle_single(answer: &Option<Answer>, opt_idx: i32) -> Answer {
    let current = match answer {
        Some(Answer::SingleChoice {
            answer: Some(sel), ..
        }) => Some(*sel),
        _ => None,
    };
    let next = if current == Some(opt_idx) {
        None
    } else {
        Some(opt_idx)
    };
    Answer::SingleChoice {
        answer: next,
        other: None,
    }
}

fn toggle_multi(answer: &Option<Answer>, opt_idx: i32) -> Answer {
    let mut selected = match answer {
        Some(Answer::MultipleChoice {
            answer: Some(sel), ..
        }) => sel.clone(),
        _ => vec![],
    };
    if let Some(pos) = selected.iter().position(|&x| x == opt_idx) {
        selected.remove(pos);
    } else {
        selected.push(opt_idx);
    }
    Answer::MultipleChoice {
        answer: Some(selected),
        other: None,
    }
}
