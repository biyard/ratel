//! PREVIEW step card — name input, chip summary, and the live count
//! tiles (matched respondents + total source records). Pulls from the
//! `preview` signal which `handle_compute_preview` populates when the
//! user pressed 다음 in the previous step. Below the tiles we group
//! the response's `sample_records` by `filter_idx` and render a tiny
//! `RecordsTable` per chip so the user can recognize which raw rows
//! the analysis will consume before pressing 보고서 생성.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn PreviewCard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;

    let filters = ctrl.filters.read().clone();
    let preview_name = ctrl.preview_name.read().clone();
    let preview = ctrl.preview.read().clone();

    let respondents = preview
        .as_ref()
        .map(|p| p.respondent_count.to_string())
        .unwrap_or_else(|| "—".to_string());
    let data_count = preview
        .as_ref()
        .map(|p| p.data_count.to_string())
        .unwrap_or_else(|| "—".to_string());
    let sample_records: Vec<AnalyzeRecordRow> = preview
        .as_ref()
        .map(|p| p.sample_records.clone())
        .unwrap_or_default();

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
            // Empty filter list: only the respondent_count tile is
            // meaningful (data_count would always be 0), so collapse
            // to a single full-width tile via the `--solo` modifier.
            div { class: if filters.is_empty() { "preview-stats preview-stats--solo" } else { "preview-stats" },
                div { class: "preview-stat",
                    span {
                        class: "preview-stat__value",
                        id: "preview-count",
                        "data-testid": "preview-count",
                        "{respondents}"
                    }
                    span { class: "preview-stat__label", "{tr.create_preview_stat_respondents}" }
                }
                if !filters.is_empty() {
                    div { class: "preview-stat",
                        span {
                            class: "preview-stat__value",
                            id: "preview-data-count",
                            "data-testid": "preview-data-count",
                            "{data_count}"
                        }
                        span { class: "preview-stat__label", "{tr.create_preview_stat_records}" }
                    }
                }
            }

            // ── Sample raw-data tables (one per filter chip) ──
            // Only shown when at least one chip is active. The "전체"
            // branch (no chips) intentionally hides this section
            // because there's no per-source frame to draw under.
            if !filters.is_empty() {
                span { class: "builder-label", "{tr.create_preview_records_label}" }
                span { class: "builder-hint", "{tr.create_preview_records_limit_hint}" }
                if sample_records.is_empty() {
                    span { class: "builder-hint", "{tr.create_preview_records_empty}" }
                } else {
                    div { class: "preview-records-groups",
                        for (idx, f) in filters.iter().enumerate() {
                            {
                                let idx_u32 = idx as u32;
                                let group: Vec<AnalyzeRecordRow> = sample_records
                                    .iter()
                                    .filter(|r| r.filter_idx == idx_u32)
                                    .cloned()
                                    .collect();
                                let src = f.source.as_str();
                                let badge = f.source_label.clone();
                                let label = f.label.clone();
                                let is_empty = group.is_empty();
                                let body = match is_empty {
                                    true => rsx! {
                                        div { class: "preview-records-group__empty",
                                            "{tr.records_empty}"
                                        }
                                    },
                                    false => rsx! {
                                        RecordsTable { source: f.source, rows: group }
                                    },
                                };
                                rsx! {
                                    div {
                                        key: "preview-records-group-{idx}",
                                        class: "preview-records-group",
                                        div { class: "preview-records-group__head",
                                            span {
                                                class: "preview-chip",
                                                "data-source": "{src}",
                                                span { class: "preview-chip__source", "{badge}" }
                                                span { "{label}" }
                                            }
                                        }
                                        {body}
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
