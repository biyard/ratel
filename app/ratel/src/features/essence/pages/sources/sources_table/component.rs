use crate::features::essence::pages::sources::*;
use crate::*;

const PAGE_SIZE: usize = 10;

#[component]
pub fn EssenceSourcesTable() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources();
    let mut page_index = use_signal(|| 0usize);

    let filtered: Memo<Vec<EssenceSourceResponse>> = use_memo(move || {
        let list = hook.sources.read().clone();
        let kind = hook.selected_kind.read().clone();
        let status = hook.status_filter.read().clone();
        let query = hook.search_query.read().to_lowercase();
        let sort = *hook.sort_order.read();

        let mut matched: Vec<EssenceSourceResponse> = list
            .into_iter()
            .filter(|s| kind.matches(s.kind))
            .filter(|s| match status {
                StatusFilter::All => true,
                StatusFilter::Active => !s.is_paused(),
                StatusFilter::Paused => s.is_paused(),
                StatusFilter::AiFlagged => s.ai_flagged,
            })
            .filter(|s| {
                if query.is_empty() {
                    return true;
                }
                s.title.to_lowercase().contains(&query)
                    || s.source_path.to_lowercase().contains(&query)
            })
            .collect();

        match sort {
            SortOrder::LastSyncedDesc | SortOrder::LastEditedDesc => {
                // Mock data only has `last_synced_label`; keep insertion order
                // which represents recency in the seed.
            }
            SortOrder::WordCountDesc => {
                matched.sort_by(|a, b| b.word_count.cmp(&a.word_count));
            }
            SortOrder::QualityDesc => {
                matched.sort_by(|a, b| b.quality_score.partial_cmp(&a.quality_score).unwrap_or(std::cmp::Ordering::Equal));
            }
            SortOrder::TitleAsc => {
                matched.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            }
        }

        matched
    });

    let total = use_memo(move || filtered().len());
    let total_pages = use_memo(move || {
        let t = total();
        if t == 0 { 1 } else { t.div_ceil(PAGE_SIZE) }
    });

    use_effect(move || {
        // Reset to page 0 whenever filters shrink results below the current
        // page. Reads are explicit so the effect only runs on those changes.
        if page_index() >= total_pages() {
            page_index.set(0);
        }
    });

    let page_start = use_memo(move || page_index() * PAGE_SIZE);
    let page_end = use_memo(move || (page_start() + PAGE_SIZE).min(total()));
    // Clamp bounds against `list.len()` directly — `total`/`page_end` memos
    // can hold stale values for a render tick right after `filtered`
    // recomputes, which used to crash here with "range end N out of range
    // for slice of length M" the moment a filter shrank the result set.
    let page_rows = use_memo(move || {
        let list = filtered();
        let len = list.len();
        let start = page_start().min(len);
        let end = page_end().min(len).max(start);
        list[start..end].to_vec()
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-sources",
            header { class: "essence-src-head",
                span {}
                span {}
                span { "{tr.col_title}" }
                span { "{tr.col_words}" }
                span { "{tr.col_last_sync}" }
                span { "{tr.col_quality}" }
                span { style: "text-align: center", "{tr.col_in_house}" }
                span {}
            }

            if total() == 0 {
                EmptyState {}
            } else {
                for source in page_rows().iter() {
                    SourceRow { key: "{source.id}", source: source.clone() }
                }
            }

            Pagination {
                page_index: page_index(),
                page_size: PAGE_SIZE,
                total: total(),
                on_prev: move |_| {
                    if page_index() > 0 {
                        page_index.set(page_index() - 1);
                    }
                },
                on_next: move |_| {
                    if page_index() + 1 < total_pages() {
                        page_index.set(page_index() + 1);
                    }
                },
                on_page: move |p: usize| page_index.set(p),
            }
        }
    }
}

#[component]
fn SourceRow(source: EssenceSourceResponse) -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let mut hook = use_essence_sources();
    let selected = use_memo({
        let id = source.id.clone();
        move || hook.selected_rows.read().contains(&id)
    });

    let id_for_check = source.id.clone();
    let id_for_toggle = source.id.clone();
    let kind_class = match source.kind {
        EssenceSourceKind::Notion => "essence-src-icon--notion",
        EssenceSourceKind::RatelPost => "essence-src-icon--post",
        EssenceSourceKind::Comment => "essence-src-icon--comment",
        EssenceSourceKind::Action => "essence-src-icon--action",
    };
    let quality = source.quality();

    rsx! {
        div { class: "essence-src-row", "data-selected": selected(),
            button {
                class: "essence-src-check",
                aria_label: "{tr.row_select_label}",
                onclick: move |_| {
                    let id = id_for_check.clone();
                    let mut set = hook.selected_rows.write();
                    if set.contains(&id) {
                        set.remove(&id);
                    } else {
                        set.insert(id);
                    }
                },
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "20 6 9 17 4 12" }
                }
            }

            span { class: "essence-src-icon {kind_class}",
                match source.kind {
                    EssenceSourceKind::Notion => rsx! { "N" },
                    EssenceSourceKind::RatelPost => rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                            path { d: "M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4z" }
                        }
                    },
                    EssenceSourceKind::Comment => rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                        }
                    },
                    EssenceSourceKind::Action => rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "22 12 18 12 15 21 9 3 6 12 2 12" }
                        }
                    },
                }
            }

            div { class: "essence-src-title-wrap",
                span { class: "essence-src-title", "{source.title}" }
                span { class: "essence-src-meta",
                    span { class: "essence-src-meta__link", "{source.source_path}" }
                    span { class: "essence-src-meta__dot", "·" }
                    "{source.chunks} chunks"
                    if let Some(extra) = source.extra_meta.as_ref() {
                        span { class: "essence-src-meta__dot", "·" }
                        if source.is_paused() {
                            span { class: "essence-src-meta__badge essence-src-meta__badge--paused",
                                "{extra}"
                            }
                        } else if source.ai_flagged {
                            span { class: "essence-src-meta__badge essence-src-meta__badge--flagged",
                                "{extra}"
                            }
                        } else {
                            "{extra}"
                        }
                    }
                }
            }

            span { class: "essence-src-words", "{format_thousands(source.word_count as u64)}" }
            span { class: "essence-src-synced", "{source.last_synced_label}" }

            span { class: "essence-src-quality essence-src-quality--{quality.css_modifier()}",
                svg { view_box: "0 0 24 24", fill: "currentColor",
                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                }
                "{format_score(source.quality_score)}"
            }

            button {
                class: "essence-switch",
                "aria-checked": source.in_house.is_on(),
                aria_label: "{tr.row_in_house_label}",
                onclick: move |_| {
                    hook.toggle_in_house.call(id_for_toggle.clone());
                },
            }

            button { class: "essence-src-more", aria_label: "{tr.row_more_label}",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "1" }
                    circle { cx: "12", cy: "5", r: "1" }
                    circle { cx: "12", cy: "19", r: "1" }
                }
            }
        }
    }
}

#[component]
fn Pagination(
    page_index: usize,
    page_size: usize,
    total: usize,
    on_prev: EventHandler<()>,
    on_next: EventHandler<()>,
    on_page: EventHandler<usize>,
) -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let total_pages = if total == 0 { 1 } else { total.div_ceil(page_size) };
    let start = page_index * page_size + 1;
    let end = ((page_index + 1) * page_size).min(total);
    let shown_start = if total == 0 { 0 } else { start };
    let shown_end = if total == 0 { 0 } else { end };

    let page_numbers = compact_page_numbers(page_index, total_pages);

    rsx! {
        div { class: "essence-pagination",
            span { class: "essence-pagination__info",
                "{tr.pagination_prefix} "
                strong { "{shown_start} – {shown_end}" }
                " {tr.pagination_of} "
                strong { "{format_thousands(total as u64)}" }
            }
            div { class: "essence-pagination__actions",
                button {
                    class: "essence-page-btn",
                    aria_label: "{tr.pagination_previous}",
                    disabled: page_index == 0,
                    onclick: move |_| on_prev.call(()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                for entry in page_numbers.into_iter() {
                    match entry {
                        PageEntry::Number(p) => rsx! {
                            button {
                                key: "p-{p}",
                                class: "essence-page-btn",
                                "aria-current": if p == page_index { "page" } else { "" },
                                onclick: move |_| on_page.call(p),
                                "{p + 1}"
                            }
                        },
                        PageEntry::Ellipsis(key) => rsx! {
                            span { key: "e-{key}", class: "essence-page-btn essence-page-btn--ellipsis", "…" }
                        },
                    }
                }
                button {
                    class: "essence-page-btn",
                    aria_label: "{tr.pagination_next}",
                    disabled: page_index + 1 >= total_pages,
                    onclick: move |_| on_next.call(()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PageEntry {
    Number(usize),
    Ellipsis(usize),
}

/// Produce a compact 1-based page strip: 0 .. current±1 .. last with
/// ellipsis placeholders. Matches the "1, 2, 3, …, 172" layout in the
/// mockup while scaling for any page count.
fn compact_page_numbers(current: usize, total_pages: usize) -> Vec<PageEntry> {
    if total_pages <= 7 {
        return (0..total_pages).map(PageEntry::Number).collect();
    }

    let mut out = Vec::with_capacity(7);
    out.push(PageEntry::Number(0));

    let window_start = current.saturating_sub(1).max(1);
    let window_end = (current + 1).min(total_pages - 2);

    if window_start > 1 {
        out.push(PageEntry::Ellipsis(0));
    }
    for p in window_start..=window_end {
        out.push(PageEntry::Number(p));
    }
    if window_end < total_pages - 2 {
        out.push(PageEntry::Ellipsis(1));
    }
    out.push(PageEntry::Number(total_pages - 1));
    out
}

#[component]
fn EmptyState() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    rsx! {
        div { class: "essence-empty",
            span { class: "essence-empty__title", "{tr.empty_title}" }
            span { class: "essence-empty__sub", "{tr.empty_subtitle}" }
        }
    }
}

fn format_thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

fn format_score(score: f32) -> String {
    format!("{score:.1}")
}
