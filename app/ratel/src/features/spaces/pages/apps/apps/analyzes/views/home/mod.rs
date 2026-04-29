use super::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::providers::use_space_context;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

/// Public entrypoint. Creators get the carousel; everyone else gets a
/// minimal "no access" splash. Reads the *real* role from
/// `SpaceContextProvider`, not the derived `current_role` (which
/// flips Creator → Participant once a space is Ongoing) — analyze is
/// always a creator-only surface.
#[component]
pub fn SpaceAnalyzesAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut ctx = use_space_context();
    let real_role = ctx.role();

    if real_role != SpaceUserRole::Creator {
        return rsx! {
            ViewerEmpty {}
        };
    }

    rsx! {
        AnalyzesListArena { space_id }
    }
}

/// LIST arena. Dioxus renders the markup; `script.js` owns every
/// piece of carousel UX (native horizontal scroll-snap, prev/next
/// buttons, dot navigation, ArrowLeft/ArrowRight keyboard, and the
/// `.is-active` highlight that follows the centered card). This
/// mirrors the `pages/index/action_dashboard` carousel — same scroll
/// container, same MutationObserver bind-once boot, same JS-owned
/// active-class toggling. Card click handlers stay in Dioxus so route
/// pushes go through `Route::*` enum variants, not string URLs.
#[component]
fn AnalyzesListArena(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    let ctrl = use_analyze_reports(space_id)?;
    let reports = ctrl.reports.read().clone().items;

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();

    let count_label = format!("{} {}", reports.len(), tr.list_count_unit);

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "analyze-arena",
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
                        span { class: "topbar-title", id: "arena-title", "{tr.arena_topbar_title}" }
                    }
                }

                // ── Body ────────────────────────────────────
                div { class: "split", "data-mode": "list",
                    main { class: "main",
                        section {
                            class: "analyze-builder",
                            id: "analyze-builder",
                            "data-mode": "list",

                            div { class: "builder-list", "data-state": "list",
                                div { class: "builder-list-head",
                                    h2 { "{tr.list_heading}" }
                                    span {
                                        class: "builder-list-head__count",
                                        id: "report-count",
                                        "data-testid": "report-count",
                                        "{count_label}"
                                    }
                                }
                                p { class: "builder-hint", "{tr.list_hint}" }

                                // Carousel — Dioxus renders structure
                                // only; script.js handles scroll-snap,
                                // prev/next/dot clicks, and keyboard.
                                div { class: "report-carousel",
                                    button {
                                        r#type: "button",
                                        class: "report-carousel__arrow report-carousel__arrow--prev",
                                        id: "report-prev",
                                        "data-testid": "report-prev",
                                        "aria-label": "{tr.arrow_prev_label}",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2.4",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            polyline { points: "15 18 9 12 15 6" }
                                        }
                                    }

                                    div {
                                        class: "report-carousel__viewport",
                                        id: "report-viewport",
                                        div {
                                            class: "report-carousel__track",
                                            id: "report-track",
                                            // Saved-report slides
                                            for rep in reports.iter() {
                                                {
                                                    let report_id = rep.id.clone();
                                                    let status = rep.status;
                                                    let mut toast = use_toast();
                                                    let pending_msg = tr.list_card_pending_toast.to_string();
                                                    rsx! {
                                                        div {
                                                            key: "{rep.id}",
                                                            class: "report-carousel__slide",
                                                            "data-card-type": "saved",
                                                            SavedReportCard {
                                                                report: rep.clone(),
                                                                onclick: move |_| {
                                                                    if matches!(status, AnalyzeReportStatus::Finish) {
                                                                        nav.push(Route::SpaceAnalyzeReportPage {
                                                                            space_id: space_id(),
                                                                            report_id: report_id.clone(),
                                                                        });
                                                                    } else {
                                                                        toast.info(pending_msg.clone());
                                                                    }
                                                                },
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // "+ new" slide — always last
                                            div {
                                                key: "plus",
                                                class: "report-carousel__slide",
                                                "data-card-type": "new",
                                                NewReportCard {
                                                    onclick: move |_| {
                                                        nav.push(Route::SpaceAnalyzeCreatePage {
                                                            space_id: space_id(),
                                                        });
                                                    },
                                                }
                                            }
                                        }
                                    }

                                    button {
                                        r#type: "button",
                                        class: "report-carousel__arrow report-carousel__arrow--next",
                                        id: "report-next",
                                        "data-testid": "report-next",
                                        "aria-label": "{tr.arrow_next_label}",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2.4",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            polyline { points: "9 18 15 12 9 6" }
                                        }
                                    }
                                }

                                // Dots — one per slide. JS owns
                                // click handlers and the .active class.
                                div {
                                    class: "report-carousel__dots",
                                    id: "report-dots",
                                    for rep in reports.iter() {
                                        button {
                                            key: "dot-{rep.id}",
                                            r#type: "button",
                                            class: "report-carousel__dot",
                                            "data-testid": "report-dot-{rep.id}",
                                            "aria-label": "{tr.dot_report_label_prefix}",
                                        }
                                    }
                                    button {
                                        key: "dot-plus",
                                        r#type: "button",
                                        class: "report-carousel__dot",
                                        "data-testid": "report-dot-new",
                                        "aria-label": "{tr.dot_new_label}",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SavedReportCard(report: AnalyzeReport, onclick: EventHandler<()>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let status = report.status;
    let status_label = match status {
        AnalyzeReportStatus::Finish => tr.status_finish.to_string(),
        AnalyzeReportStatus::InProgress => tr.status_in_progress.to_string(),
        AnalyzeReportStatus::Failed => tr.status_failed.to_string(),
    };
    let status_attr = status.as_str();

    rsx! {
        button {
            r#type: "button",
            class: "report-card-large report-card-large--saved",
            "data-report-id": "{report.id}",
            "data-status": "{status_attr}",
            "data-testid": "report-card-{report.id}",
            onclick: move |_| onclick.call(()),

            // Status badge — top-left
            span {
                class: "report-card-large__status",
                "data-status": "{status_attr}",
                if matches!(status, AnalyzeReportStatus::InProgress) {
                    span { class: "report-card-large__status-dot" }
                } else {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2.4",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                }
                "{status_label}"
            }

            // Saved-card icon — cyan analyze chip
            span { class: "report-card-large__icon report-card-large__icon--saved",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2.4",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    path { d: "M3 3v18h18" }
                    path { d: "M7 14l4-4 4 4 5-5" }
                }
            }
            span { class: "report-card-large__title", "{report.name}" }

            // Filter chips — bottom of card
            if report.filters.is_empty() {
                div { class: "report-card-large__chips",
                    span { class: "report-card-large__chips-empty", "{tr.chips_empty}" }
                }
            } else {
                div { class: "report-card-large__chips",
                    for f in report.filters.iter() {
                        {
                            let src = f.source.as_str();
                            let label_upper = f.source_label.to_uppercase();
                            rsx! {
                                span {
                                    key: "chip-{f.item_id}-{f.option_id}",
                                    class: "report-card-large__chip",
                                    "data-source": "{src}",
                                    span { class: "report-card-large__chip-source", "{label_upper}" }
                                    span { class: "report-card-large__chip-text", "{f.label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NewReportCard(onclick: EventHandler<()>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    rsx! {
        button {
            r#type: "button",
            class: "report-card-large report-card-large--new",
            "data-testid": "report-card-new",
            onclick: move |_| onclick.call(()),

            span { class: "report-card-large__icon",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "1.8",
                    "stroke-linecap": "round",
                    line {
                        x1: "12",
                        y1: "5",
                        x2: "12",
                        y2: "19",
                    }
                    line {
                        x1: "5",
                        y1: "12",
                        x2: "19",
                        y2: "12",
                    }
                }
            }
            span { class: "report-card-large__title", "{tr.new_analysis_title}" }
            span { class: "report-card-large__desc", "{tr.new_analysis_desc}" }
        }
    }
}

#[component]
fn ViewerEmpty() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "analyze-arena",
            div { class: "arena-viewer",
                span { class: "arena-viewer__title", "No access" }
            }
        }
    }
}
