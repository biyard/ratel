use crate::*;

/// TODO: 추후 디자인 구현 예정
#[component]
pub fn RankingTable(data: RankingTableData, title: String) -> Element {
    let mut current_page = use_signal(|| 0usize);
    let page_size = data.page_size;
    let total_pages = (data.entries.len() + page_size - 1) / page_size;

    let start_idx = current_page() * page_size;
    let end_idx = (start_idx + page_size).min(data.entries.len());
    let page_entries = &data.entries[start_idx..end_idx];

    rsx! {
        div {
            h4 { "{title}" }
            table {
                thead {
                    tr {
                        for col in data.columns.iter() {
                            th { "{col}" }
                        }
                    }
                }
                tbody {
                    for entry in page_entries.iter() {
                        tr {
                            td { "{entry.rank}" }
                            td { "{entry.name}" }
                            td { "{entry.score} P" }
                            td { "{entry.change}" }
                        }
                    }
                }
            }
            
            div {
                button {
                    disabled: current_page() == 0,
                    onclick: move |_| {
                        if current_page() > 0 {
                            current_page.set(current_page() - 1);
                        }
                    },
                    "Previous"
                }
                span { " Page {current_page() + 1}/{total_pages} " }
                button {
                    disabled: current_page() >= total_pages - 1,
                    onclick: move |_| {
                        if current_page() < total_pages - 1 {
                            current_page.set(current_page() + 1);
                        }
                    },
                    "Next"
                }
            }
        }
    }
}
