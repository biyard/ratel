use crate::*;

#[component]
pub fn ProgressList(data: ProgressListData) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full h-full min-h-0 p-[30px] bg-space-dashboard-card rounded-2xl",

            // Header Section
            div { class: "flex items-center justify-between mb-5",

                // Left: Icon
                div {
                    class: "flex items-center justify-center w-11 h-11 bg-yellow-500 rounded-[10px]",
                    span { class: "text-2xl", "{data.icon}" }
                }

                // Right: Main Stats
                div { class: "flex-1 text-right",
                    div { class: "text-2xl font-bold text-white", "{data.main_value}" }
                    div { class: "mt-1 text-[15px] font-semibold text-space-dashboard-muted",
                        "{data.main_label}"
                    }
                }
            }

            // Progress Items
            div { class: "flex-1 min-h-0 pr-1 space-y-5 overflow-y-auto",

                for item in data.items.iter() {
                    div { class: "space-y-2",

                        // Label and Value Row
                        div { class: "flex items-center justify-between text-white",

                            // Label
                            span { class: "text-xs truncate", "{item.label}" }

                            // Current Value
                            span { class: "text-xs font-semibold", "{item.current:.0}" }
                        }

                        // Progress Bar
                        div { class: "w-full h-2 bg-popover rounded-full overflow-hidden",

                            div {
                                class: "h-full bg-yellow-500 rounded-full transition-all duration-300",
                                style: "width: {(item.current / item.total * 100.0).min(100.0):.1}%;",
                            }
                        }

                        // Completed Text
                        div { class: "text-xs text-space-dashboard-muted",
                            "{item.current:.0} / {item.total:.0} Completed"
                        }
                    }
                }
            }
        }
    }
}
