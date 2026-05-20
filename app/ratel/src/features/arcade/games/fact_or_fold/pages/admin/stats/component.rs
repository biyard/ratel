use crate::*;

use super::i18n::FactFoldAdminStatsTranslate;

/// `/admin/fact-or-fold/stats` — Round-level aggregations (accuracy,
/// insider effect, mind-flip count). Real data depends on the
/// `FactFoldRound` entity which lands in PR4. The page renders the
/// mockup-aligned layout with empty-state messages so it still
/// provides a navigable shell today.
#[component]
pub fn FactFoldAdminStatsPage() -> Element {
    let tr: FactFoldAdminStatsTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        section { class: "ff-stats",
            div { class: "ff-stats__notice",
                strong { "{tr.notice_title}" }
                p { "{tr.notice_body}" }
            }

            // KPI strip — placeholder values from spec defaults
            div { class: "ff-stats__kpi",
                StatsKpiTile {
                    label: "{tr.kpi_total_rounds}",
                    value: "—",
                    sub: "{tr.no_data}",
                }
                StatsKpiTile {
                    label: "{tr.kpi_avg_accuracy}",
                    value: "—",
                    sub: "{tr.no_data}",
                }
                StatsKpiTile {
                    label: "{tr.kpi_insider_win_rate}",
                    value: "—",
                    sub: "{tr.no_data}",
                }
                StatsKpiTile {
                    label: "{tr.kpi_avg_flip_count}",
                    value: "—",
                    sub: "{tr.no_data}",
                }
            }

            // Section A — Last 30 rounds chart (placeholder)
            div { class: "ff-stats__panel",
                header { class: "ff-stats__panel-head",
                    span { class: "ff-stats__panel-title", "{tr.panel_recent}" }
                    span { class: "ff-stats__panel-sub", "{tr.panel_recent_sub}" }
                }
                div { class: "ff-stats__chart-placeholder",
                    span { "{tr.chart_placeholder}" }
                }
            }

            // Section B — Per-subject breakdown (placeholder)
            div { class: "ff-stats__panel",
                header { class: "ff-stats__panel-head",
                    span { class: "ff-stats__panel-title", "{tr.panel_breakdown}" }
                    span { class: "ff-stats__panel-sub", "{tr.panel_breakdown_sub}" }
                }
                div { class: "ff-stats__empty", "{tr.empty}" }
            }
        }
    }
}

#[component]
fn StatsKpiTile(label: String, value: String, sub: String) -> Element {
    rsx! {
        div { class: "ff-stats__kpi-tile",
            div { class: "ff-stats__kpi-label", "{label}" }
            div { class: "ff-stats__kpi-value", "{value}" }
            div { class: "ff-stats__kpi-sub", "{sub}" }
        }
    }
}
