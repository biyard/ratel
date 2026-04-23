use crate::features::essence::controllers::*;
use crate::features::essence::types::*;
use crate::*;
use std::collections::HashSet;

/// Page size — mirrors the server-side default on `list_essences_handler`.
/// Keep these two in sync so the numbered pagination stays aligned.
const PAGE_SIZE: usize = 10;

/// Shared state + mutations for the Essence Sources page.
///
/// Pagination is **server-driven**: each page is a separate DynamoDB query
/// whose starting cursor is stored in `page_bookmarks` (index `N` holds the
/// bookmark to START page `N`; index `0` is always `None`). Jumping to a
/// page further than the cache pre-fetches intermediate pages so the
/// DynamoDB cursor chain stays intact.
///
/// `stats` is backed by the `UserEssenceStats` singleton, so the hero,
/// breakdown cards, and table's "total N" reflect the true totals (not
/// whatever happens to be in the currently-loaded page).
#[derive(Clone, Copy, DioxusController)]
pub struct UseEssenceSources {
    pub stats: Loader<EssenceStatsResponse>,

    /// Rows for the currently visible page. Replaced on every fetch.
    pub items: Signal<Vec<EssenceResponse>>,
    pub page_index: Signal<usize>,
    /// `page_bookmarks[N]` is the DynamoDB cursor for the START of page N.
    /// Index `0` is seeded to `None`. Extended as the user fetches forward.
    pub page_bookmarks: Signal<Vec<Option<String>>>,
    /// `true` while a fetch is in flight so the table can render a dim state
    /// on the current rows without unmounting them.
    pub is_loading: Signal<bool>,
    /// `true` when the last fetched page returned a non-null bookmark — i.e.
    /// there is at least one more page after `page_index` for the current
    /// (sort, kind) combination.
    pub has_next: Signal<bool>,

    pub selected_kind: Signal<KindFilter>,
    pub sort_order: Signal<EssenceSort>,
    pub search_query: Signal<String>,
    pub selected_rows: Signal<HashSet<String>>,

    /// Navigate to an absolute page index. Missing cursors between the
    /// current cache end and `target` are fetched sequentially.
    pub go_to_page: Action<(usize,), ()>,
    /// Switch the kind filter. Resets pagination to page 0 and re-fetches.
    pub set_kind: Action<(KindFilter,), ()>,
    /// Switch the sort order. Resets pagination to page 0 and re-fetches.
    pub set_sort: Action<(EssenceSort,), ()>,

    /// Detach a single Essence row (does NOT delete the underlying source).
    pub delete_essence: Action<(String,), ()>,
    /// Detach every currently-selected row in one go.
    pub bulk_remove: Action<(), ()>,
}

#[track_caller]
pub fn use_essence_sources() -> std::result::Result<UseEssenceSources, RenderError> {
    let ctx: Option<UseEssenceSources> = try_use_context();
    if let Some(ctx) = ctx {
        return Ok(ctx);
    }

    let sort_order = use_signal(EssenceSort::default);
    let selected_kind = use_signal(KindFilter::default);
    let search_query = use_signal(String::new);
    let selected_rows: Signal<HashSet<String>> = use_signal(HashSet::new);

    let items: Signal<Vec<EssenceResponse>> = use_signal(Vec::new);
    let page_index = use_signal(|| 0usize);
    let page_bookmarks: Signal<Vec<Option<String>>> = use_signal(|| vec![None]);
    let is_loading = use_signal(|| false);
    let has_next = use_signal(|| false);

    let stats = use_loader(move || async move { get_essence_stats_handler().await })?;

    // Initial fetch — page 0 on first mount.
    {
        let mut items = items;
        let mut is_loading = is_loading;
        let mut has_next = has_next;
        let mut page_bookmarks = page_bookmarks;
        use_hook(move || {
            spawn(async move {
                is_loading.set(true);
                let res = list_essences_handler(
                    Some(sort_query_value(sort_order()).to_string()),
                    None,
                    Some(kind_query_value(selected_kind()).to_string()),
                    Some(PAGE_SIZE as u32),
                )
                .await;
                is_loading.set(false);
                match res {
                    Ok(resp) => {
                        items.set(resp.items);
                        has_next.set(resp.bookmark.is_some());
                        if let Some(next) = resp.bookmark {
                            let mut bks = page_bookmarks.write();
                            if bks.len() < 2 {
                                bks.resize(2, None);
                            }
                            bks[1] = Some(next);
                        }
                    }
                    Err(e) => {
                        crate::error!("essence initial list failed: {e}");
                    }
                }
            });
        });
    }

    let go_to_page = use_action(move |target: usize| async move {
        let mut page_bookmarks = page_bookmarks;
        let mut items = items;
        let mut page_index = page_index;
        let mut is_loading = is_loading;
        let mut has_next = has_next;

        let known_len = page_bookmarks.read().len();
        let mut cursor_idx = known_len.saturating_sub(1);

        // Fill in any cursors we don't have yet by walking forward from the
        // furthest known bookmark. Each step issues one list call and stores
        // its returned cursor as the START of the NEXT page.
        while cursor_idx < target {
            let bm = page_bookmarks
                .read()
                .get(cursor_idx)
                .cloned()
                .flatten();
            let resp = list_essences_handler(
                Some(sort_query_value(sort_order()).to_string()),
                bm,
                Some(kind_query_value(selected_kind()).to_string()),
                Some(PAGE_SIZE as u32),
            )
            .await?;
            let Some(next_bm) = resp.bookmark else {
                // No more data — stop where we are.
                break;
            };
            {
                let mut bks = page_bookmarks.write();
                if bks.len() <= cursor_idx + 1 {
                    bks.resize(cursor_idx + 2, None);
                }
                bks[cursor_idx + 1] = Some(next_bm);
            }
            cursor_idx += 1;
        }

        let clamped_target = target.min(cursor_idx);
        let bm = page_bookmarks
            .read()
            .get(clamped_target)
            .cloned()
            .flatten();

        is_loading.set(true);
        let resp = list_essences_handler(
            Some(sort_query_value(sort_order()).to_string()),
            bm,
            Some(kind_query_value(selected_kind()).to_string()),
            Some(PAGE_SIZE as u32),
        )
        .await;
        is_loading.set(false);

        match resp {
            Ok(resp) => {
                items.set(resp.items);
                page_index.set(clamped_target);
                has_next.set(resp.bookmark.is_some());
                if let Some(next) = resp.bookmark {
                    let mut bks = page_bookmarks.write();
                    if bks.len() <= clamped_target + 1 {
                        bks.resize(clamped_target + 2, None);
                    }
                    bks[clamped_target + 1] = Some(next);
                }
            }
            Err(e) => {
                crate::error!("essence page fetch failed: {e}");
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let set_kind = use_action(move |kind: KindFilter| async move {
        let mut selected_kind = selected_kind;
        if selected_kind() == kind {
            return Ok::<(), crate::common::Error>(());
        }
        selected_kind.set(kind);
        reset_and_reload(
            items,
            page_index,
            page_bookmarks,
            is_loading,
            has_next,
            sort_order(),
            kind,
        )
        .await;
        Ok(())
    });

    let set_sort = use_action(move |sort: EssenceSort| async move {
        let mut sort_order = sort_order;
        if sort_order() == sort {
            return Ok::<(), crate::common::Error>(());
        }
        sort_order.set(sort);
        reset_and_reload(
            items,
            page_index,
            page_bookmarks,
            is_loading,
            has_next,
            sort,
            selected_kind(),
        )
        .await;
        Ok(())
    });

    let delete_essence = use_action(move |id: String| async move {
        delete_essence_handler(id).await?;
        let mut stats_for_refetch = stats;
        stats_for_refetch.restart();
        // After a delete the current page may have shifted — reload from the
        // same page index using the still-valid start bookmark.
        reload_current_page(
            items,
            page_index,
            page_bookmarks,
            is_loading,
            has_next,
            sort_order(),
            selected_kind(),
        )
        .await;
        Ok::<(), crate::common::Error>(())
    });

    let bulk_remove = use_action(move || async move {
        let ids: Vec<String> = selected_rows.read().iter().cloned().collect();
        for id in ids {
            if let Err(e) = delete_essence_handler(id).await {
                crate::error!("bulk_remove: delete failed: {e}");
            }
        }
        let mut selected_for_bulk = selected_rows;
        selected_for_bulk.write().clear();
        let mut stats_for_refetch = stats;
        stats_for_refetch.restart();
        reload_current_page(
            items,
            page_index,
            page_bookmarks,
            is_loading,
            has_next,
            sort_order(),
            selected_kind(),
        )
        .await;
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseEssenceSources {
        stats,
        items,
        page_index,
        page_bookmarks,
        is_loading,
        has_next,
        selected_kind,
        sort_order,
        search_query,
        selected_rows,
        go_to_page,
        set_kind,
        set_sort,
        delete_essence,
        bulk_remove,
    }))
}

/// Reset pagination state and reload page 0 with the given (sort, kind).
async fn reset_and_reload(
    mut items: Signal<Vec<EssenceResponse>>,
    mut page_index: Signal<usize>,
    mut page_bookmarks: Signal<Vec<Option<String>>>,
    mut is_loading: Signal<bool>,
    mut has_next: Signal<bool>,
    sort: EssenceSort,
    kind: KindFilter,
) {
    page_index.set(0);
    page_bookmarks.set(vec![None]);
    is_loading.set(true);
    let res = list_essences_handler(
        Some(sort_query_value(sort).to_string()),
        None,
        Some(kind_query_value(kind).to_string()),
        Some(PAGE_SIZE as u32),
    )
    .await;
    is_loading.set(false);
    match res {
        Ok(resp) => {
            items.set(resp.items);
            has_next.set(resp.bookmark.is_some());
            if let Some(next) = resp.bookmark {
                let mut bks = page_bookmarks.write();
                if bks.len() < 2 {
                    bks.resize(2, None);
                }
                bks[1] = Some(next);
            }
        }
        Err(e) => {
            crate::error!("essence reload failed: {e}");
            items.set(vec![]);
            has_next.set(false);
        }
    }
}

/// Re-fetch the current `page_index` without changing it — used after
/// delete to keep the table in sync with the server.
async fn reload_current_page(
    mut items: Signal<Vec<EssenceResponse>>,
    page_index: Signal<usize>,
    mut page_bookmarks: Signal<Vec<Option<String>>>,
    mut is_loading: Signal<bool>,
    mut has_next: Signal<bool>,
    sort: EssenceSort,
    kind: KindFilter,
) {
    let idx = page_index();
    let bm = page_bookmarks.read().get(idx).cloned().flatten();
    is_loading.set(true);
    let res = list_essences_handler(
        Some(sort_query_value(sort).to_string()),
        bm,
        Some(kind_query_value(kind).to_string()),
        Some(PAGE_SIZE as u32),
    )
    .await;
    is_loading.set(false);
    match res {
        Ok(resp) => {
            items.set(resp.items);
            has_next.set(resp.bookmark.is_some());
            if let Some(next) = resp.bookmark {
                let mut bks = page_bookmarks.write();
                if bks.len() <= idx + 1 {
                    bks.resize(idx + 2, None);
                }
                bks[idx + 1] = Some(next);
            }
        }
        Err(e) => {
            crate::error!("essence page reload failed: {e}");
        }
    }
}

/// Canonical query-string representation of a sort option. The server's
/// `list_essences_handler::parse_sort` matches on these exact values.
fn sort_query_value(s: EssenceSort) -> &'static str {
    match s {
        EssenceSort::LastEditedDesc => "last_edited",
        EssenceSort::WordCountDesc => "word_count",
        EssenceSort::TitleAsc => "title",
    }
}

/// Canonical query-string representation of a kind filter. Mirrors the
/// server's `parse_kind`. `All` sends an empty string so the server skips
/// the filter entirely.
fn kind_query_value(k: KindFilter) -> &'static str {
    match k {
        KindFilter::All => "all",
        KindFilter::Notion => "notion",
        KindFilter::Post => "post",
        KindFilter::Comment => "comment",
        KindFilter::Poll => "poll",
        KindFilter::Quiz => "quiz",
    }
}
