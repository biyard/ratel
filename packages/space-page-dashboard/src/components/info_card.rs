use crate::*;

#[component]
pub fn InfoCard(data: InfoCardData) -> Element {
    rsx! {
        div { class: "flex flex-col w-full h-full min-h-0 p-[30px] bg-space-dashboard-card rounded-2xl",

            // Header Section
            div { class: "flex items-center justify-between mb-5",

                // Left: Icon
                div { class: "flex items-center justify-center w-11 h-11 bg-blue-500 rounded-[10px]",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "flex-1 text-right",
                    div { class: "text-2xl font-bold text-white",
                        "{data.main_value}"
                        if !data.unit.is_empty() {
                            span { class: "ml-1 text-[15px] text-white", "{data.unit}" }
                        }
                    }
                    div { class: "mt-1 text-[15px] font-semibold text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Info Items
            div { class: "flex-1 min-h-0 pt-2 pr-1 space-y-2 overflow-y-auto border-t border-separator",

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
