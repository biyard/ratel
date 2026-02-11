use crate::*;

#[component]
pub fn TabChart(data: TabChartData) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        div {
            class: "bg-[var(--color-dashboard-card-bg)] rounded-2xl p-[1.875rem] flex flex-col w-full h-full min-h-0",
            
            // Header Section
            div {
                class: "flex items-start justify-between mb-2",
                
                // Left: Icon
                div {
                    class: "w-8 h-8 rounded-lg flex items-center justify-center",
                    style: "background-color: {data.icon_bg};",
                    span {
                        class: "text-lg",
                        "{data.icon}"
                    }
                }
                
                // Right: Main Stats
                div {
                    class: "text-right",
                    div {
                        class: "text-xl font-bold text-[var(--color-dashboard-text-primary)]",
                        "{data.main_value}"
                    }
                    div {
                        class: "text-xs text-[var(--color-dashboard-text-secondary)] mt-0.5",
                        "{data.main_label}"
                    }
                }
            }
            
            // Tab Buttons
            div {
                class: "flex gap-0 mb-2 bg-[var(--color-dashboard-bg)] rounded-lg p-0.5",
                
                for (idx, tab) in data.tabs.iter().enumerate() {
                    button {
                        class: if selected_tab() == idx {
                            "flex-1 px-3 py-1.5 rounded-md text-xs font-medium text-[var(--color-dashboard-text-primary)] bg-[var(--color-dashboard-card-bg-dark)] transition-all"
                        } else {
                            "flex-1 px-3 py-1.5 rounded-md text-xs font-medium text-[var(--color-dashboard-text-secondary)] hover:text-[var(--color-dashboard-text-primary)] transition-all"
                        },
                        onclick: move |_| selected_tab.set(idx),
                        "{tab.label}"
                    }
                }
            }
            
            // Chart Content
            div { class: "flex-1 min-h-0",
                if let Some(tab) = data.tabs.get(selected_tab()) {
                    div {
                        class: "space-y-2 min-h-0 h-full overflow-y-auto pr-1",

                        for cat in tab.categories.iter() {
                            div {
                                class: "flex items-center gap-2",

                                // Category Name
                                div {
                                    class: "w-10 text-xs text-[var(--color-dashboard-text-primary)] truncate",
                                    "{cat.name}"
                                }

                                // Progress Bar
                                div {
                                    class: "flex-1",

                                    div {
                                        class: "w-full bg-[var(--color-dashboard-progress-bg)] rounded-full h-1.5 overflow-hidden",

                                        div {
                                            class: "h-full rounded-full transition-all duration-300",
                                            style: "width: {cat.value.min(100.0):.1}%; background-color: {cat.color};",
                                        }
                                    }
                                }

                                // Percentage
                                div {
                                    class: "w-10 text-right text-xs text-[var(--color-dashboard-text-primary)] truncate",
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
