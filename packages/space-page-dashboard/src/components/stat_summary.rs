use crate::*;

#[component]
pub fn StatSummary(data: StatSummaryData) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full h-full min-h-0 bg-space-dashboard-card rounded-2xl",
            style: "padding: 1.875rem;",

            // Header Section
            div { class: "flex items-center justify-between mb-5",

                // Left: Icon
                div {
                    class: "bg-table-selection flex items-center justify-center",
                    style: "width: 2.75rem; height: 2.75rem; border-radius: 0.625rem;",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "text-right flex-1",
                    div { class: "text-2xl font-bold text-text-primary", "{data.main_value}" }
                    div { class: "text-[15px] font-semibold mt-1 text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Stats Items
            div { class: "space-y-5 min-h-0 flex-1 overflow-y-auto pr-1",

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
                                    span { style: "color: var(--color-red-600);", "↓ {item.trend:.0}%" }
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
