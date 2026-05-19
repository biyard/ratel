use crate::features::spaces::pages::report::*;

mod candidate_page;
mod creator_page;
mod i18n;
mod list_page;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
use i18n::*;
use list_page::*;
use participant_page::*;
use viewer_page::*;

/// Page-level provider that installs `UseReportListContext` (mock data
/// for now) and renders the carousel list view. Mirrors PR #1593's
/// `SocialLayout` → `TeamArenaLayout` chain: layout publishes the
/// context, the inner view consumes via `use_report_list_context()`.
#[component]
pub fn SpaceReportPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let _ctx = use_report_list_context_provider(space_id)?;

    rsx! {
        ReportListPage {}
    }
}
