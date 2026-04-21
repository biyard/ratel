use crate::features::essence::controllers::*;
use crate::features::essence::types::*;
use crate::*;
use dioxus_core::CapturedError;
use std::collections::HashSet;

/// Shared state + mutations for the Essence Sources page. Follows the
/// Ratel hook convention (see `features/me/hooks/use_my_spaces.rs`):
/// one `try_use_context` early-return, `use_loader` for server data, and
/// `use_action` wrappers for every mutation so components just call
/// `hook.delete_essence.call(id)` without importing handlers directly.
///
/// When `sort_order` changes the `sources` loader reads it and Dioxus
/// re-subscribes automatically — that triggers a refetch against the
/// matching GSI, so the server returns items already correctly ordered.
/// `stats` is backed by the `UserEssenceStats` singleton so the hero can
/// show accurate totals in one roundtrip, not a paginated sum.
#[derive(Clone, Copy, DioxusController)]
pub struct UseEssenceSources {
    pub sources: Loader<ListResponse<EssenceResponse>>,
    pub stats: Loader<EssenceStatsResponse>,

    pub selected_kind: Signal<KindFilter>,
    pub sort_order: Signal<EssenceSort>,
    pub search_query: Signal<String>,
    /// Set of essence ids the user has checkbox-selected. Exposed so the
    /// bulk-remove bar can read it; `bulk_remove` clears it after running.
    pub selected_rows: Signal<HashSet<String>>,

    /// Detach a single Essence row (does NOT delete the underlying source).
    /// Invoked from the `...` popover on each table row.
    pub delete_essence: Action<(String,), ()>,
    /// Detach every currently-selected row in one go.
    pub bulk_remove: Action<(), ()>,
}

#[track_caller]
pub fn use_essence_sources() -> std::result::Result<UseEssenceSources, Loading> {
    let ctx: Option<UseEssenceSources> = try_use_context();
    if let Some(ctx) = ctx {
        return Ok(ctx);
    }

    let sort_order = use_signal(EssenceSort::default);
    let selected_kind = use_signal(KindFilter::default);
    let search_query = use_signal(String::new);
    let selected_rows: Signal<HashSet<String>> = use_signal(HashSet::new);

    // Reading `sort_order()` inside the closure makes the loader reactive
    // to sort changes — switching dropdown re-runs the fetch against the
    // matching GSI instead of sorting stale data on the client.
    let sources = use_loader(move || {
        let sort = sort_order();
        async move {
            let sort_param = sort_query_value(sort).to_string();
            list_essences_handler(Some(sort_param), None).await
        }
    })?;

    let stats = use_loader(move || async move { get_essence_stats_handler().await })?;

    let mut sources_for_refetch = sources;
    let mut stats_for_refetch = stats;
    let mut selected_for_bulk = selected_rows;

    let delete_essence = use_action(move |id: String| async move {
        delete_essence_handler(id)
            .await
            .map_err(CapturedError::from)?;
        sources_for_refetch.restart();
        stats_for_refetch.restart();
        Ok::<(), CapturedError>(())
    });

    let bulk_remove = use_action(move || async move {
        let ids: Vec<String> = selected_for_bulk.read().iter().cloned().collect();
        for id in ids {
            // Best-effort per row — surface the last error but keep going
            // so one stale id doesn't abort the batch.
            if let Err(e) = delete_essence_handler(id).await {
                crate::error!("bulk_remove: delete failed: {e}");
            }
        }
        selected_for_bulk.write().clear();
        sources_for_refetch.restart();
        stats_for_refetch.restart();
        Ok::<(), CapturedError>(())
    });

    Ok(use_context_provider(move || UseEssenceSources {
        sources,
        stats,
        selected_kind,
        sort_order,
        search_query,
        selected_rows,
        delete_essence,
        bulk_remove,
    }))
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
