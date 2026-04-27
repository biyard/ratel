use super::super::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;

/// "선택된 교차 필터" banner — sits above the split as a full-width
/// strip in result mode. Renders the loaded report's filter chips with
/// per-source colour; meta line shows filter count + creation date.
#[component]
pub fn ReportBanner(report: AnalyzeReport) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let filter_count = report.filters.len();
    let meta = format!(
        "{} {}{} · {} {} {}",
        tr.detail_meta_filter_count,
        filter_count,
        tr.detail_meta_filter_count_unit,
        tr.detail_meta_created_prefix,
        report.created_at,
        report.created_at_time,
    );
    let is_empty = report.filters.is_empty();

    rsx! {
        section { class: "builder-result", "data-state": "result",
            div { class: "builder-result__chips-row",
                div { class: "builder-result__topline",
                    span { class: "builder-result__label", "{tr.detail_active_filters_label}" }
                    span {
                        class: "builder-result__meta",
                        id: "result-meta",
                        "data-testid": "result-meta",
                        "{meta}"
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
