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
        div {
            class: "bg-[var(--color-table-bg)] rounded-2xl p-[1.875rem] w-full h-full min-h-0 flex flex-col",
            
            // Table
            div {
                class: "overflow-auto min-h-0 flex-1 -mx-[1.875rem] -mb-[1.875rem]",
                
                table {
                    class: "w-full",
                    
                    // Header
                    thead {
                        tr {
                            class: "bg-[var(--color-table-header-bg)]",
                            
                            for col in data.columns.iter() {
                                th {
                                    class: "px-4 py-4 text-left text-[13px] font-semibold text-[var(--color-table-header-text)] tracking-[-0.14px] border-b border-[var(--color-table-stroke)]",
                                    
                                    div {
                                        class: "flex items-center gap-1",
                                        span { "{col}" }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Body
                    tbody {
                        for entry in page_entries.iter() {
                            tr {
                                class: "hover:bg-[var(--color-table-row-hover)] transition-colors",
                                
                                // Rank
                                td {
                                    class: "px-4 py-4 text-[13px] font-semibold text-[var(--color-dashboard-text-primary)] tracking-[-0.14px] border-b border-[var(--color-table-stroke)]",
                                    "{entry.rank}"
                                }
                                
                                // Participant (Avatar + Name)
                                td {
                                    class: "px-4 py-4 border-b border-[var(--color-table-stroke)]",
                                    
                                    div {
                                        class: "flex items-center gap-2",
                                        
                                        // Avatar
                                        if !entry.avatar.is_empty() {
                                            img {
                                                class: "w-6 h-6 rounded-full",
                                                src: "{entry.avatar}",
                                                alt: "{entry.name}",
                                            }
                                        } else {
                                            div {
                                                class: "w-6 h-6 rounded-full flex items-center justify-center bg-[var(--color-dashboard-avatar-bg)]",
                                                span {
                                                    class: "text-xs text-white font-medium",
                                                    "{entry.name.chars().next().unwrap_or('U')}"
                                                }
                                            }
                                        }
                                        
                                        // Name
                                        span {
                                            class: "text-[13px] font-medium text-[var(--color-dashboard-text-primary)]",
                                            "{entry.name}"
                                        }
                                    }
                                }
                                
                                // Point
                                td {
                                    class: "px-4 py-4 text-[13px] font-semibold text-[var(--color-dashboard-text-primary)] tracking-[-0.14px] border-b border-[var(--color-table-stroke)]",
                                    "{entry.score:.0}"
                                }
                                
                                // Score
                                td {
                                    class: "px-4 py-4 text-[13px] font-semibold text-[var(--color-dashboard-text-primary)] tracking-[-0.14px] border-b border-[var(--color-table-stroke)]",
                                    "{entry.change}"
                                }
                            }
                        }
                    }
                }
            }
            
            // Pagination
            if total_pages > 1 {
                div {
                    class: "flex items-center justify-center gap-2 pt-4",
                    
                    // Previous Button
                    button {
                        class: if current_page() == 0 {
                            "border border-white border-opacity-50 rounded-lg flex items-center justify-center opacity-50 w-8 h-8"
                        } else {
                            "border border-white rounded-lg flex items-center justify-center w-8 h-8 hover:bg-[var(--color-dashboard-card-bg)] transition-colors"
                        },
                        disabled: current_page() == 0,
                        onclick: move |_| {
                            if current_page() > 0 {
                                current_page.set(current_page() - 1);
                            }
                        },
                        "‹"
                    }
                    
                    // Page Numbers
                    for page_num in 0..total_pages {
                        button {
                            class: if current_page() == page_num {
                                "border border-white rounded-lg flex items-center justify-center bg-white text-[var(--color-dashboard-bg)] w-8 h-8 text-[14px] font-bold"
                            } else {
                                "border border-white rounded-lg flex items-center justify-center w-8 h-8 text-[14px] font-bold hover:bg-[var(--color-dashboard-card-bg)] transition-colors"
                            },
                            onclick: move |_| current_page.set(page_num),
                            "{page_num + 1}"
                        }
                    }
                    
                    // Next Button
                    button {
                        class: if current_page() >= total_pages - 1 {
                            "border border-white border-opacity-50 rounded-lg flex items-center justify-center opacity-50 w-8 h-8"
                        } else {
                            "border border-white rounded-lg flex items-center justify-center w-8 h-8 hover:bg-[var(--color-dashboard-card-bg)] transition-colors"
                        },
                        disabled: current_page() >= total_pages - 1,
                        onclick: move |_| {
                            if current_page() < total_pages - 1 {
                                current_page.set(current_page() + 1);
                            }
                        },
                        "›"
                    }
                }
            }
        }
    }
}
