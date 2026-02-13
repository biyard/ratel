use crate::*;

#[component]
pub fn StatCard(data: StatCardData) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5 w-full h-full min-h-0 p-[30px] bg-space-dashboard-card rounded-2xl",

            // Header Section
            div { class: "flex items-start justify-between",

                // Left: Icon
                div { class: "flex items-center justify-center w-11 h-11 bg-brand-primary rounded-[10px]",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "flex-1 text-right",
                    div { class: "flex items-baseline justify-end gap-1",
                        span { class: "text-2xl font-bold text-text-primary", "{data.value}" }
                        if !data.trend_label.is_empty() {
                            span { class: "text-[15px] text-text-primary-muted", "{data.trend_label}" }
                        }
                    }
                    div { class: "mt-0.5 text-[15px] font-semibold text-text-primary-muted",
                        "{data.label}"
                    }
                }
            }

            // Additional Info
            div { class: "flex-1 min-h-0 pr-1 space-y-0.5 overflow-y-auto",
                if !data.total_winners.is_empty() {
                    div { class: "flex items-center justify-between text-text-primary",
                        span { class: "text-xs", "Total Winners" }
                        span { class: "text-xs font-semibold", "{data.total_winners}" }
                    }
                }

                if !data.rank_rate.is_empty() {
                    p { class: "text-xs text-text-primary-muted", "Rank Rate" }
                }
            }

            // Bottom Section
            if !data.incentive_pool.is_empty() {
                div { class: "pt-2 border-t border-separator",
                    div { class: "flex items-center justify-between text-xs text-text-primary",
                        span { "Incentive Pool" }
                        span { class: "font-medium font-mono", "{data.incentive_pool}" }
                    }
                }
            }
        }
    }
}
