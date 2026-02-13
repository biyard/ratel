use crate::*;

#[component]
pub fn StatSummary(data: StatSummaryData) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full h-full min-h-0 p-[30px] bg-space-dashboard-card rounded-2xl",

            // Header Section
            div { class: "flex items-center justify-between mb-5",

                // Left: Icon
                div {
                    class: "flex items-center justify-center w-11 h-11 bg-table-selection rounded-[10px]",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "flex-1 text-right",
                    div { class: "text-2xl font-bold text-text-primary", "{data.main_value}" }
                    div { class: "mt-1 text-[15px] font-semibold text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Stats Items
            div { class: "flex-1 min-h-0 pr-1 space-y-5 overflow-y-auto",

                for item in data.items.iter() {
                    div { class: "flex items-center justify-between",

                        // Left: Label and Trend
                        div { class: "flex flex-col gap-0.5",

                            // Label
                            div { class: "text-xs truncate text-white", "{item.label}" }

                            // Trend
                            div { class: "flex items-center gap-1 text-xs",

                                // Arrow and percentage
                                if item.trend > 0.0 {
                                    span { class: "text-space-dashboard-muted", "↑ +{item.trend:.0}%" }
                                } else if item.trend < 0.0 {
                                    span { class: "text-red-600", "↓ {item.trend:.0}%" }
                                } else {
                                    span { class: "text-white", "→ 0%" }
                                }

                                // Trend Label (e.g., "7d")
                                if !item.trend_label.is_empty() {
                                    span { class: "text-space-dashboard-muted", "{item.trend_label}" }
                                }
                            }
                        }

                        // Right: Icon and Value
                        div { class: "flex items-center gap-1.5",

                            // Icon
                            if !item.icon.is_empty() {
                                span { class: "text-white", "{item.icon}" }
                            }

                            // Value
                            span { class: "text-xs font-semibold text-white", "{item.value}" }
                        }
                    }
                }
            }
        }
    }
}
