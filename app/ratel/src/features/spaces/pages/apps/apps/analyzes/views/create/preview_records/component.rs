//! Per-source merged record tables — one paginated table per action
//! type that has at least one chip. Each table reads the same set of
//! filters from the controller and slices the generated row dataset
//! by `current_page` × `PREVIEW_PAGE_SIZE`.
//!
//! Mirrors `buildMergedRecordsTable` + `renderPreviewRecords` from the
//! HTML mockup. Column order stays stable: poll → quiz → discussion →
//! follow.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn PreviewRecords(filters: Vec<AnalyzeReportFilter>) -> Element {
    if filters.is_empty() {
        // No filters → "전체" mode. The chip strip above already
        // communicates this; the records area stays empty.
        return rsx! {
            div { class: "preview-records", id: "preview-records" }
        };
    }

    // Group by source while preserving the ORDER of first appearance
    // among the canonical sources. Duplicates within the same source
    // chunk are kept (each becomes its own block of rows).
    const ORDER: [AnalyzeFilterSource; 4] = [
        AnalyzeFilterSource::Poll,
        AnalyzeFilterSource::Quiz,
        AnalyzeFilterSource::Discussion,
        AnalyzeFilterSource::Follow,
    ];

    rsx! {
        div { class: "preview-records", id: "preview-records",
            for src in ORDER.iter() {
                {
                    let src = *src;
                    let group: Vec<AnalyzeReportFilter> = filters
                        .iter()
                        .filter(|f| f.source == src)
                        .cloned()
                        .collect();
                    if group.is_empty() {
                        rsx! {}
                    } else {
                        let table = build_preview_table(src, &group);
                        rsx! {
                            PreviewTableView { key: "{src.as_str()}", table }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PreviewTableView(table: PreviewTable) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut current_page = use_signal(|| 0usize);

    let page_size = PREVIEW_PAGE_SIZE;
    let total = table.rows.len();
    let total_pages = total.div_ceil(page_size).max(1);
    let cp = current_page().min(total_pages.saturating_sub(1));

    let start = cp * page_size;
    let end = (start + page_size).min(total);
    let visible_rows = if start < total {
        &table.rows[start..end]
    } else {
        &[]
    };

    let prev_disabled = cp == 0;
    let next_disabled = cp >= total_pages.saturating_sub(1);
    let page_info = format!("{} / {}", cp + 1, total_pages);

    let src_attr = table.source.as_str();
    let badge = table.source_badge.clone();
    let header_label = table.header_label.clone();
    let count_label = table.count_label.clone();

    rsx! {
        div { class: "prv-table", "data-source": "{src_attr}",
            header { class: "prv-table__head",
                span { class: "prv-table__chip", "data-source": "{src_attr}", "{badge}" }
                span { class: "prv-table__title", "{header_label}" }
                span { class: "prv-table__count", "{count_label}" }
            }
            div { class: "prv-table__scroll",
                table { class: "prv-table__grid",
                    thead {
                        tr {
                            for col in table.columns.iter() {
                                th { key: "{col}", "{col}" }
                            }
                        }
                    }
                    tbody {
                        for (i, row) in visible_rows.iter().enumerate() {
                            tr { key: "row-{start + i}",
                                td { class: "prv-cell--type", "{row.type_label}" }
                                td { class: "prv-cell--id", "{row.display_id}" }
                                td { "{row.third_col}" }
                                td { class: "prv-cell--answer",
                                    "{row.fourth_col}"
                                    if row.correct_tag {
                                        span { class: "prv-tag-correct",
                                            "{tr.create_preview_correct_tag}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            footer { class: "prv-table__pager",
                button {
                    r#type: "button",
                    class: "prv-pager-btn",
                    disabled: prev_disabled,
                    onclick: move |_| {
                        let p = current_page();
                        if p > 0 {
                            current_page.set(p - 1);
                        }
                    },
                    "{tr.create_pager_prev}"
                }
                span { class: "prv-pager-info", "{page_info}" }
                button {
                    r#type: "button",
                    class: "prv-pager-btn",
                    disabled: next_disabled,
                    onclick: move |_| {
                        let p = current_page();
                        if p + 1 < total_pages {
                            current_page.set(p + 1);
                        }
                    },
                    "{tr.create_pager_next}"
                }
            }
        }
    }
}
