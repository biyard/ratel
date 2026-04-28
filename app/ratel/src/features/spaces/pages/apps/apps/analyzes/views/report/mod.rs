//! Phase-3: Analyze REPORT detail flow.
//!
//! Read-only split-panel result view of a saved analyze report. Layout
//! and class names mirror `assets/design/analyze-detail-arena.html`
//! verbatim — same `arena`, `arena-topbar`, `split`, `builder-result`,
//! `panel`, `card`, `sidebar`, `sb-group`, `sb-item` markup.
//!
//! Sidebar item click → which panel is visible is **JS-owned**: see
//! `script.js`. Dioxus only owns the loaded `AnalyzeReport` (via
//! `UseAnalyzeReportDetail`); everything else (panel switching,
//! sb-group collapse, filter highlight, LDA edit-mode toggle) is JS-
//! driven through `data-*`/`aria-*` attributes that the script flips
//! directly. This keeps the read-only UX feeling instant and matches
//! the home/script.js + action_dashboard precedent.

mod banner;
mod discussion_panel;
mod follow_panel;
mod poll_panel;
mod quiz_panel;
mod sidebar;

pub use banner::*;
pub use discussion_panel::*;
pub use follow_panel::*;
pub use poll_panel::*;
pub use quiz_panel::*;
pub use sidebar::*;

use super::*;
use crate::features::spaces::space_common::hooks::use_space;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

/// Public entrypoint for the report detail view. The `space_id` is
/// kept as a `ReadSignal<SpacePartition>` so the topbar back button
/// can hop back to the list arena via `nav.go_back()` without needing
/// the value at all — but we keep it for parity with the LIST page
/// signature and so future deep-link navigation (e.g. "back to space")
/// can use it.
#[component]
pub fn SpaceAnalyzeReportPage(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();
    let ctrl = use_analyze_report_detail(report_id, space_id)?;

    let detail = ctrl.detail.read().clone();
    let report = detail.report.clone();
    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();
    let report_name = report.name.clone();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "analyze-arena analyze-arena--detail",
            div { class: "arena",
                // ── Topbar ───────────────────────────────────
                header { class: "arena-topbar", role: "banner",
                    div { class: "arena-topbar__left",
                        button {
                            r#type: "button",
                            class: "back-btn",
                            "aria-label": "{tr.detail_back_btn_aria}",
                            "data-testid": "topbar-back",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                path { d: "M19 12H5" }
                                path { d: "M12 19l-7-7 7-7" }
                            }
                        }
                        img {
                            class: "arena-topbar__logo",
                            alt: "Space logo",
                            src: "{space_logo}",
                        }
                        nav { class: "breadcrumb",
                            span { class: "breadcrumb__item", "{space_title}" }
                            span { class: "breadcrumb__sep", "›" }
                            span { class: "breadcrumb__item", "{tr.arena_breadcrumb_apps}" }
                            span { class: "breadcrumb__sep", "›" }
                            span { class: "breadcrumb__item breadcrumb__current",
                                "{tr.arena_breadcrumb_current}"
                            }
                        }
                        span { class: "type-badge", "data-testid": "type-badge",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                path { d: "M3 3v18h18" }
                                path { d: "M7 14l4-4 4 4 5-5" }
                            }
                            "{tr.arena_breadcrumb_current}"
                        }
                        span {
                            class: "topbar-title",
                            id: "arena-title",
                            "data-testid": "arena-title",
                            "{report_name}"
                        }
                    }
                }

                // ── Body ────────────────────────────────────
                div { class: "split", "data-mode": "result",
                    // Banner — full-width, spans both columns in result mode
                    ReportBanner { report: report.clone() }

                    // Main column — all four panels live here. JS owns
                    // which is visible via `data-active`.
                    main { class: "main",
                        PollPanel {}
                        QuizPanel {}
                        DiscussionPanel {}
                        FollowPanel {}
                    }

                    // Sidebar — ANALYZES groups
                    ReportSidebar {}
                }
            }
        }
    }
}
