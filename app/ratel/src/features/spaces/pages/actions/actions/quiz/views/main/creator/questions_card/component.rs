use crate::features::spaces::pages::actions::actions::poll::types::ChoiceQuestion;
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::actions::quiz::views::main::creator::QuizCreatorTranslate;

fn qtype_str(q: &Question) -> &'static str {
    match q {
        Question::MultipleChoice(_) | Question::Checkbox(_) => "multi",
        _ => "single",
    }
}

fn options_of(q: &Question) -> Vec<String> {
    match q {
        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => cq.options.clone(),
        Question::Checkbox(c) => c.options.clone(),
        Question::Dropdown(d) => d.options.clone(),
        _ => Vec::new(),
    }
}

fn title_of(q: &Question) -> String {
    q.title().to_string()
}

fn set_title(q: &mut Question, value: String) {
    match q {
        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => cq.title = value,
        Question::Subjective(s) | Question::ShortAnswer(s) => s.title = value,
        Question::Checkbox(c) => c.title = value,
        Question::Dropdown(d) => d.title = value,
        Question::LinearScale(l) => l.title = value,
    }
}

fn convert_to_qtype(existing: &Question, target: &str) -> Question {
    let title = title_of(existing);
    let options = options_of(existing);
    let opts = if options.is_empty() {
        vec![String::new(), String::new()]
    } else {
        options
    };
    match target {
        "multi" => Question::MultipleChoice(ChoiceQuestion {
            title,
            options: opts,
            ..Default::default()
        }),
        _ => Question::SingleChoice(ChoiceQuestion {
            title,
            options: opts,
            ..Default::default()
        }),
    }
}

#[component]
pub fn QuestionsCard() -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let ctx = use_space_quiz_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let mut questions = ctx.questions;
    let mut answers = ctx.answers;
    let pass_score = ctx.pass_score;
    let retry_count = ctx.retry_count;

    let save = move || {
        spawn(async move {
            let req = UpdateQuizRequest {
                questions: Some(questions()),
                answers: Some(answers()),
                pass_score: Some(pass_score()),
                retry_count: Some(retry_count()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to save questions: {:?}", err);
                toast.error(err);
            }
        });
    };

    let qs = questions.read().clone();

    rsx! {
        section { class: "pager__page", "data-page": "1",
            article { class: "page-card", "data-testid": "page-card-questions",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_2}" }
                        div {
                            h1 { class: "page-card__title", "{tr.card_questions_title}" }
                            div { class: "page-card__subtitle", "{tr.card_questions_subtitle}" }
                        }
                    }
                }

                section { class: "section", "data-testid": "section-questions",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_questions_label}" }
                        span { class: "section__hint", "{tr.section_questions_hint}" }
                    }

                    for (idx , question) in qs.iter().enumerate() {
                        QuestionBlock {
                            key: "q-{idx}",
                            idx,
                            question: question.clone(),
                            answer: answers.read().get(idx).cloned().unwrap_or_default(),
                            on_title_change: move |(i, title): (usize, String)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    set_title(q, title);
                                }
                            },
                            on_type_change: move |(i, target): (usize, String)| {
                                let new_q = {
                                    let qs = questions.read();
                                    if let Some(existing) = qs.get(i) {
                                        convert_to_qtype(existing, &target)
                                    } else {
                                        return;
                                    }
                                };
                                let new_a = QuizCorrectAnswer::for_question(&new_q);
                                {
                                    let mut qs = questions.write();
                                    if let Some(q) = qs.get_mut(i) {
                                        *q = new_q;
                                    }
                                }
                                {
                                    let mut ans = answers.write();
                                    if let Some(a) = ans.get_mut(i) {
                                        *a = new_a;
                                    }
                                }
                                save();
                            },
                            on_option_change: move |(i, opt_idx, text): (usize, usize, String)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    match q {
                                        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => {
                                            if let Some(o) = cq.options.get_mut(opt_idx) {
                                                *o = text;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            },
                            on_correct_toggle: move |(i, opt_idx): (usize, usize)| {
                                let qtype = {
                                    let qs = questions.read();
                                    qs.get(i).map(qtype_str).unwrap_or("single").to_string()
                                };
                                let mut ans = answers.write();
                                if let Some(a) = ans.get_mut(i) {
                                    let opt_i32 = opt_idx as i32;
                                    match qtype.as_str() {
                                        "multi" => {
                                            if let QuizCorrectAnswer::Multiple { answers: arr } = a {
                                                if let Some(pos) = arr.iter().position(|x| *x == opt_i32) {
                                                    arr.remove(pos);
                                                } else {
                                                    arr.push(opt_i32);
                                                    arr.sort();
                                                }
                                            } else {
                                                *a = QuizCorrectAnswer::Multiple {
                                                    answers: vec![opt_i32],
                                                };
                                            }
                                        }
                                        _ => {
                                            *a = QuizCorrectAnswer::Single {
                                                answer: Some(opt_i32),
                                            };
                                        }
                                    }
                                }
                                drop(ans);
                                save();
                            },
                            on_remove: move |i: usize| {
                                {
                                    let mut qs = questions.write();
                                    if i < qs.len() {
                                        qs.remove(i);
                                    }
                                }
                                {
                                    let mut ans = answers.write();
                                    if i < ans.len() {
                                        ans.remove(i);
                                    }
                                }
                                save();
                            },
                            on_blur_save: move |_| save(),
                        }
                    }

                    button {
                        class: "add-btn",
                        r#type: "button",
                        "data-testid": "quiz-question-add",
                        onclick: move |_| {
                            let new_q = Question::SingleChoice(ChoiceQuestion {
                                title: String::new(),
                                options: vec![String::new(), String::new()],
                                ..Default::default()
                            });
                            let new_a = QuizCorrectAnswer::for_question(&new_q);
                            questions.write().push(new_q);
                            answers.write().push(new_a);
                            save();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "12",
                                y1: "5",
                                x2: "12",
                                y2: "19",
                            }
                            line {
                                x1: "5",
                                y1: "12",
                                x2: "19",
                                y2: "12",
                            }
                        }
                        "{tr.add_question}"
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionBlock(
    idx: usize,
    question: Question,
    answer: QuizCorrectAnswer,
    on_title_change: EventHandler<(usize, String)>,
    on_type_change: EventHandler<(usize, String)>,
    on_option_change: EventHandler<(usize, usize, String)>,
    on_correct_toggle: EventHandler<(usize, usize)>,
    on_remove: EventHandler<usize>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let qtype = qtype_str(&question);
    let title = title_of(&question);
    let q_num = idx + 1;

    let is_single = qtype == "single";
    let is_multi = qtype == "multi";

    rsx! {
        div {
            class: "q-block",
            "data-qtype": qtype,
            "data-testid": "quiz-question-{idx}",
            div { class: "q-block__head",
                span { class: "q-block__num", "Q{q_num}" }
                div {
                    class: "segmented segmented--sm",
                    role: "tablist",
                    "data-testid": "quiz-question-{idx}-type",
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_single,
                        onclick: move |_| on_type_change.call((idx, "single".into())),
                        "{tr.qtype_single}"
                    }
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_multi,
                        onclick: move |_| on_type_change.call((idx, "multi".into())),
                        "{tr.qtype_multi}"
                    }
                }
                div { class: "q-block__head-spacer" }
                button {
                    class: "icon-btn",
                    r#type: "button",
                    aria_label: "{tr.remove_question}",
                    onclick: move |_| on_remove.call(idx),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
            }

            input {
                class: "input",
                value: "{title}",
                oninput: move |e| on_title_change.call((idx, e.value())),
                onblur: move |_| on_blur_save.call(()),
            }

            ChoiceOptions {
                idx,
                question: question.clone(),
                answer: answer.clone(),
                is_check: is_multi,
                on_option_change,
                on_correct_toggle,
                on_blur_save,
            }
        }
    }
}

#[component]
fn ChoiceOptions(
    idx: usize,
    question: Question,
    answer: QuizCorrectAnswer,
    is_check: bool,
    on_option_change: EventHandler<(usize, usize, String)>,
    on_correct_toggle: EventHandler<(usize, usize)>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let options = options_of(&question);

    let body_class = if is_check {
        "q-body q-body--multi"
    } else {
        "q-body q-body--single"
    };
    let opt_class = if is_check { "q-opt q-opt--check" } else { "q-opt" };

    rsx! {
        div { class: "{body_class}",
            for (i , opt) in options.iter().enumerate() {
                {
                    let checked = match &answer {
                        QuizCorrectAnswer::Single { answer: Some(v) } => *v == i as i32,
                        QuizCorrectAnswer::Multiple { answers } => answers.contains(&(i as i32)),
                        _ => false,
                    };
                    rsx! {
                        div {
                            key: "opt-{i}",
                            class: "{opt_class}",
                            "aria-checked": checked,
                            "data-testid": "quiz-question-{idx}-opt-{i}",
                            span {
                                class: "q-opt__radio",
                                onclick: move |_| on_correct_toggle.call((idx, i)),
                            }
                            input {
                                class: "input",
                                value: "{opt}",
                                oninput: move |e| on_option_change.call((idx, i, e.value())),
                                onblur: move |_| on_blur_save.call(()),
                            }
                            span { class: "q-opt__tag", "{tr.correct_tag}" }
                        }
                    }
                }
            }
        }
    }
}
