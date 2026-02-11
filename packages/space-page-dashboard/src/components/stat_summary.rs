use crate::*;

#[component]
pub fn StatSummary(data: StatSummaryData) -> Element {
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
            
            // Stats Items
            div {
                class: "space-y-2 min-h-0 flex-1 overflow-y-auto pr-1",
                
                for item in data.items.iter() {
                    div {
                        class: "flex items-center justify-between",
                        
                        // Left: Label and Trend
                        div {
                            class: "flex flex-col gap-0.5",
                            
                            // Label
                            div {
                                class: "text-xs text-[var(--color-dashboard-text-primary)] truncate",
                                "{item.label}"
                            }
                            
                            // Trend
                            div {
                                class: "flex items-center gap-1 text-xs",
                                
                                // Arrow and percentage
                                if item.trend > 0.0 {
                                    span {
                                        class: "text-[var(--color-dashboard-icon-bg-green)] text-xs",
                                        "↑ +{item.trend:.0}%"
                                    }
                                } else if item.trend < 0.0 {
                                    span {
                                        class: "text-[var(--color-dashboard-badge-bg)] text-xs",
                                        "↓ {item.trend:.0}%"
                                    }
                                } else {
                                    span {
                                        class: "text-[var(--color-dashboard-text-tertiary)] text-xs",
                                        "→ 0%"
                                    }
                                }
                                
                                // Trend Label (e.g., "7d")
                                if !item.trend_label.is_empty() {
                                    span {
                                        class: "text-[var(--color-dashboard-text-tertiary)] text-xs",
                                        "{item.trend_label}"
                                    }
                                }
                            }
                        }
                        
                        // Right: Icon and Value
                        div {
                            class: "flex items-center gap-1.5",
                            
                            // Icon
                            if !item.icon.is_empty() {
                                span {
                                    class: "text-[var(--color-dashboard-text-secondary)]",
                                    "{item.icon}"
                                }
                            }
                            
                            // Value
                            span {
                                class: "text-sm font-semibold text-[var(--color-dashboard-text-primary)]",
                                "{item.value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
