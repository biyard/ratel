use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::types::*;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Data picker modal — pick an analyze, then a source tab, then a
/// specific item to insert as a chart block. Wholly driven by the
/// context's `picker_*` signals; selecting an item appends a chart
/// `<figure>` to the body via `insert_chart_for_item` and closes the
/// modal.
#[component]
pub fn DataPicker() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();

    rsx! {
        div {
            class: "report-detail__picker-overlay",
            "data-open": ctx.is_data_picker_open(),
            "aria-hidden": !ctx.is_data_picker_open(),
            onclick: move |_| ctx.close_drawer(),
            div {
                class: "report-detail__picker-panel",
                onclick: move |e| e.stop_propagation(),
                div { class: "report-detail__picker-head",
                    div { class: "report-detail__picker-title-col",
                        span { class: "report-detail__picker-eyebrow", "{tr.picker_eyebrow}" }
                        span { class: "report-detail__picker-title", "{tr.picker_title}" }
                    }
                    button {
                        class: "report-detail__picker-close",
                        "aria-label": tr.picker_close_aria,
                        onclick: move |_| ctx.close_drawer(),
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

                div { class: "report-detail__picker-analyze",
                    span { class: "report-detail__picker-label", "{tr.picker_analyze_label}" }
                    div { class: "report-detail__picker-analyze-list",
                        for analyze in ctx.analyzes() {
                            AnalyzeOption {
                                key: "{analyze.id}",
                                selected: analyze.id == ctx.picker_analyze_id_value(),
                                analyze,
                            }
                        }
                    }
                }

                div { class: "report-detail__picker-tabs", role: "tablist",
                    for src in ActionSource::VARIANTS.iter().copied() {
                        SourceTab { key: "{src.as_token()}", source: src }
                    }
                }

                div { class: "report-detail__picker-items",
                    if ctx.current_picker_items_is_empty() {
                        div { class: "report-detail__picker-empty", "{tr.picker_empty}" }
                    } else {
                        for item in ctx.current_picker_items() {
                            ItemRow { key: "{item.id}", item }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AnalyzeOption(analyze: Analyze, selected: bool) -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();
    let id = analyze.id.clone();
    let respondents = analyze.respondents;
    let total = analyze.total_items();

    rsx! {
        button {
            class: "report-detail__picker-opt",
            "aria-selected": selected,
            r#type: "button",
            onclick: move |_| ctx.picker_analyze_id.set(id.clone()),
            div { class: "report-detail__picker-opt-name", "{analyze.name}" }
            div { class: "report-detail__picker-opt-meta",
                "{respondents} {tr.picker_respondents_unit} · {total} {tr.picker_items_unit}"
            }
            if !analyze.filters.is_empty() {
                div { class: "report-detail__xf-chips",
                    for chip in analyze.filters.iter() {
                        ChipPill { key: "{chip.label}", chip: chip.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ChipPill(chip: CrossFilterChip) -> Element {
    let src = chip.source.as_token();
    let badge = match chip.source {
        ActionSource::Poll => "POLL",
        ActionSource::Quiz => "QUIZ",
        ActionSource::Discussion => "DISCUSSION",
        ActionSource::Follow => "FOLLOW",
    };
    rsx! {
        span { class: "report-detail__xf-chip", "data-source": src,
            span { class: "report-detail__xf-chip-src", "{badge}" }
            span { class: "report-detail__xf-chip-label", "{chip.label}" }
        }
    }
}

#[component]
fn SourceTab(source: ActionSource) -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();
    let label = match source {
        ActionSource::Poll => tr.picker_tab_poll,
        ActionSource::Quiz => tr.picker_tab_quiz,
        ActionSource::Discussion => tr.picker_tab_discussion,
        ActionSource::Follow => tr.picker_tab_follow,
    };
    rsx! {
        button {
            class: "report-detail__picker-tab",
            "data-source": source.as_token(),
            "aria-selected": ctx.picker_source_value() == source,
            disabled: ctx.picker_items_count_for(source) == 0,
            r#type: "button",
            onclick: move |_| ctx.picker_source.set(source),
            span { class: "report-detail__picker-tab-label", "{label}" }
            span { class: "report-detail__picker-tab-count", " · {ctx.picker_items_count_for(source)}" }
        }
    }
}

#[component]
fn ItemRow(item: AnalyzeItem) -> Element {
    let mut ctx = use_report_detail_context();
    let cloned = item.clone();
    rsx! {
        button {
            class: "report-detail__picker-item",
            r#type: "button",
            onclick: move |_| {
                if let Some(analyze) = ctx.current_analyze() {
                    // The picker's source tab decides which aggregate
                    // bucket the item lives in — pass that source down
                    // so the inserted figure is keyed to the right
                    // surface (drives badge color + chart-type swap).
                    let source = ctx.picker_source_value();
                    ctx.insert_chart_from_picker(&analyze, &cloned, source);
                } else {
                    ctx.close_drawer();
                }
            },
            div { class: "report-detail__picker-item-title", "{item.title}" }
            div { class: "report-detail__picker-item-meta", "{item.meta}" }
        }
    }
}
