use crate::*;

#[component]
pub fn InfoCard(data: InfoCardData) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full h-full min-h-0 bg-space-dashboard-card rounded-2xl",
            style: "padding: 1.875rem;",

            // Header Section
            div { class: "flex items-center justify-between mb-5",

                // Left: Icon
                div {
                    class: "flex items-center justify-center bg-blue-500",
                    style: "width: 2.75rem; height: 2.75rem; border-radius: 0.625rem;",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "text-right flex-1",
                    div { class: "text-2xl font-bold text-white",
                        "{data.main_value}"
                        if !data.unit.is_empty() {
                            span { class: "ml-1 text-[15px] text-white", "{data.unit}" }
                        }
                    }
                    div { class: "text-[15px] font-semibold mt-1 text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Info Items
            div { class: "space-y-2 pt-2 border-t border-separator flex-1 min-h-0 overflow-y-auto pr-1",

                for item in data.items.iter() {
                    div { class: "flex items-center justify-between text-white",

                        // Label
                        span { class: "text-xs truncate", "{item.label}" }

                        // Value
                        span { class: "text-xs font-semibold", "{item.value}" }
                    }
                }
            }
        }
    }
}
