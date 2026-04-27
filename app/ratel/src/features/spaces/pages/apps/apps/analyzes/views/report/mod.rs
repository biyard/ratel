//! Phase-3 stub: Analyze REPORT detail flow.
//!
//! Real implementation lands in Phase 3 (split-panel result view from
//! the mockup). For now this just renders a placeholder so the route
//! resolves and saved-card clicks have somewhere to navigate.

use super::*;

#[component]
pub fn SpaceAnalyzeReportPage(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Element {
    let _sid = space_id;
    let rid = report_id();

    rsx! {
        div { class: "p-8 text-center text-foreground-muted",
            "Analyze report — coming soon (Phase 3): "
            "{rid}"
        }
    }
}
