//! cf-sunji card — opens below the cross-filter when the picker has
//! something to commit.
//!
//! Source-specific layout:
//! - Poll: questions + options as nested checkbox rows. Question
//!   index drives the chip's `question_id`; option index drives
//!   `option_id`.
//! - Quiz: same as Poll plus a "정답" badge on options that match the
//!   loaded `correct_answers`.
//! - Discussion: keyword input only — no predefined options.
//! - Follow: flat target list as multi-select checkboxes — no item
//!   layer above it.

use crate::features::spaces::pages::actions::actions::poll::Question;
use crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer;
use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn CfSunjiCard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;

    let picking_type = ctrl.picking_type.read().clone();
    let picked_item_id = ctrl.picked_item_id.read().clone();
    let picked_sunji = ctrl.picked_sunji.read().clone();
    let keyword_input = ctrl.keyword_input.read().clone();

    let (item_id, src) = match (picked_item_id.clone(), picking_type) {
        (Some(id), Some(s)) => (id, s),
        _ => {
            return rsx! {
                section { class: "cf-sunji", id: "cf-sunji", hidden: true }
            };
        }
    };

    let title_text = match src {
        AnalyzeFilterSource::Follow => tr.create_sunji_follow_title.to_string(),
        _ => {
            let stored = ctrl.picked_item_title.read().clone();
            if stored.is_empty() {
                tr.create_sunji_default_title.to_string()
            } else {
                stored
            }
        }
    };

    let badge_text = src.badge();
    let src_attr = src.as_str();

    let is_discussion = matches!(src, AnalyzeFilterSource::Discussion);
    let is_follow = matches!(src, AnalyzeFilterSource::Follow);

    let has_kw = is_discussion
        && !keyword_input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .is_empty();
    let has_sunji = !picked_sunji.is_empty();
    let confirm_disabled = !(has_sunji || has_kw);

    rsx! {
        section { class: "cf-sunji", id: "cf-sunji",

            header { class: "cf-sunji__head",
                span {
                    class: "cf-sunji__type-chip",
                    id: "cf-sunji-type",
                    "data-source": "{src_attr}",
                    "{badge_text}"
                }
                span { class: "cf-sunji__title", id: "cf-sunji-title", "{title_text}" }
                button {
                    r#type: "button",
                    class: "cross-filter__pick-cancel",
                    id: "cf-sunji-back",
                    "data-testid": "cf-sunji-back",
                    onclick: move |_| {
                        if is_follow {
                            ctrl.back_to_action();
                        } else {
                            ctrl.clear_item();
                        }
                    },
                    "{tr.create_sunji_back}"
                }
            }

            div { class: "cf-sunji__list", id: "cf-sunji-list",

                match src {
                    AnalyzeFilterSource::Discussion => rsx! {
                        KeywordBlock { space_id }
                    },
                    AnalyzeFilterSource::Follow => rsx! {
                        FollowTargetsBlock { space_id }
                    },
                    AnalyzeFilterSource::Poll => rsx! {
                        PollQuestionsBlock { space_id, item_id }
                    },
                    AnalyzeFilterSource::Quiz => rsx! {
                        QuizQuestionsBlock { space_id, item_id }
                    },
                }
            }

            div { class: "cf-sunji__foot",
                button {
                    r#type: "button",
                    class: "btn btn--primary",
                    id: "cf-sunji-confirm",
                    "data-testid": "cf-sunji-confirm",
                    disabled: confirm_disabled,
                    onclick: move |_| ctrl.confirm_sunji(),
                    "{tr.create_sunji_confirm}"
                }
            }
        }
    }
}

#[component]
fn PollQuestionsBlock(space_id: ReadSignal<SpacePartition>, item_id: String) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let ctrl = use_analyze_create(space_id)?;
    let detail = ctrl.selected_poll.read().clone();
    let questions = detail
        .as_ref()
        .map(|p| p.questions.clone())
        .unwrap_or_default();

    // `Loader<T>` keeps the previous value visible while the next fetch
    // is in flight, so a freshly-picked poll briefly shows the prior
    // (often-empty) data. Suppress the empty state until the loader
    // settles to avoid the false "no questions" flash.
    if questions.is_empty() {
        if ctrl.selected_poll.loading() {
            return rsx! {
                div {
                    class: "cross-filter__chips-empty",
                    style: "padding: 20px 4px;",
                    "{tr.create_sunji_loading}"
                }
            };
        }
        return rsx! {
            div {
                class: "cross-filter__chips-empty",
                style: "padding: 20px 4px;",
                "{tr.create_sunji_empty}"
            }
        };
    }

    rsx! {
        for (q_idx, question) in questions.iter().enumerate() {
            QuestionBlock {
                key: "q-{q_idx}",
                space_id,
                item_id: item_id.clone(),
                q_idx,
                question: question.clone(),
                correct_indices: Vec::new(),
            }
        }
    }
}

#[component]
fn QuizQuestionsBlock(space_id: ReadSignal<SpacePartition>, item_id: String) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let ctrl = use_analyze_create(space_id)?;
    let detail = ctrl.selected_quiz.read().clone();
    let questions = detail
        .as_ref()
        .map(|q| q.questions.clone())
        .unwrap_or_default();
    let correct_answers = detail
        .as_ref()
        .and_then(|q| q.correct_answers.clone())
        .unwrap_or_default();

    if questions.is_empty() {
        if ctrl.selected_quiz.loading() {
            return rsx! {
                div {
                    class: "cross-filter__chips-empty",
                    style: "padding: 20px 4px;",
                    "{tr.create_sunji_loading}"
                }
            };
        }
        return rsx! {
            div {
                class: "cross-filter__chips-empty",
                style: "padding: 20px 4px;",
                "{tr.create_sunji_empty}"
            }
        };
    }

    rsx! {
        for (q_idx, question) in questions.iter().enumerate() {
            QuestionBlock {
                key: "q-{q_idx}",
                space_id,
                item_id: item_id.clone(),
                q_idx,
                question: question.clone(),
                correct_indices: correct_indices_at(&correct_answers, q_idx),
            }
        }
    }
}

#[component]
fn QuestionBlock(
    space_id: ReadSignal<SpacePartition>,
    item_id: String,
    q_idx: usize,
    question: Question,
    correct_indices: Vec<u32>,
) -> Element {
    let _ = item_id;
    let q_title = question.title().to_string();
    let options = options_of(&question);

    if options.is_empty() {
        // Skip questions that don't have a finite option list (e.g.
        // short-answer / subjective). They can't drive a chip.
        return rsx! {};
    }

    rsx! {
        div { class: "cf-question",
            div { class: "cf-question__title", "{q_title}" }
            div { class: "cf-question__options",
                for (o_idx, label) in options.iter().enumerate() {
                    QuestionOption {
                        key: "{q_idx}-{o_idx}",
                        space_id,
                        q_idx,
                        o_idx,
                        label: label.clone(),
                        correct: correct_indices.contains(&(o_idx as u32)),
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionOption(
    space_id: ReadSignal<SpacePartition>,
    q_idx: usize,
    o_idx: usize,
    label: String,
    correct: bool,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;

    let token = format!("{}:{}", q_idx, o_idx);
    let token_for_toggle = token.clone();
    let checked = ctrl.picked_sunji.read().contains(&token);

    rsx! {
        label { class: "cf-option cf-option--inline",
            input {
                r#type: "checkbox",
                "data-sunji-id": "{token}",
                checked,
                onchange: move |_| ctrl.toggle_sunji(token_for_toggle.clone()),
            }
            span { class: "cf-option__body",
                span { class: "cf-option__title", "{label}" }
            }
            if correct {
                span { class: "cf-option__correct", "{tr.create_sunji_correct_badge}" }
            }
        }
    }
}

#[component]
fn FollowTargetsBlock(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;
    let follows = ctrl.follows.read().clone();
    let targets = follows.items.clone();

    if targets.is_empty() {
        return rsx! {
            div {
                class: "cross-filter__chips-empty",
                style: "padding: 20px 4px;",
                "{tr.create_sunji_follow_empty}"
            }
        };
    }

    rsx! {
        div { class: "cf-question",
            div { class: "cf-question__title", "{tr.create_sunji_follow_title}" }
            div { class: "cf-question__options",
                for target in targets.iter() {
                    {
                        let pk_str = target.user_pk.to_string();
                        let token_for_toggle = pk_str.clone();
                        let label = if target.display_name.is_empty() {
                            target.username.clone()
                        } else {
                            target.display_name.clone()
                        };
                        let checked = ctrl.picked_sunji.read().contains(&pk_str);
                        rsx! {
                            label { key: "follow-{pk_str}", class: "cf-option cf-option--inline",
                                input {
                                    r#type: "checkbox",
                                    "data-sunji-id": "{pk_str}",
                                    checked,
                                    onchange: move |_| ctrl.toggle_sunji(token_for_toggle.clone()),
                                }
                                span { class: "cf-option__body",
                                    span { class: "cf-option__title", "{label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KeywordBlock(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;

    let value = ctrl.keyword_input.read().clone();

    rsx! {
        div { class: "cf-question cf-keyword-block",
            div { class: "cf-question__title", "{tr.create_keyword_block_title}" }
            input {
                r#type: "text",
                class: "cf-keyword-input",
                id: "cf-keyword-input",
                "data-testid": "cf-keyword-input",
                placeholder: "{tr.create_keyword_input_placeholder}",
                autocomplete: "off",
                value: "{value}",
                oninput: move |evt| ctrl.keyword_input.set(evt.value()),
            }
            div { class: "cf-keyword-hint", "{tr.create_keyword_hint}" }
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────

fn options_of(question: &Question) -> Vec<String> {
    match question {
        Question::SingleChoice(q) | Question::MultipleChoice(q) => q.options.clone(),
        Question::Checkbox(q) => q.options.clone(),
        Question::Dropdown(q) => q.options.clone(),
        _ => Vec::new(),
    }
}

fn correct_indices_at(answers: &[QuizCorrectAnswer], idx: usize) -> Vec<u32> {
    match answers.get(idx) {
        Some(QuizCorrectAnswer::Single { answer: Some(v) }) => vec![*v as u32],
        Some(QuizCorrectAnswer::Multiple { answers }) => {
            answers.iter().map(|v| *v as u32).collect()
        }
        _ => Vec::new(),
    }
}
