use super::super::*;
use crate::common::chrono::TimeZone;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// "선택된 교차 필터" banner — sits above the split as a full-width
/// strip in result mode. Renders the loaded report's filter chips with
/// per-source colour; meta line shows filter count + creation date.
/// The "사용된 데이터 확인하기" CTA on the top-right routes to the
/// raw-records page; report.id is already the SpaceAnalyzeReport id.
#[component]
pub fn ReportBanner(
    report: AnalyzeReport,
    space_id: ReadSignal<SpacePartition>,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let nav = use_navigator();
    let filter_count = report.filters.len();
    // Render the saved timestamp in the viewer's local timezone — the
    // raw `created_at` is a UTC unix-millis i64, so converting through
    // `chrono::Local` (which uses the browser's clock under WASM)
    // matches what the user sees on their machine instead of UTC.
    let created = crate::common::chrono::Local
        .timestamp_millis_opt(report.created_at)
        .single()
        .map(|dt| dt.format("%Y.%m.%d %H:%M").to_string())
        .unwrap_or_default();
    let meta = format!(
        "{} {}{} · {} {}",
        tr.detail_meta_filter_count,
        filter_count,
        tr.detail_meta_filter_count_unit,
        tr.detail_meta_created_prefix,
        created,
    );
    let is_empty = report.filters.is_empty();
    let report_id = report.id.clone();

    rsx! {
        section { class: "builder-result", "data-state": "result",
            div { class: "builder-result__chips-row",
                div { class: "builder-result__topline",
                    span { class: "builder-result__label", "{tr.detail_active_filters_label}" }
                    div { class: "builder-result__topline-right",
                        span {
                            class: "builder-result__meta",
                            id: "result-meta",
                            "data-testid": "result-meta",
                            "{meta}"
                        }
                        // Hide the "사용된 데이터 확인하기" CTA when no
                        // filters were saved on the report — there's
                        // nothing per-chip to drill into, and the
                        // records page would render an empty chip
                        // strip. The records page also handles the
                        // empty-filters case gracefully if reached
                        // by a deep link, but we keep the entrance
                        // point off the banner UX-wise.
                        if !is_empty {
                            button {
                                r#type: "button",
                                class: "builder-result__records-btn",
                                "data-testid": "result-records-btn",
                                onclick: move |_| {
                                    nav.push(Route::SpaceAnalyzeRecordsPage {
                                        space_id: space_id(),
                                        report_id: report_id.clone(),
                                    });
                                },
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    path { d: "M3 3h18v4H3z" }
                                    path { d: "M3 11h18v4H3z" }
                                    path { d: "M3 19h18v2H3z" }
                                }
                                span { "{tr.detail_records_btn}" }
                            }
                        }
                    }
                }
                div {
                    class: "preview-chips",
                    id: "result-chips",
                    "data-testid": "result-chips",
                    if is_empty {
                        span { class: "preview-chips__empty", "{tr.detail_active_filters_empty}" }
                    } else {
                        for f in report.filters.iter() {
                            {
                                let src = f.source.as_str();
                                rsx! {
                                    span {
                                        key: "preview-chip-{f.item_id}-{f.option_id}",
                                        class: "preview-chip",
                                        "data-source": "{src}",
                                        span { class: "preview-chip__source", "{f.source_label}" }
                                        span { "{f.label}" }
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
