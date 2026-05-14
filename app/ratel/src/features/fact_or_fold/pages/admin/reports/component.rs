use crate::*;

use super::i18n::FactFoldAdminReportsTranslate;

/// `/admin/fact-or-fold/reports` — Moderation queue for in-round
/// player reports (insider self-doxxing, abusive chat, etc.). The
/// `FactFoldReport` entity + endpoints land in PR4 alongside chat;
/// this page renders the mockup-aligned shell + empty-state today.
#[component]
pub fn FactFoldAdminReportsPage() -> Element {
    let tr: FactFoldAdminReportsTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        section { class: "ff-reports",
            div { class: "ff-reports__notice",
                strong { "{tr.notice_title}" }
                p { "{tr.notice_body}" }
            }

            // Filter tabs (visual only)
            div { class: "ff-reports__tabs",
                ReportTab { label: "{tr.tab_open}", active: true }
                ReportTab { label: "{tr.tab_resolved}", active: false }
                ReportTab { label: "{tr.tab_dismissed}", active: false }
            }

            // List
            div { class: "ff-reports__panel",
                div { class: "ff-reports__empty", "{tr.empty}" }
            }
        }
    }
}

#[component]
fn ReportTab(label: String, active: bool) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "ff-reports__tab",
            "aria-selected": active,
            disabled: true,
            "{label}"
        }
    }
}
