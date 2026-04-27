//! Cross-filter card — the heart of the CREATE wizard.
//!
//! Owns three internal states (`idle | picking-action | picking-item`)
//! driven by `data-add-state` on the section. Class names match
//! `assets/design/analyze-create-arena.html` verbatim — the JS in the
//! mockup is replaced wholesale by Dioxus signals on `UseAnalyzeCreate`.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn CrossFilterCard() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;

    let add_state = ctrl.add_state.read().clone();
    let filters = ctrl.filters.read().clone();
    let picking_type = ctrl.picking_type.read().clone();
    let picked_item_id = ctrl.picked_item_id.read().clone();

    // The cross-filter collapses to a chips-only summary the moment the
    // cf-sunji card opens (see CSS rule scoped on `data-sunji-open`).
    // We compute it here so the consumer markup stays declarative.
    let sunji_open = matches!(add_state, AddState::PickingItem) && picked_item_id.is_some();

    let add_state_attr = add_state.as_str();
    let empty_attr = if filters.is_empty() { "true" } else { "false" };
    let sunji_open_attr = if sunji_open { "true" } else { "false" };

    let pick_label_full = if let Some(src) = picking_type {
        format!(
            "{} {}",
            src.type_label(),
            tr.create_cf_pick_item_label_with_type
        )
    } else {
        tr.create_cf_pick_item_label.to_string()
    };

    rsx! {
        section {
            class: "cross-filter",
            id: "cross-filter",
            "data-add-state": "{add_state_attr}",
            "data-empty": "{empty_attr}",
            "data-sunji-open": "{sunji_open_attr}",

            // ── Head ────────────────────────────────────
            header { class: "cross-filter__head",
                h2 { class: "cross-filter__title",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
                    }
                    "{tr.create_cf_title}"
                }
                p { class: "cross-filter__hint", "{tr.create_cf_hint}" }
            }

            // ── Selected chips strip (always visible) ───
            div { class: "cross-filter__chips", id: "cross-filter-chips",
                if filters.is_empty() {
                    span { class: "cross-filter__chips-all", "{tr.create_cf_chips_all}" }
                } else {
                    for (idx, f) in filters.iter().enumerate() {
                        {
                            let src = f.source.as_str();
                            let badge = f.source.badge();
                            let label = f.label.clone();
                            rsx! {
                                span { key: "chip-{idx}", class: "filter-chip", "data-source": "{src}",
                                    span { class: "filter-chip__source", "{badge}" }
                                    span { class: "filter-chip__label", "{label}" }
                                    button {
                                        r#type: "button",
                                        class: "filter-chip__remove",
                                        "aria-label": "{tr.create_cf_remove_filter_aria}",
                                        "data-testid": "cf-remove-{idx}",
                                        onclick: move |_| ctrl.remove_filter(idx),
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
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
                            }
                        }
                    }
                }
            }

            // ── IDLE: + 필터 추가 CTA ───────────────────
            div { class: "cross-filter__row", "data-when": "idle",
                button {
                    r#type: "button",
                    class: "cross-filter__add-btn",
                    id: "cf-add-start",
                    "data-testid": "cf-add-start",
                    onclick: move |_| ctrl.start_add(),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2.4",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
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
                    "{tr.create_cf_add_filter}"
                }
            }

            // ── PICKING-ACTION: 4-tile picker ────────────
            div { class: "cross-filter__row", "data-when": "picking-action",
                div { class: "cross-filter__pick-head",
                    span { class: "cross-filter__pick-label", "{tr.create_cf_pick_action_label}" }
                    button {
                        r#type: "button",
                        class: "cross-filter__pick-cancel",
                        "data-cf-back": "true",
                        "data-testid": "cf-cancel-action",
                        onclick: move |_| ctrl.cancel_add(),
                        "{tr.create_cf_pick_action_cancel}"
                    }
                }
                div { class: "cf-action-grid",
                    ActionTile {
                        source: AnalyzeFilterSource::Poll,
                        label: tr.create_action_tile_poll.to_string(),
                        count_text: format!(
                            "{}{}",
                            mock_action_count(AnalyzeFilterSource::Poll),
                            tr.create_action_count_unit,
                        ),
                    }
                    ActionTile {
                        source: AnalyzeFilterSource::Quiz,
                        label: tr.create_action_tile_quiz.to_string(),
                        count_text: format!(
                            "{}{}",
                            mock_action_count(AnalyzeFilterSource::Quiz),
                            tr.create_action_count_unit,
                        ),
                    }
                    ActionTile {
                        source: AnalyzeFilterSource::Discussion,
                        label: tr.create_action_tile_discussion.to_string(),
                        count_text: format!(
                            "{}{}",
                            mock_action_count(AnalyzeFilterSource::Discussion),
                            tr.create_action_count_unit,
                        ),
                    }
                    ActionTile {
                        source: AnalyzeFilterSource::Follow,
                        label: tr.create_action_tile_follow.to_string(),
                        count_text: format!(
                            "{}{}",
                            mock_action_count(AnalyzeFilterSource::Follow),
                            tr.create_action_count_unit,
                        ),
                    }
                }
            }

            // ── PICKING-ITEM: radio list ────────────────
            div { class: "cross-filter__row", "data-when": "picking-item",
                div { class: "cross-filter__pick-head",
                    span {
                        class: "cross-filter__pick-label",
                        id: "cf-pick-type-label",
                        "{pick_label_full}"
                    }
                    button {
                        r#type: "button",
                        class: "cross-filter__pick-cancel",
                        "data-cf-back-to-action": "true",
                        "data-testid": "cf-back-to-action",
                        onclick: move |_| ctrl.back_to_action(),
                        "{tr.create_cf_back_to_action}"
                    }
                }
                ItemsList {}
            }
        }
    }
}

#[component]
fn ActionTile(source: AnalyzeFilterSource, label: String, count_text: String) -> Element {
    let mut ctrl = use_analyze_create()?;
    let action_attr = source.as_str();

    rsx! {
        button {
            r#type: "button",
            class: "cf-action-tile",
            "data-action-type": "{action_attr}",
            "data-testid": "cf-action-{action_attr}",
            onclick: move |_| ctrl.pick_action(source),
            span { class: "cf-action-tile__icon", {action_icon_svg(source)} }
            span { class: "cf-action-tile__label", "{label}" }
            span { class: "cf-action-tile__count", "{count_text}" }
        }
    }
}

fn action_icon_svg(source: AnalyzeFilterSource) -> Element {
    match source {
        AnalyzeFilterSource::Poll => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                line {
                    x1: "18",
                    y1: "20",
                    x2: "18",
                    y2: "10",
                }
                line {
                    x1: "12",
                    y1: "20",
                    x2: "12",
                    y2: "4",
                }
                line {
                    x1: "6",
                    y1: "20",
                    x2: "6",
                    y2: "14",
                }
            }
        },
        AnalyzeFilterSource::Quiz => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9 9a3 3 0 0 1 6 0c0 2-3 3-3 3" }
                line {
                    x1: "12",
                    y1: "17",
                    x2: "12.01",
                    y2: "17",
                }
            }
        },
        AnalyzeFilterSource::Discussion => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }
        },
        AnalyzeFilterSource::Follow => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                circle { cx: "8.5", cy: "7", r: "4" }
                line {
                    x1: "20",
                    y1: "8",
                    x2: "20",
                    y2: "14",
                }
                line {
                    x1: "23",
                    y1: "11",
                    x2: "17",
                    y2: "11",
                }
            }
        },
    }
}

#[component]
fn ItemsList() -> Element {
    let mut ctrl = use_analyze_create()?;
    let picking_type = ctrl.picking_type.read().clone();
    let picked_item_id = ctrl.picked_item_id.read().clone();

    let items: Vec<AnalyzeActionItem> = picking_type
        .map(mock_action_items)
        .unwrap_or_default();

    rsx! {
        div { class: "cf-options-list", id: "cf-options-list",
            for item in items.iter() {
                {
                    let item_id = item.id.clone();
                    let item_id_for_click = item_id.clone();
                    let title = item.title.clone();
                    let meta = item.meta.clone();
                    let checked = picked_item_id.as_deref() == Some(item_id.as_str());
                    rsx! {
                        label {
                            key: "{item_id}",
                            class: "cf-option",
                            "data-testid": "cf-item-{item_id}",
                            // Bind on the label, not the input. `evt.checked()`
                            // on a radio's onchange isn't reliable across
                            // platforms — clicking the label always means
                            // "pick this item", so dispatch directly.
                            onclick: move |_| ctrl.pick_item(item_id_for_click.clone()),
                            input {
                                r#type: "radio",
                                name: "cf-item-pick",
                                "data-item-id": "{item_id}",
                                checked,
                                // No-op handler — without it the radio
                                // becomes uncontrolled and React-style
                                // warnings show in the console.
                                onchange: move |_| {},
                            }
                            span { class: "cf-option__body",
                                span { class: "cf-option__title", "{title}" }
                                span { class: "cf-option__meta", "{meta}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
