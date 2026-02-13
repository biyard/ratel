use crate::*;

#[component]
pub fn TabChart(data: TabChartData) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        div { class: "flex flex-col w-full h-full min-h-0 p-[30px] bg-space-dashboard-card rounded-2xl",

            // Header Section
            div { class: "flex items-start justify-between mb-2",

                // Left: Icon
                div {
                    class: "flex items-center justify-center w-8 h-8 rounded-lg",
                    style: "background-color: {data.icon_bg};",
                    span { class: "text-lg", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "text-right",
                    div { class: "text-xl font-bold text-text-primary", "{data.main_value}" }
                    div { class: "mt-1 text-[15px] font-semibold text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Tab Buttons
            div { class: "flex gap-0 mb-2 bg-black rounded-lg border border-white overflow-hidden",

                for (idx , tab) in data.tabs.iter().enumerate() {
                    button {
                        class: if selected_tab() == idx { "flex-1 px-3 py-1.5 text-xs font-medium hover:bg-white hover:text-black transition-all" } else { "flex-1 px-3 py-1.5 text-xs font-medium text-white transition-all" },
                        class: if idx < data.tabs.len() - 1 { "border-r border-white" },
                        onclick: move |_| selected_tab.set(idx),
                        "{tab.label}"
                    }
                }
            }

            // Chart Content
            div { class: "flex-1 min-h-0",
                if let Some(tab) = data.tabs.get(selected_tab()) {
                    div { class: "h-full min-h-0 pr-1 space-y-2 overflow-y-auto",

                        for cat in tab.categories.iter() {
                            div { class: "flex items-center gap-2",

                                // Category Name
                                div { class: "w-10 text-xs text-white truncate", "{cat.name}" }

                                // Progress Bar
                                div { class: "flex-1",

                                    div { class: "w-full h-1.5 bg-popover rounded-full overflow-hidden",

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
