use crate::*;

#[component]
pub fn StatSummary(data: StatSummaryData) -> Element {
    rsx! {
        div { class: "flex h-full w-full min-h-0 flex-col p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-space-dashboard-card",

            // Header Section
            div { class: "mb-5 max-mobile:mb-4 flex items-center justify-between gap-3 max-mobile:gap-2",

                // Left: Icon
                div { class: "flex h-11 w-11 shrink-0 flex-col items-center justify-center gap-2.5 px-6 py-3 max-mobile:h-9 max-mobile:w-9 rounded-[10px] bg-blue-500",
                    icons::graph::BarChart {
                        width: "24",
                        height: "24",
                        class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
                    }
                }

                // Right: Main Stats
                div { class: "flex-1 min-w-0 text-right",
                    div { class: "self-stretch text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary sp-dash-font-inter",
                        "{data.main_value}"
                    }
                    div { class: "mt-1 self-stretch text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-space-dashboard-muted sp-dash-font-raleway",
                        "{data.main_label}"
                    }
                }
            }

            // Stats Items
            div { class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",

                for item in data.items.iter() {
                    div { class: "flex items-center justify-between gap-2",

                        // Left: Label and Trend
                        div { class: "flex min-w-0 flex-col gap-0.5",

                            // Label
                            div { class: "truncate text-text-primary text-xs leading-4 font-medium sp-dash-font-raleway",
                                "{item.label}"
                            }

                            // Trend
                            div { class: "flex items-center gap-1 text-xs leading-4 font-medium sp-dash-font-inter",

                                // Arrow and percentage
                                if item.trend > 0.0 {
                                    div { class: "h-[18px] w-[18px] shrink-0",
                                        icons::arrows::ShapeArrowUp {
                                            width: "18",
                                            height: "18",
                                            class: "h-[18px] w-[18px] text-icon-primary [&>path]:fill-current",
                                        }
                                    }
                                    span { class: "text-space-dashboard-muted", "+{item.trend:.0}%" }
                                } else if item.trend < 0.0 {
                                    span { class: "text-red-600", "↓ {item.trend:.0}%" }
                                } else {
                                    span { class: "text-text-primary", "→ 0%" }
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
                            div { class: "h-[18px] w-[18px] shrink-0",
                                if item.label == "Total Participants" {
                                    icons::user::UserCheck {
                                        width: "18",
                                        height: "18",
                                        class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
                                    }
                                } else if item.label == "Total Likes" {
                                    icons::emoji::ThumbsUp {
                                        width: "18",
                                        height: "18",
                                        class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
                                    }
                                } else if item.label == "Total Comments" {
                                    icons::chat::RoundBubble {
                                        width: "18",
                                        height: "18",
                                        class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current [&>line]:stroke-current",
                                    }
                                } else if item.label == "Total Action" || item.label == "Total Post" {
                                    icons::notes_clipboard::Note1 {
                                        width: "18",
                                        height: "18",
                                        class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
                                    }
                                } else if !item.icon.is_empty() {
                                    span { class: "text-sm text-text-primary leading-[18px]",
                                        "{item.icon}"
                                    }
                                }
                            }

                            // Value
                            span { class: "text-text-primary text-xs leading-4 font-medium sp-dash-font-raleway",
                                "{item.value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
