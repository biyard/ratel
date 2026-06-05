use crate::features::spaces::pages::report::*;

mod candidate_page;
mod creator_page;
pub(crate) mod detail;
mod i18n;
mod list_page;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
pub use detail::*;
use i18n::*;
use list_page::*;
use participant_page::*;
use viewer_page::*;

/// Page-level provider that installs `UseReportListContext` (mock data
/// for now) and renders the carousel list view. Mirrors PR #1593's
/// `SocialLayout` → `TeamArenaLayout` chain: layout publishes the
/// context, the inner view consumes via `use_report_list_context()`.
///
/// Access is enforced server-side — `list_reports` rejects non-admin
/// callers when the request isn't pinned to `status=Published`, and
/// `create/update/delete_report` already require Creator role. The
/// client just renders whatever the controllers hand back.
#[component]
pub fn SpaceReportPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let _ctx = use_report_list_context_provider(space_id)?;

    rsx! {
        ReportListPage {}
    }
}
