use crate::*;

/// `FactFoldMatchingPage` — Phase A stub. Phase C fills with the
/// matching.html mockup (4 slots + progress bar + rules hint strip +
/// cancel/refund button + auto-redirect to game room on capacity).
#[component]
pub fn FactFoldMatchingPage() -> Element {
    rsx! {
        SeoMeta { title: "Matching · Fact or Fold" }
        section { class: "ff-matching",
            div { style: "padding: 60px 20px; text-align: center; color: var(--text-faint); font-style: italic",
                "Matching room — coming online next step."
            }
        }
    }
}
