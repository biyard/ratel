use crate::*;

#[component]
pub fn StatCard(data: StatCardData) -> Element {
    rsx! {
        div {
            class: "bg-[var(--color-dashboard-card-bg)] rounded-2xl p-[1.875rem] flex flex-col gap-2.5 w-full h-full min-h-0",
            
            // Header Section
            div {
                class: "flex items-start justify-between",
                
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
                        class: "flex items-baseline justify-end gap-1",
                        span {
                            class: "text-xl font-bold text-[var(--color-dashboard-text-primary)]",
                            "{data.value}"
                        }
                        if !data.trend_label.is_empty() {
                            span {
                                class: "text-xs text-[var(--color-dashboard-text-secondary)]",
                                "{data.trend_label}"
                            }
                        }
                    }
                    div {
                        class: "text-xs text-[var(--color-dashboard-text-secondary)] mt-0.5",
                        "{data.label}"
                    }
                }
            }
            
            // Additional Info
            div {
                class: "space-y-1.5 flex-1 min-h-0 overflow-y-auto pr-1",
                
                div {
                    class: "flex items-center justify-between",
                    span {
                        class: "text-xs text-[var(--color-dashboard-text-primary)]",
                        "Total Winners"
                    }
                    span {
                        class: "text-sm font-semibold text-[var(--color-dashboard-text-primary)]",
                        "10"
                    }
                }
                
                div {
                    span {
                        class: "text-xs text-[var(--color-dashboard-text-secondary)]",
                        "Rank Rate"
                    }
                }
            }
            
            // Bottom Section
            div {
                class: "pt-2 border-t border-[var(--color-dashboard-border)]",
                
                div {
                    class: "flex items-center justify-between",
                    span {
                        class: "text-xs text-[var(--color-dashboard-text-primary)]",
                        "Incentive Pool"
                    }
                    span {
                        class: "text-xs font-medium text-[var(--color-dashboard-text-secondary)] font-mono",
                        "0xA3f9...D2F4"
                    }
                }
            }
        }
    }
}
