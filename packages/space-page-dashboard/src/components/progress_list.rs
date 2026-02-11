use crate::*;

#[component]
pub fn ProgressList(data: ProgressListData) -> Element {
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
            
            // Progress Items
            div {
                class: "space-y-2 min-h-0 flex-1 overflow-y-auto pr-1",
                
                for item in data.items.iter() {
                    div {
                        class: "space-y-1.5",
                        
                        // Label and Value Row
                        div {
                            class: "flex items-center justify-between",
                            
                            // Label
                            span {
                                class: "text-xs text-[var(--color-dashboard-text-primary)] truncate",
                                "{item.label}"
                            }
                            
                            // Current Value
                            span {
                                class: "text-sm font-semibold text-[var(--color-dashboard-text-primary)]",
                                "{item.current:.0}"
                            }
                        }
                        
                        // Progress Bar
                        div {
                            class: "w-full bg-[var(--color-dashboard-progress-bg)] rounded-full h-1.5 overflow-hidden",
                            
                            div {
                                class: "h-full rounded-full transition-all duration-300",
                                style: "width: {(item.current / item.total * 100.0).min(100.0):.1}%; background-color: {item.color};",
                            }
                        }
                        
                        // Completed Text
                        div {
                            class: "text-xs text-[var(--color-dashboard-text-secondary)]",
                            "{item.current:.0} / {item.total:.0} Completed"
                        }
                    }
                }
            }
        }
    }
}
