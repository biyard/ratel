use crate::*;

/// Renders a single DashboardExtension by matching its component_type
#[component]
pub fn ComponentRenderer(ext: DashboardExtension) -> Element {
    match ext.data {
        DashboardComponentData::StatCard(data) => rsx! {
            StatCard { data }
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

/// Renders dashboard extensions split into cards (top grid) and tables (bottom).
// col-span-1 col-span-2 col-span-3 col-span-4
#[component]
pub fn DashboardGrid(extensions: Vec<DashboardExtension>) -> Element {
    let mut sorted = extensions;
    sorted.sort_by_key(|e| e.order());

    let cards: Vec<_> = sorted.iter().filter(|e| e.is_card()).cloned().collect();
    let tables: Vec<_> = sorted.iter().filter(|e| !e.is_card()).cloned().collect();

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full h-full min-h-0",
            // Card grid
            if !cards.is_empty() {
                div { class: "grid grid-cols-4 gap-2.5 w-full",
                    for ext in cards.into_iter() {
                        {
                            let id = ext.id.clone();
                            let (col_span, _row_span) = ext.grid_size();

                            rsx! {
                                div {
                                    class: "col-span-{col_span} min-w-0 min-h-0",
                                    key: "{id}",
                                    ComponentRenderer { ext }
                                }
                            }
                        }
                    }
                }
            }

            // Table section (full width)
            for ext in tables.into_iter() {
                {
                    let id = ext.id.clone();
                    rsx! {
                        div {
                            class: "w-full min-w-0 min-h-0",
                            key: "{id}",
                            ComponentRenderer { ext }
                        }
                    }
                }
            }
        }
    }
}
