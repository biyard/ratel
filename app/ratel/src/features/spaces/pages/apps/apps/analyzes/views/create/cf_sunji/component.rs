//! cf-sunji card — opens below the cross-filter when an item radio
//! is picked. Lists the item's questions with options as nested
//! checkbox rows. For DISCUSSION items, prepends a comma-separated
//! keyword input that drains into chips on 확인.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn CfSunjiCard() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;

    let picking_type = ctrl.picking_type.read().clone();
    let picked_item_id = ctrl.picked_item_id.read().clone();
    let picked_sunji = ctrl.picked_sunji.read().clone();
    let keyword_input = ctrl.keyword_input.read().clone();

    // Hidden whenever no item is picked. Returning `None` means the
    // section is not rendered, which matches the HTML mockup's
    // `[hidden]` attribute toggling.
    let (item_id, src) = match (picked_item_id.clone(), picking_type) {
        (Some(id), Some(s)) => (id, s),
        _ => {
            return rsx! {
                section { class: "cf-sunji", id: "cf-sunji", hidden: true }
            };
        }
    };

    let item = find_action_item(src, &item_id);
    let title_text = item
        .as_ref()
        .map(|i| i.title.clone())
        .unwrap_or_else(|| tr.create_sunji_default_title.to_string());
    let badge_text = src.badge();
    let src_attr = src.as_str();

    // Discussion items skip the predefined `keywords` question — the
    // keyword input replaces it. LDA topics still render.
    let mut questions = mock_questions_for(&item_id);
    if matches!(src, AnalyzeFilterSource::Discussion) {
        questions.retain(|q| q.id != "keywords");
    }

    let has_visible_questions = !questions.is_empty();
    let is_discussion = matches!(src, AnalyzeFilterSource::Discussion);
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
                    onclick: move |_| ctrl.clear_item(),
                    "{tr.create_sunji_back}"
                }
            }

            div { class: "cf-sunji__list", id: "cf-sunji-list",

                // Discussion: keyword input goes first.
                if is_discussion {
                    KeywordBlock {}
                }

                // Empty state — discussion's keyword input still works
                // even when there are no other LDA-style questions.
                if !has_visible_questions && !is_discussion {
                    div {
                        class: "cross-filter__chips-empty",
                        style: "padding: 20px 4px;",
                        "{tr.create_sunji_empty}"
                    }
                }

                for question in questions.iter() {
                    {
                        let q_id = question.id.clone();
                        let q_title = question.title.clone();
                        let options = question.options.clone();
                        rsx! {
                            div { key: "{q_id}", class: "cf-question",
                                div { class: "cf-question__title", "{q_title}" }
                                div { class: "cf-question__options",
                                    for option in options.iter() {
                                        QuestionOption {
                                            key: "{q_id}-{option.id}",
                                            question_id: q_id.clone(),
                                            option_id: option.id.clone(),
                                            label: option.label.clone(),
                                            correct: option.correct,
                                        }
                                    }
                                }
                            }
                        }
                    }
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
fn QuestionOption(
    question_id: String,
    option_id: String,
    label: String,
    correct: bool,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;

    let token = format!("{}:{}", question_id, option_id);
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
fn KeywordBlock() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;

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
