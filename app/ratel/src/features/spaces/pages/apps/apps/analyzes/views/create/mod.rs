//! Phase-2: Analyze CREATE wizard.
//!
//! Two-step wizard (CREATE → PREVIEW) ported 1:1 from
//! `assets/design/analyze-create-arena.html`. Class names match the
//! mockup verbatim — `.cross-filter`, `.cf-action-tile`, `.cf-options-list`,
//! `.cf-sunji`, `.cf-question`, `.cf-keyword-input`, `.preview-chips`,
//! `.preview-stats`, `.preview-records`, `.prv-table`, `.prv-pager-btn`,
//! `.builder-actions`, `.builder-create`, etc.
//!
//! State lives in `UseAnalyzeCreate` — every signal the JS in the mockup
//! tracked is now a Dioxus signal. Components destructure from the hook
//! and never call server `_handler` functions directly. Phase 2 is a
//! pure visual port — no controllers, no DynamoDB. Confirm just navigates
//! to the existing detail mock report.

mod cf_sunji;
mod cross_filter;
mod footer_create;
mod footer_preview;
mod preview_card;
mod preview_records;

pub use cf_sunji::*;
pub use cross_filter::*;
pub use footer_create::*;
pub use footer_preview::*;
pub use preview_card::*;
pub use preview_records::*;

use super::*;
use crate::features::spaces::space_common::hooks::use_space;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

/// Public entrypoint for the Analyze CREATE wizard. Provides the
/// `UseAnalyzeCreate` controller hook to the subtree, then renders the
/// arena shell + the appropriate step (CREATE or PREVIEW).
#[component]
pub fn SpaceAnalyzeCreatePage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    let ctrl = use_analyze_create()?;
    let mode = ctrl.mode.read().clone();
    let mode_attr = mode.as_str();

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "analyze-arena analyze-arena--create",
            div { class: "arena",
                // ── Topbar ───────────────────────────────────
                header { class: "arena-topbar", role: "banner",
                    div { class: "arena-topbar__left",
                        button {
                            r#type: "button",
                            class: "back-btn",
                            "aria-label": "Back",
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
                            "{tr.create_topbar_title}"
                        }
                    }
                }

                // ── Body ─────────────────────────────────────
                div { class: "split", "data-mode": "{mode_attr}",
                    main { class: "main",
                        section {
                            class: "analyze-builder",
                            id: "analyze-builder",
                            "data-mode": "{mode_attr}",

                            match mode {
                                CreateMode::Create => rsx! {
                                    div { class: "builder-create", "data-state": "create",
                                        CrossFilterCard {}
                                        CfSunjiCard {}
                                    }
                                },
                                CreateMode::Preview => rsx! {
                                    PreviewCard {}
                                },
                            }
                        }
                    }
                }

                // ── Page-level fixed footers ────────────────
                FooterCreate { space_id }
                FooterPreview { space_id }
            }
        }
    }
}
