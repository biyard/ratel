use crate::*;

#[component]
pub fn RankingTable(data: RankingTableData) -> Element {
    let mut current_page = use_signal(|| 0usize);
    let page_size = data.page_size;
    let total_pages = (data.entries.len() + page_size - 1) / page_size;

    let start_idx = current_page() * page_size;
    let end_idx = (start_idx + page_size).min(data.entries.len());
    let page_entries = &data.entries[start_idx..end_idx];

    rsx! {
        div { class: "flex flex-col w-full h-full min-h-0 bg-space-dashboard-card rounded-2xl overflow-hidden",

            // Header
            div {
                class: "py-4 px-[30px] bg-space-dashboard-header",

                div { class: "flex",

                    for col in data.columns.iter() {
                        div { class: "flex-1 px-4 py-4 text-left text-[13px] font-semibold tracking-[-0.14px] text-text-primary",

                            div { class: "flex items-center gap-1",
                                span { "{col}" }
                            }
                        }
                    }
                }
            }

            // Table Body
            div { class: "px-[30px]",

                table { class: "w-full",

                    // Body
                    tbody {
                        for entry in page_entries.iter() {
                            tr { class: "transition-colors",

                                // Rank
                                td { class: "px-4 py-4 text-[13px] font-semibold tracking-[-0.14px] text-text-primary border-b border-separator",
                                    "{entry.rank}"
                                }

                                // Participant (Avatar + Name)
                                td { class: "px-4 py-4 border-b border-separator",

                                    div { class: "flex items-center gap-2",

                                        // Avatar
                                        if !entry.avatar.is_empty() {
                                            img {
                                                class: "w-6 h-6 rounded-full",
                                                src: "{entry.avatar}",
                                                alt: "{entry.name}",
                                            }
                                        } else {
                                            div { class: "flex items-center justify-center w-6 h-6 bg-space-dashboard-accent rounded-full",
                                                span { class: "text-xs font-medium text-space-dashboard-dark",
                                                    "{entry.name.chars().next().unwrap_or('U')}"
                                                }
                                            }
                                        }

                                        // Name
                                        span { class: "text-[13px] font-medium text-text-primary",
                                            "{entry.name}"
                                        }
                                    }
                                }

                                // Point
                                td { class: "px-4 py-4 text-[13px] font-semibold tracking-[-0.14px] text-text-primary border-b border-separator",
                                    "{entry.score:.0} P"
                                }

                                // Score
                                td { class: "px-4 py-4 text-[13px] font-semibold tracking-[-0.14px] text-text-primary border-b border-separator",
                                    "{entry.change}"
                                }
                            }
                        }
                    }
                }
            }

            // Pagination
            div { class: "py-4 px-[30px]",
                Pagination { current_page, total_pages }
            }
        }
    }
}
