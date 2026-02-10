use crate::*;

/// Renders a single DashboardExtension by matching its widget_type
#[component]
pub fn WidgetRenderer(ext: DashboardExtension) -> Element {
    match (&ext.widget_type, &ext.data) {
        (DashboardWidgetType::StatCard, DashboardWidgetData::StatCard(data)) => {
            rsx! { StatCard { data: data.clone(), title: ext.title.clone() } }
        }
        (DashboardWidgetType::StatSummary, DashboardWidgetData::StatSummary(data)) => {
            rsx! { StatSummary { data: data.clone(), title: ext.title.clone() } }
        }
        (DashboardWidgetType::ProgressList, DashboardWidgetData::ProgressList(data)) => {
            rsx! { ProgressList { data: data.clone(), title: ext.title.clone() } }
        }
        (DashboardWidgetType::TabChart, DashboardWidgetData::TabChart(data)) => {
            rsx! { TabChart { data: data.clone(), title: ext.title.clone() } }
        }
        (DashboardWidgetType::InfoCard, DashboardWidgetData::InfoCard(data)) => {
            rsx! { InfoCard { data: data.clone(), title: ext.title.clone() } }
        }
        (DashboardWidgetType::RankingTable, DashboardWidgetData::RankingTable(data)) => {
            rsx! { RankingTable { data: data.clone(), title: ext.title.clone() } }
        }
        _ => {
            rsx! {
                div {
                    class: "bg-yellow-900/20 border border-yellow-700 rounded-2xl p-4 text-yellow-400 text-sm",
                    "Unknown widget type"
                }
            }
        }
    }
}

/// Renders a grid of DashboardExtension items, sorted by order
#[component]
pub fn DashboardGrid(extensions: Vec<DashboardExtension>) -> Element {
    let mut sorted = extensions.clone();
    sorted.sort_by_key(|e| e.order);

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
            for ext in sorted.into_iter() {
                {
                    let span_class = match ext.span {
                        2 => "md:col-span-2",
                        3 => "md:col-span-2 lg:col-span-3",
                        4 => "md:col-span-2 lg:col-span-4",
                        _ => "",
                    };
                    rsx! {
                        div {
                            class: "{span_class}",
                            key: "{ext.id}",
                            WidgetRenderer { ext }
                        }
                    }
                }
            }
        }
    }
}
