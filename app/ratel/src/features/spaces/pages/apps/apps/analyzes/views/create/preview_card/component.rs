//! PREVIEW step card — name input, chip summary, stat tiles, and the
//! per-source merged record tables. Layout maps verbatim to
//! `assets/design/analyze-create-arena.html` `.builder-create[data-state="preview"]`.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn PreviewCard() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;

    let filters = ctrl.filters.read().clone();
    let preview_name = ctrl.preview_name.read().clone();

    let respondents = pseudo_respondent_count(filters.len()).to_string();
    let filter_count = filters.len().to_string();

    rsx! {
        div { class: "builder-create", "data-state": "preview",

            header {
                class: "cross-filter__head",
                style: "border-bottom: none; padding-bottom: 0;",
                h2 { class: "cross-filter__title",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        polyline { points: "12 6 12 12 16 14" }
                    }
                    "{tr.create_preview_title}"
                }
                p { class: "cross-filter__hint", "{tr.create_preview_hint}" }
            }

            // ── Name input ──────────────────────────────
            span { class: "builder-label", "{tr.create_preview_name_label}" }
            input {
                r#type: "text",
                class: "builder-name-input",
                id: "preview-name",
                "data-testid": "preview-name",
                placeholder: "{tr.create_preview_name_placeholder}",
                value: "{preview_name}",
                oninput: move |evt| ctrl.preview_name.set(evt.value()),
            }
            span { class: "builder-hint", "{tr.create_preview_name_hint}" }

            // ── Chip summary ────────────────────────────
            span { class: "builder-label", "{tr.create_preview_chips_label}" }
            div { class: "preview-chips", id: "preview-chips",
                if filters.is_empty() {
                    span { class: "cross-filter__chips-all", "{tr.create_cf_chips_all}" }
                } else {
                    for (idx, f) in filters.iter().enumerate() {
                        {
                            let src = f.source.as_str();
                            let badge = f.source.type_label();
                            let label = f.label.clone();
                            rsx! {
                                span {
                                    key: "preview-chip-{idx}",
                                    class: "preview-chip",
                                    "data-source": "{src}",
                                    span { class: "preview-chip__source", "{badge}" }
                                    span { "{label}" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Stat tiles ──────────────────────────────
            div { class: "preview-stats",
                div { class: "preview-stat",
                    span {
                        class: "preview-stat__value",
                        id: "preview-count",
                        "data-testid": "preview-count",
                        "{respondents}"
                    }
                    span { class: "preview-stat__label", "{tr.create_preview_stat_respondents}" }
                }
                div { class: "preview-stat",
                    span {
                        class: "preview-stat__value",
                        id: "preview-filter-count",
                        "data-testid": "preview-filter-count",
                        "{filter_count}"
                    }
                    span { class: "preview-stat__label", "{tr.create_preview_stat_records}" }
                }
            }

            // ── Per-source merged record tables ─────────
            PreviewRecords { filters: filters.clone() }
        }
    }
}
