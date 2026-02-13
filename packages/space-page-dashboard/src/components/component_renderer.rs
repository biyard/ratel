use crate::*;

/// Renders a single DashboardExtension by matching its component_type
#[component]
pub fn ComponentRenderer(ext: DashboardExtension) -> Element {
    match ext.data {
        DashboardComponentData::StatCard(data) => rsx! {
            StatCard { data }
        },
        DashboardComponentData::StatSummary(data) => rsx! {
            StatSummary { data }
        },
        DashboardComponentData::ProgressList(data) => rsx! {
            ProgressList { data }
        },
        DashboardComponentData::TabChart(data) => rsx! {
            TabChart { data }
        },
        DashboardComponentData::InfoCard(data) => rsx! {
            InfoCard { data }
        },
        DashboardComponentData::RankingTable(data) => rsx! {
            RankingTable { data }
        },
    }
}

/// Renders a grid of DashboardExtension items, sorted by order.
// col-span-1 col-span-2 col-span-3 col-span-4
// row-span-1 row-span-2 row-span-3 row-span-4 row-span-5 row-span-6
#[component]
pub fn DashboardGrid(extensions: Vec<DashboardExtension>) -> Element {
    let mut sorted = extensions;
    sorted.sort_by_key(|e| e.order());

    rsx! {
        div { class: "grid grid-cols-4 grid-rows-6 gap-2.5 w-full h-full min-h-0",
            for ext in sorted.into_iter() {
                {
                    let id = ext.id.clone();
                    let (col_span, row_span) = ext.grid_size();

                    rsx! {
                        div {
                            class: "col-span-{col_span} row-span-{row_span} min-w-0 min-h-0",
                            key: "{id}",
                            ComponentRenderer { ext }
                        }
                    }
                }
            }
        }
    }
}
