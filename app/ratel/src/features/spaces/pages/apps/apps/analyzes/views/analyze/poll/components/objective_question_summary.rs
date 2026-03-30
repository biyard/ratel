use crate::features::spaces::pages::apps::apps::analyzes::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzeChoiceStat {
    pub label: String,
    pub count: i64,
    pub percentage: f64,
    pub color: &'static str,
}

#[component]
pub fn BarChart(chart_id: String, entries: Vec<AnalyzeChoiceStat>) -> Element {
    let request = RenderAnalyzeBarChartRequest {
        container_id: chart_id.clone(),
        entries: entries
            .into_iter()
            .map(|entry| AnalyzeChartDatum {
                label: entry.label,
                count: entry.count,
                percentage: entry.percentage,
                color: entry.color.to_string(),
            })
            .collect(),
    };

    use_effect(move || {
        let _ = render_analyze_bar_chart(&request);
    });

    rsx! {
        div { id: "{chart_id}", class: "w-full" }
    }
}

#[component]
pub fn PieChart(chart_id: String, entries: Vec<AnalyzeChoiceStat>) -> Element {
    let request = RenderAnalyzePieChartRequest {
        container_id: chart_id.clone(),
        entries: entries
            .into_iter()
            .map(|entry| AnalyzeChartDatum {
                label: entry.label,
                count: entry.count,
                percentage: entry.percentage,
                color: entry.color.to_string(),
            })
            .collect(),
    };

    use_effect(move || {
        let _ = render_analyze_pie_chart(&request);
    });

    rsx! {
        div { id: "{chart_id}", class: "flex w-full min-w-0 justify-center" }
    }
}

#[component]
pub fn ObjectiveQuestionSummary(
    title: String,
    total_responses: i64,
    response_unit: String,
    chart_base_id: String,
    answers: Vec<AnalyzeChoiceStat>,
    other_answers: Vec<(String, i64)>,
) -> Element {
    let bar_chart_id = format!("{chart_base_id}-bar");
    let pie_chart_id = format!("{chart_base_id}-pie");

    rsx! {
        div { class: "flex w-full flex-col gap-5 rounded-xl border border-input-box-border bg-transparent p-5",
            div { class: "flex items-center justify-between gap-4 border-b border-input-box-border pb-2",
                div { class: "min-w-0 text-base font-semibold text-text-secondary",
                    "{title}"
                }
                div { class: "shrink-0 text-sm font-medium text-text-secondary",
                    "{total_responses} {response_unit}"
                }
            }

            div { class: "flex w-full min-w-0 flex-col gap-3",
                BarChart { chart_id: bar_chart_id, entries: answers.clone() }
                PieChart { chart_id: pie_chart_id, entries: answers }
            }
        }
    }
}
