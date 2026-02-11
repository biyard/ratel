use crate::*;

#[component]
pub fn InfoCard(data: InfoCardData) -> Element {
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
                        if !data.unit.is_empty() {
                            span {
                                class: "ml-1 text-sm text-[var(--color-dashboard-text-secondary)]",
                                "{data.unit}"
                            }
                        }
                    }
                    div {
                        class: "text-xs text-[var(--color-dashboard-text-secondary)] mt-0.5",
                        "{data.main_label}"
                    }
                }
            }
            
            // Info Items
            div {
                class: "space-y-2 pt-2 border-t border-[var(--color-dashboard-border)] flex-1 min-h-0 overflow-y-auto pr-1",
                
                for item in data.items.iter() {
                    div {
                        class: "flex items-center justify-between",
                        
                        // Label
                        span {
                            class: "text-xs text-[var(--color-dashboard-text-primary)] truncate",
                            "{item.label}"
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
