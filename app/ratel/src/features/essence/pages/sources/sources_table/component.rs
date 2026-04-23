use crate::features::essence::pages::sources::*;
use crate::*;

/// Page size — mirrors `use_essence_sources::PAGE_SIZE` / the server default
/// on `list_essences_handler`. Keep these three in sync.
const PAGE_SIZE: usize = 10;

#[component]
pub fn EssenceSourcesTable() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources()?;

    // Current page's rows come from the server already filtered by kind and
    // sorted by the chosen GSI. The client-side search filter still runs
    // (it's a free-text narrowing over the 10 rows of the current page).
    //
    // IMPORTANT: search only filters the current page. A match on page 3
    // won't be found when the user is on page 1. This is a known trade-off
    // of server-driven pagination without a search index; the chip-style
    // filter + stats-backed total N is the authoritative navigation.
    let page_rows: Memo<Vec<EssenceResponse>> = use_memo(move || {
        let list = hook.items.read().clone();
        let query = hook.search_query.read().to_lowercase();
        if query.is_empty() {
            return list;
        }
        list.into_iter()
            .filter(|s| {
                s.title.to_lowercase().contains(&query)
                    || s.source_path.to_lowercase().contains(&query)
            })
            .collect()
    });

    // Authoritative total for the active kind chip — comes from
    // `UserEssenceStats` not the currently-loaded page, so numbers are
    // accurate across pagination. Falls back to the loaded-page size when
    // stats has stale/zero per-kind counters (e.g. pre-migration users):
    // this keeps the table usable before the admin runs the migrate
    // endpoint that back-fills per-kind totals.
    let total = use_memo(move || {
        let s = hook.stats.read();
        let kind = *hook.selected_kind.read();
        let stats_total = match kind {
            KindFilter::All => s.total_sources.max(0) as usize,
            KindFilter::Notion => s.total_notion.max(0) as usize,
            KindFilter::Post => s.total_post.max(0) as usize,
            KindFilter::Comment => s.total_comment.max(0) as usize,
            KindFilter::Poll => s.total_poll.max(0) as usize,
            KindFilter::Quiz => s.total_quiz.max(0) as usize,
        };
        let items_total = hook.items.read().len();
        let page_index = (hook.page_index)();
        let min_from_pagination = page_index * PAGE_SIZE + items_total;
        stats_total.max(min_from_pagination)
    });

    let total_pages = use_memo(move || {
        let t = total();
        if t == 0 { 1 } else { t.div_ceil(PAGE_SIZE) }
    });

    let mut go_to_page = hook.go_to_page;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-sources",
            header { class: "essence-src-head",
                span {}
                span { "{tr.col_title}" }
                span { "{tr.col_words}" }
                span { "{tr.col_last_sync}" }
                span {}
            }

            // Gate the empty state on the loaded page, not the stats counter —
            // stats can lag (or be zero pre-migration) while the server still
            // returns real rows via the GSI scan.
            if page_rows().is_empty() {
                EmptyState {}
            } else {
                for source in page_rows().iter() {
                    SourceRow { key: "{source.id}", source: source.clone() }
                }
            }

            Pagination {
                page_index: (hook.page_index)(),
                page_size: PAGE_SIZE,
                total: total(),
                on_prev: move |_| {
                    let idx = (hook.page_index)();
                    if idx > 0 {
                        go_to_page.call(idx - 1);
                    }
                },
                on_next: move |_| {
                    let idx = (hook.page_index)();
                    if idx + 1 < total_pages() || (hook.has_next)() {
                        go_to_page.call(idx + 1);
                    }
                },
                on_page: move |p: usize| go_to_page.call(p),
            }
        }
    }
}

#[component]
fn SourceRow(source: EssenceResponse) -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let mut hook = use_essence_sources()?;
    let mut menu_open = use_signal(|| false);
    let nav = use_navigator();

    let id_for_delete = source.id.clone();
    let kind_class = match source.source_kind {
        EssenceSourceKind::Notion => "essence-src-icon--notion",
        EssenceSourceKind::Post => "essence-src-icon--post",
        EssenceSourceKind::PostComment | EssenceSourceKind::DiscussionComment => {
            "essence-src-icon--comment"
        }
        EssenceSourceKind::Poll | EssenceSourceKind::Quiz => "essence-src-icon--action",
    };

    let target = navigation_target(&source);
    let is_clickable = target.is_some();

    rsx! {
        div {
            class: "essence-src-row",
            "data-clickable": is_clickable,
            onclick: move |_| {
                if let Some(route) = target.clone() {
                    nav.push(route);
                }
            },
            span { class: "essence-src-icon {kind_class}", {kind_icon(source.source_kind)} }

            div { class: "essence-src-title-wrap",
                // Defensive strip: titles are supposed to be stored as
                // plain text (see `common::utils::html::summarize_plain`
                // used by the indexer), but older rows predating the
                // tag-strip fix may still contain raw markup like
                // `<p>option 1</p>`. Running `strip_html_tags` at render
                // time cleans those up without needing to re-migrate.
                span { class: "essence-src-title",
                    "{crate::common::utils::html::strip_html_tags(&source.title)}"
                }
                // Per-row kind badge. Every row gets one so rows share the
                // same two-line height; comment rows additionally pick up
                // a colored `--comment` / `--discussion` modifier to echo
                // their icon tint.
                span { class: "essence-src-meta",
                    {
                        let (label, modifier) = match source.source_kind {
                            EssenceSourceKind::Notion => (tr.tag_notion, ""),
                            EssenceSourceKind::Post => (tr.tag_post, ""),
                            EssenceSourceKind::PostComment => {
                                (tr.tag_post_comment, "essence-src-meta__badge--comment")
                            }
                            EssenceSourceKind::Poll => (tr.tag_poll, ""),
                            EssenceSourceKind::Quiz => (tr.tag_quiz, ""),
                            EssenceSourceKind::DiscussionComment => {
                                (tr.tag_discussion_comment, "essence-src-meta__badge--discussion")
                            }
                        };
                        rsx! {
                            span { class: "essence-src-meta__badge {modifier}", "{label}" }
                        }
                    }
                }
            }

            span { class: "essence-src-words", "{format_thousands(source.word_count.max(0) as u64)}" }
            span { class: "essence-src-synced", "{format_relative_time(source.updated_at)}" }

            div {
                class: "essence-src-menu-wrap",
                onclick: move |evt| evt.stop_propagation(),
                button {
                    class: "essence-src-more",
                    aria_label: "{tr.row_more_label}",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        let was = menu_open();
                        menu_open.set(!was);
                    },
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
                if menu_open() {
                    div { class: "essence-src-menu",
                        button {
                            class: "essence-src-menu__item essence-src-menu__item--danger",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                let id = id_for_delete.clone();
                                menu_open.set(false);
                                hook.delete_essence.call(id);
                            },
                            "{tr.menu_delete}"
                        }
                    }
                }
            }
        }
    }
}

/// Route to push when a row is clicked. Returns `None` when the source
/// type has no meaningful detail page (e.g. Notion, or a discussion comment
/// whose space was not stored at migration time).
fn navigation_target(source: &EssenceResponse) -> Option<Route> {
    // `source_pk` is the raw partition string (e.g. "FEED#abc", "SPACE#xyz").
    // Strip the `PREFIX#` to get just the id.
    let source_id = source.source_pk.split_once('#').map(|(_, id)| id.to_string());
    let space_id = source
        .space_pk
        .as_ref()
        .and_then(|s| s.split_once('#').map(|(_, id)| id.to_string()));

    match source.source_kind {
        EssenceSourceKind::Post | EssenceSourceKind::PostComment => {
            source_id.map(|id| Route::PostDetail {
                post_id: FeedPartition(id),
            })
        }
        EssenceSourceKind::Poll | EssenceSourceKind::Quiz => {
            source_id.map(|id| Route::SpaceIndexPage {
                space_id: SpacePartition(id),
            })
        }
        EssenceSourceKind::DiscussionComment => {
            space_id.map(|id| Route::SpaceIndexPage {
                space_id: SpacePartition(id),
            })
        }
        EssenceSourceKind::Notion => None,
    }
}

fn kind_icon(kind: EssenceSourceKind) -> Element {
    match kind {
        EssenceSourceKind::Notion => rsx! { "N" },
        EssenceSourceKind::Post => rsx! {
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
        EssenceSourceKind::PostComment | EssenceSourceKind::DiscussionComment => rsx! {
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
        EssenceSourceKind::Poll => rsx! {
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
        EssenceSourceKind::Quiz => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                line {
                    x1: "12",
                    y1: "17",
                    x2: "12.01",
                    y2: "17",
                }
            }
        },
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

/// Windowed page strip: shows up to `WINDOW_SIZE` consecutive numbers
/// centered on the window the current page falls into. Page 0–9 → window
/// `0..10`; page 10–19 → window `10..20`; etc. Clicking next/prev moves
/// one page at a time, so crossing a window boundary (e.g. page 10 → 11)
/// naturally slides the strip to the next chunk of numbers without an
/// ellipsis "jump-to-end" shortcut. The ellipsis variant is retained on
/// `PageEntry` for forward-compat but no longer emitted.
fn compact_page_numbers(current: usize, total_pages: usize) -> Vec<PageEntry> {
    const WINDOW_SIZE: usize = 10;
    if total_pages == 0 {
        return Vec::new();
    }
    let window_idx = current / WINDOW_SIZE;
    let window_start = window_idx * WINDOW_SIZE;
    let window_end = (window_start + WINDOW_SIZE).min(total_pages);
    (window_start..window_end).map(PageEntry::Number).collect()
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

/// Render a unix timestamp as "2m ago" / "3h ago" / "yesterday" / "3d ago".
/// Good enough for the sources table — we don't need minute-level precision
/// past 24h.
fn format_relative_time(ts_seconds: i64) -> String {
    let now = chrono::Utc::now().timestamp();
    let delta = now.saturating_sub(ts_seconds).max(0);

    if delta < 60 {
        return "just now".to_string();
    }
    if delta < 60 * 60 {
        return format!("{}m ago", delta / 60);
    }
    if delta < 24 * 60 * 60 {
        return format!("{}h ago", delta / 3_600);
    }
    if delta < 48 * 60 * 60 {
        return "yesterday".to_string();
    }
    format!("{}d ago", delta / 86_400)
}
