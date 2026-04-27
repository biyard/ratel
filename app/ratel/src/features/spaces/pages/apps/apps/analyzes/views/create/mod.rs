//! Phase-2 stub: Analyze CREATE flow.
//!
//! Real implementation lands in Phase 2 (cross-filter wizard from the
//! mockup). For now this just renders a placeholder so the route
//! resolves and the LIST page's "+" card has somewhere to navigate.

use super::*;

#[component]
pub fn SpaceAnalyzeCreatePage(space_id: ReadSignal<SpacePartition>) -> Element {
    // Reference space_id so the prop isn't flagged as unused — the
    // Phase-2 implementation will build the cross-filter wizard
    // around the real space.
    let _sid = space_id;

    rsx! {
        div { class: "p-8 text-center text-foreground-muted",
            "Analyze create — coming soon (Phase 2)"
        }
    }
}
