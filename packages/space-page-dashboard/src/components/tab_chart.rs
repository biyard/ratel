use crate::*;

#[component]
pub fn TabChart(data: TabChartData) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        div { class: "grid h-full w-full min-h-0 grid-rows-[auto_auto_minmax(0,_1fr)] gap-4 max-mobile:gap-3 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-space-dashboard-card",

            // Header Section
            div { class: "flex items-start justify-between gap-3 max-mobile:gap-2",

                // Left: Icon
                div { class: "flex h-11 w-11 shrink-0 flex-col items-center justify-center gap-2.5 px-6 py-3 max-mobile:h-9 max-mobile:w-9 rounded-[10px] bg-cyan-500",
                    icons::user::UserGroup {
                        width: "24",
                        height: "24",
                        class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
                    }
                }

                // Right: Main Stats
                div { class: "flex-1 min-w-0 text-right",
                    div { class: "self-stretch text-right text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary sp-dash-font-inter",
                        "{data.main_value}"
                    }
                    div { class: "mt-1 self-stretch text-right text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-space-dashboard-muted sp-dash-font-raleway",
                        "{data.main_label}"
                    }
                }
            }

            // Tab Buttons
            div { class: "flex self-stretch items-start justify-end gap-0 overflow-hidden rounded-lg border border-white bg-black",

                for (idx , tab) in data.tabs.iter().enumerate() {
                    {
                        let button_class = format!(
                            "flex-1 px-3 max-mobile:px-2.5 py-1.5 text-xs max-mobile:text-[11px] font-medium transition-all{}{}",
                            if selected_tab() == idx {
                                " hover:bg-white hover:text-black"
                            } else {
                                " text-text-primary"
                            },

                            if idx < data.tabs.len() - 1 { " border-r border-white" } else { "" },
                        );
                        rsx! {
                            button { class: "{button_class}", onclick: move |_| selected_tab.set(idx), "{tab.label}" }
                        }
                    }
                }
            }

            // Chart Content
            div { class: "min-h-0",
                if let Some(tab) = data.tabs.get(selected_tab()) {
                    div { class: "h-full min-h-0 pr-1 space-y-3 max-mobile:space-y-2 overflow-y-auto",

                        for cat in tab.categories.iter() {
                            div { class: "flex self-stretch items-center justify-between gap-2 max-mobile:gap-1.5",

                                // Category Name
                                div { class: "w-10 truncate text-xs max-mobile:text-[11px] text-text-primary",
                                    "{cat.name}"
                                }

                                // Progress Bar
                                div { class: "flex-1",

                                    div { class: "w-full h-1.5 bg-popover rounded-full overflow-hidden",

                                        div {
                                            class: "h-full rounded-full transition-all duration-300",
                                            style: "width: {cat.value.min(100.0):.1}%; background-color: {cat.color};",
                                        }
                                    }
                                }

                                // Percentage
                                div { class: "w-10 truncate text-right text-xs max-mobile:text-[11px] text-text-primary",
                                    if !cat.percentage.is_empty() {
                                        "{cat.percentage}"
                                    } else {
                                        "{cat.value:.1}%"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
