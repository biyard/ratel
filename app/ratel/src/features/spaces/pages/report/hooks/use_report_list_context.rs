//! Reports list context — turn-key data + create-action for the
//! carousel list page.
//!
//! Two loaders are exposed:
//! - `all_reports` — unfiltered server-side query. Drives the section
//!   stats (Total / Drafts / Published) so they always reflect the
//!   space's true counts regardless of which chip is selected.
//! - `filtered_reports` — reacts to the `filter` signal and fires a
//!   server-side query with the matching `?status=...` parameter on
//!   every chip click. Drives the carousel rows.
//!
//! Keeping the two concerns split means the chip row triggers a real
//! API request (`feedback`: chip click without network call) while
//! the stat counters stay stable across filter changes.

use crate::features::spaces::pages::report::controllers::{
    create_report, delete_report, list_reports, CreateReportRequest,
};
use crate::features::spaces::pages::report::*;
use crate::*;

/// One pending delete target — the report card the user is about to
/// destroy. The overflow menu writes `Pending` on click, the confirm
/// modal reads it for the title, and `cancel_delete` / a successful
/// `handle_delete.call()` clears it back to `None`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PendingDelete {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Copy, DioxusController)]
pub struct UseReportListContext {
    pub space_id: ReadSignal<SpacePartition>,
    /// Unfiltered list — backs the stat badges.
    pub all_reports: Loader<ListResponse<ReportListItem>>,
    /// Status-filtered list — backs the carousel cards. Re-runs every
    /// time `filter` changes (the outer closure of `use_loader` reads
    /// the signal, which is how the loader subscribes; see
    /// `feedback_use_loader_ssr_subscriptions`).
    pub filtered_reports: Loader<ListResponse<ReportListItem>>,
    /// Chip-row selection. Mutated via `set_filter`; the
    /// `filtered_reports` loader auto-restarts on change.
    pub filter: Signal<ReportFilter>,
    /// Which card has its overflow menu open. `None` keeps the
    /// dropdown closed; flipping this open closes any previously open
    /// one (only one is ever visible at a time).
    pub menu_open_for: Signal<Option<String>>,
    /// The report queued for deletion. `Some` opens the confirm modal.
    pub delete_target: Signal<Option<PendingDelete>>,
    /// Create a fresh draft and route into its detail editor. The
    /// create card binds the button's `pending()` state directly off
    /// the action so it can disable itself while the POST is in flight.
    pub handle_create: Action<(), ()>,
    /// Confirmed deletion — fires after the user clicks `삭제` in the
    /// modal. Restarts both loaders on success so the carousel and
    /// stat badges drop the removed card immediately.
    pub handle_delete: Action<(), ()>,
}

impl UseReportListContext {
    /// Rows the carousel should render — already status-filtered by
    /// the server, so the view just iterates without re-filtering.
    pub fn items(&self) -> Vec<ReportListItem> {
        self.filtered_reports().items.clone()
    }

    /// Stat counts always reflect the full, unfiltered list.
    pub fn drafts_count(&self) -> usize {
        self.all_reports()
            .items
            .iter()
            .filter(|r| r.status == ReportStatus::Draft)
            .count()
    }

    pub fn published_count(&self) -> usize {
        self.all_reports()
            .items
            .iter()
            .filter(|r| r.status == ReportStatus::Published)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.all_reports().items.len()
    }

    pub fn filter_value(&self) -> ReportFilter {
        *self.filter.read()
    }

    pub fn set_filter(&mut self, filter: ReportFilter) {
        self.filter.set(filter);
    }

    pub fn menu_open_id(&self) -> Option<String> {
        self.menu_open_for.read().clone()
    }

    pub fn is_menu_open_for(&self, id: &str) -> bool {
        self.menu_open_for
            .read()
            .as_deref()
            .map(|open| open == id)
            .unwrap_or(false)
    }

    pub fn toggle_menu(&mut self, id: &str) {
        let next = if self.is_menu_open_for(id) {
            None
        } else {
            Some(id.to_string())
        };
        self.menu_open_for.set(next);
    }

    pub fn close_menu(&mut self) {
        self.menu_open_for.set(None);
    }

    /// Pulled from the open menu's row, drives the confirm modal.
    pub fn request_delete(&mut self, item: &ReportListItem) {
        self.menu_open_for.set(None);
        self.delete_target.set(Some(PendingDelete {
            id: item.id.clone(),
            title: item.title.clone(),
        }));
    }

    pub fn cancel_delete(&mut self) {
        self.delete_target.set(None);
    }

    pub fn delete_target_value(&self) -> Option<PendingDelete> {
        self.delete_target.read().clone()
    }
}

#[track_caller]
pub fn use_report_list_context() -> UseReportListContext {
    use_context()
}

#[track_caller]
pub fn use_report_list_context_provider(
    space_id: ReadSignal<SpacePartition>,
) -> Result<UseReportListContext, Loading> {
    let filter = use_signal(|| ReportFilter::All);
    let menu_open_for = use_signal(|| Option::<String>::None);
    let mut delete_target = use_signal(|| Option::<PendingDelete>::None);

    // Unfiltered loader — outer closure reads only `space_id`, so it
    // doesn't re-fire when the chip filter changes.
    let mut all_reports = use_loader(move || {
        let sid = space_id();
        async move { list_reports(sid, None, None).await }
    })?;

    // Filtered loader — outer closure reads `filter`, which is how
    // the loader subscribes to filter changes. On every chip click
    // the signal mutation re-runs this closure, which fires a fresh
    // request with the new `?status=...` value.
    let mut filtered_reports = use_loader(move || {
        let sid = space_id();
        let status = filter().to_status();
        async move { list_reports(sid, None, status).await }
    })?;

    let nav = use_navigator();
    let handle_create = use_action(move || async move {
        let resp = create_report(space_id(), CreateReportRequest::default()).await?;
        // Both loaders need to refresh: the unfiltered list (for
        // accurate stats) and the filtered list (so the new draft
        // shows up immediately when "All" or "Drafts" is selected).
        all_reports.restart();
        filtered_reports.restart();
        nav.push(Route::ReportDetailPage {
            space_id: space_id(),
            report_id: resp.id,
        });
        Ok::<(), crate::common::Error>(())
    });

    // Confirmed delete — reads the queued `PendingDelete` for the
    // target id, fires DELETE, then clears the target + refreshes the
    // loaders so both the carousel and stat badges drop the removed
    // row in one render pass.
    let handle_delete = use_action(move || async move {
        let pending = delete_target.peek().clone();
        let Some(p) = pending else {
            return Ok::<(), crate::common::Error>(());
        };
        let report_id_typed: SpaceReportEntityType = p.id.clone().into();
        delete_report(space_id(), report_id_typed).await?;
        delete_target.set(None);
        all_reports.restart();
        filtered_reports.restart();
        Ok::<(), crate::common::Error>(())
    });

    let ctx = use_context_provider(move || UseReportListContext {
        space_id,
        all_reports,
        filtered_reports,
        filter,
        menu_open_for,
        delete_target,
        handle_create,
        handle_delete,
    });

    Ok(ctx)
}
