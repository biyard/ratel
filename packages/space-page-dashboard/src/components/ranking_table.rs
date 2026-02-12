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
        div { class: "w-full h-full min-h-0 flex flex-col rounded-2xl overflow-hidden bg-space-dashboard-card",

            // Header
            div {
                class: "bg-space-dashboard-header",
                style: "padding: 1rem 1.875rem;",

                div { class: "flex",

                    for col in data.columns.iter() {
                        div { class: "px-4 py-4 text-left text-[13px] font-semibold tracking-[-0.14px] flex-1 text-text-primary",

                            div { class: "flex items-center gap-1",
                                span { "{col}" }
                            }
                        }
                    }
                }
            }

            // Table Body
            div { style: "padding: 0 1.875rem;",

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
                                            div { class: "w-6 h-6 rounded-full flex items-center justify-center bg-space-dashboard-accent",
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
            div { style: "padding: 1rem 1.875rem;",
                Pagination { current_page, total_pages }
            }
        }
    }
}
