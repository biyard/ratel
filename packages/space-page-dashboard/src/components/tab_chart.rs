use crate::*;

#[component]
pub fn TabChart(data: TabChartData) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        div { class: "rounded-2xl p-[1.875rem] flex flex-col w-full h-full min-h-0 bg-space-dashboard-card",

            // Header Section
            div { class: "flex items-start justify-between mb-2",

                // Left: Icon
                div {
                    class: "w-8 h-8 rounded-lg flex items-center justify-center",
                    style: "background-color: {data.icon_bg};",
                    span { class: "text-lg", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "text-right",
                    div { class: "text-xl font-bold --color-font-primary", "{data.main_value}" }
                    div { class: "text-[15px] font-semibold mt-1 text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Tab Buttons
            div {
                class: "flex gap-0 mb-2 rounded-lg overflow-hidden bg-black",
                style: "border: 1px solid",

                for (idx , tab) in data.tabs.iter().enumerate() {
                    button {
                        class: if selected_tab() == idx { "flex-1 px-3 py-1.5 text-xs font-medium transition-all hover:bg-white hover:text-black" } else { "flex-1 px-3 py-1.5 text-xs font-medium transition-all text-white" },
                        style: if idx < data.tabs.len() - 1 { "border-right: 1px solid;" } else { "" },
                        onclick: move |_| selected_tab.set(idx),
                        "{tab.label}"
                    }
                }
            }

            // Chart Content
            div { class: "flex-1 min-h-0",
                if let Some(tab) = data.tabs.get(selected_tab()) {
                    div { class: "space-y-2 min-h-0 h-full overflow-y-auto pr-1",

                        for cat in tab.categories.iter() {
                            div { class: "flex items-center gap-2",

                                // Category Name
                                div { class: "w-10 text-xs text-white truncate", "{cat.name}" }

                                // Progress Bar
                                div { class: "flex-1",

                                    div { class: "w-full bg-popover rounded-full h-1.5 overflow-hidden",

                                        div {
                                            class: "h-full rounded-full transition-all duration-300",
                                            style: "width: {cat.value.min(100.0):.1}%; background-color: {cat.color};",
                                        }
                                    }
                                }

                                // Percentage
                                div { class: "w-10 text-right text-xs text-white truncate",
                                    if !cat.percentage.is_empty() {
                                        "{cat.percentage}"
                                    } else {
                                        "{cat.value:.1}%"
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
