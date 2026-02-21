use crate::*;

#[component]
pub fn ProgressList(data: ProgressListData) -> Element {
    rsx! {
        div { class: "flex h-full w-full min-h-0 flex-col p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-space-dashboard-card",

            // Header Section
            div { class: "mb-5 max-mobile:mb-4 flex items-center justify-between gap-3 max-mobile:gap-2",

                // Left: Icon
                div { class: "flex h-11 w-11 shrink-0 flex-col items-center justify-center gap-2.5 px-6 py-3 max-mobile:h-9 max-mobile:w-9 rounded-[10px] bg-yellow-500",
                    icons::ratel::Thunder {
                        width: "24",
                        height: "24",
                        class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5",
                    }
                }

                // Right: Main Stats
                div { class: "flex-1 min-w-0 text-right",
                    div { class: "self-stretch text-right text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary sp-dash-font-inter",
                        "{data.main_value}"
                    }
                    div { class: "mt-1 self-stretch text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-space-dashboard-muted sp-dash-font-raleway",
                        "{data.main_label}"
                    }
                }
            }

            // Progress Items
            div { class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",

                for item in data.items.iter() {
                    div { class: "space-y-2 max-mobile:space-y-1.5",

                        // Label and Value Row
                        div { class: "flex items-center justify-between gap-2 max-mobile:gap-1",

                            // Label
                            span { class: "min-w-0 truncate text-text-primary text-xs leading-4 font-medium sp-dash-font-raleway max-mobile:text-[11px]",
                                "{item.label}"
                            }

                            // Current Value
                            span { class: "shrink-0 text-text-primary text-xs leading-4 font-medium sp-dash-font-raleway max-mobile:text-[11px]",
                                "{item.current:.0}"
                            }
                        }

                        // Progress Bar
                        div { class: "h-2 w-full overflow-hidden rounded-full bg-popover",

                            div {
                                class: "h-full rounded-full bg-yellow-500 transition-all duration-300",
                                style: "width: {(item.current / item.total * 100.0).min(100.0):.1}%;",
                            }
                        }

                        // Completed Text
                        div { class: "flex items-center gap-1 text-xs max-mobile:text-[11px] text-space-dashboard-muted",
                            span { class: "text-space-dashboard-muted text-xs leading-4 font-medium sp-dash-font-inter max-mobile:text-[11px]",
                                "{item.current:.0} / {item.total:.0}"
                            }
                            span { class: "text-space-dashboard-muted text-xs leading-4 font-medium sp-dash-font-raleway max-mobile:text-[11px]",
                                "Completed"
                            }
                        }
                    }
                }
            }
        }
    }
}
