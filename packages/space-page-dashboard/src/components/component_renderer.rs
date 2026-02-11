use crate::*;

fn render_component_mismatch(component_type: DashboardComponentType) -> Element {
    rsx! {
        div {
            class: "bg-[var(--color-dashboard-card-bg)] border border-[var(--color-dashboard-border)] rounded-2xl p-4 text-[var(--color-dashboard-text-secondary)] text-sm",
            "Component type and data mismatch: {component_type:?}"
        }
    }
}

/// Renders a single DashboardExtension by matching its component_type
#[component]
pub fn ComponentRenderer(ext: DashboardExtension) -> Element {
    match ext.data {
        DashboardComponentData::StatCard(data) => {
            if ext.component_type == DashboardComponentType::StatCard {
                rsx! { StatCard { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
        DashboardComponentData::StatSummary(data) => {
            if ext.component_type == DashboardComponentType::StatSummary {
                rsx! { StatSummary { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
        DashboardComponentData::ProgressList(data) => {
            if ext.component_type == DashboardComponentType::ProgressList {
                rsx! { ProgressList { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
        DashboardComponentData::TabChart(data) => {
            if ext.component_type == DashboardComponentType::TabChart {
                rsx! { TabChart { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
        DashboardComponentData::InfoCard(data) => {
            if ext.component_type == DashboardComponentType::InfoCard {
                rsx! { InfoCard { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
        DashboardComponentData::RankingTable(data) => {
            if ext.component_type == DashboardComponentType::RankingTable {
                rsx! { RankingTable { data } }
            } else {
                render_component_mismatch(ext.component_type)
            }
        }
    }
}

/// Renders a grid of DashboardExtension items, sorted by order.
#[component]
pub fn DashboardGrid(extensions: Vec<DashboardExtension>) -> Element {
    let mut sorted = extensions;
    sorted.sort_by_key(|e| e.order);

    rsx! {
        div {
            class: "grid grid-cols-4 grid-rows-6 gap-2.5 w-full h-full min-h-0",
            for ext in sorted.into_iter() {
                {
                    let id = ext.id.clone();
                    let col_span = ext.col_span.clamp(1, 4);
                    let row_span = ext.row_span.max(1).min(6);

                    let col_class = match col_span {
                        1 => "col-span-1",
                        2 => "col-span-2",
                        3 => "col-span-3",
                        _ => "col-span-4",
                    };

                    let row_class = match row_span {
                        1 => "row-span-1",
                        2 => "row-span-2",
                        3 => "row-span-3",
                        4 => "row-span-4",
                        5 => "row-span-5",
                        6 => "row-span-6",
                        _ => "row-span-1",
                    };

                    rsx! {
                        div {
                            class: "{col_class} {row_class} min-w-0 min-h-0",
                            key: "{id}",
                            ComponentRenderer { ext }
                        }
                    }
                }
            }
        }
    }
}
