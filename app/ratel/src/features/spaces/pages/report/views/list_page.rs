//! Reports list view — direct port of `assets/design/reports/reports-list.html`.
//! Class names and DOM order are preserved verbatim so the global CSS
//! section in `main.css` keyed on `/* === src/features/spaces/pages/report/views/list_page === */`
//! can be lifted from the mockup with minimal edits. Carousel JS isn't
//! ported yet — scroll-snap CSS + native scroll handle the basic flow.

use crate::features::spaces::pages::report::*;
use crate::*;

/// Filter state for the chip row. `All` shows every card, the other
/// two filter by `ReportStatus`. The create card is always visible
/// regardless of filter (mockup contract).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportFilter {
    All,
    Drafts,
    Published,
}

#[component]
pub fn ReportListPage() -> Element {
    let nav = use_navigator();
    let ctx = use_report_list_context();
    let reports = ctx.reports();
    let total = reports.len();
    let drafts = ctx.drafts().len();
    let published = ctx.published().len();

    let mut filter = use_signal(|| ReportFilter::All);

    rsx! {
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "reports-arena",
            // ── TOPBAR ────────────────────────────────────────
            div { class: "reports-topbar",
                div { class: "reports-topbar__brand",
                    button {
                        class: "reports-topbar__back",
                        "aria-label": "뒤로",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    span { class: "reports-topbar__divider" }
                    div { class: "reports-topbar__logo", "CAD" }
                    div { class: "reports-topbar__title-col",
                        span { class: "reports-topbar__title", "Reports" }
                        span { class: "reports-topbar__handle",
                            "Climate Action DAO · @climate_action"
                        }
                    }
                }
                div { class: "reports-topbar__actions",
                    button {
                        class: "reports-topbar__btn reports-topbar__btn--primary",
                        "aria-label": "새 보고서",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                        span { "New Report" }
                    }
                }
            }

            // ── SECTION LABEL ────────────────────────────────
            div { class: "section-label",
                span { class: "section-label__dash" }
                span { class: "section-label__title", strong { "보고서" } }
                span { class: "section-label__dash" }
            }
            div { class: "section-stats",
                span { strong { "{total}" } "Total" }
                span { strong { "{drafts}" } "Drafts" }
                span { strong { "{published}" } "Published" }
            }

            // ── FILTERS ──────────────────────────────────────
            div { class: "reports-filters",
                button {
                    class: "reports-chip",
                    "data-filter": "all",
                    "aria-selected": filter() == ReportFilter::All,
                    onclick: move |_| filter.set(ReportFilter::All),
                    "All"
                }
                button {
                    class: "reports-chip",
                    "data-filter": "drafts",
                    "aria-selected": filter() == ReportFilter::Drafts,
                    onclick: move |_| filter.set(ReportFilter::Drafts),
                    "Drafts"
                }
                button {
                    class: "reports-chip",
                    "data-filter": "published",
                    "aria-selected": filter() == ReportFilter::Published,
                    onclick: move |_| filter.set(ReportFilter::Published),
                    "Published"
                }
            }

            // ── CAROUSEL ─────────────────────────────────────
            div { class: "carousel-wrapper",
                div { class: "carousel-track", id: "carousel-track",
                    // Create card — always first.
                    ReportCreateCard {}

                    for (idx, item) in reports.iter().enumerate() {
                        ReportCard {
                            key: "{item.id}",
                            idx: (idx + 1) as i32,
                            item: item.clone(),
                            hidden: !matches_filter(filter(), item.status),
                        }
                    }
                }

                // Dots — populated client-side once carousel JS is ported.
                div { class: "carousel-dots", id: "carousel-dots" }
            }
        }
    }
}

#[component]
fn ReportCreateCard() -> Element {
    let ctx = use_report_list_context();
    let nav = use_navigator();
    rsx! {
        a {
            class: "report-card report-card--create",
            href: "#",
            "data-index": "0",
            "data-kind": "create",
            onclick: move |e| {
                e.prevent_default();
                nav.push(Route::ReportDetailPage {
                    space_id: ctx.space_id(),
                    report_id: "new".to_string(),
                });
            },
            div { class: "report-card__wave" }
            div { class: "report-card__create-icon",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.4",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "12", y1: "5", x2: "12", y2: "19" }
                    line { x1: "5", y1: "12", x2: "19", y2: "12" }
                }
            }
            div { class: "report-card__create-title", "새 보고서" }
            div { class: "report-card__create-sub",
                "빈 문서에서 시작 — "
                code { "/data:" }
                " 입력 시 analyze에서 만든 데이터로 차트 삽입"
            }
            span { class: "report-card__create-cta",
                "Create"
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "9 18 15 12 9 6" }
                }
            }
        }
    }
}

fn matches_filter(filter: ReportFilter, status: ReportStatus) -> bool {
    match filter {
        ReportFilter::All => true,
        ReportFilter::Drafts => status == ReportStatus::Draft,
        ReportFilter::Published => status == ReportStatus::Published,
    }
}

#[component]
fn ReportCard(idx: i32, item: ReportListItem, hidden: bool) -> Element {
    let ctx = use_report_list_context();
    let nav = use_navigator();
    let badge_class = match item.status {
        ReportStatus::Draft => "report-card__badge report-card__badge--draft",
        ReportStatus::Published => "report-card__badge report-card__badge--published",
    };
    let badge_label = match item.status {
        ReportStatus::Draft => "Draft",
        ReportStatus::Published => "Published",
    };
    let report_id = item.id.clone();

    rsx! {
        a {
            class: "report-card",
            href: "#",
            onclick: move |e| {
                e.prevent_default();
                nav.push(Route::ReportDetailPage {
                    space_id: ctx.space_id(),
                    report_id: report_id.clone(),
                });
            },
            "data-index": "{idx}",
            hidden,
            div { class: "report-card__wave" }
            div { class: "report-card__top",
                span { class: "{badge_class}", {status_icon(item.status)} "{badge_label}" }
                button { class: "report-card__menu-btn", "aria-label": "옵션",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "12", cy: "12", r: "1" }
                        circle { cx: "12", cy: "5", r: "1" }
                        circle { cx: "12", cy: "19", r: "1" }
                    }
                }
            }
            div { class: "report-card__identity",
                div { class: "report-card__icon", {source_icon(item.source)} }
                div { class: "report-card__id",
                    span { class: "report-card__category", "{item.category}" }
                    span { class: "report-card__title", "{item.title}" }
                }
            }
            div { class: "report-card__desc", "{item.description}" }
            div { class: "report-card__spacer" }
            div { class: "report-card__footer",
                span { class: "report-card__meta",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "12", cy: "12", r: "10" }
                        polyline { points: "12 6 12 12 16 14" }
                    }
                    "{item.relative_time}"
                }
                span { class: "report-card__cta",
                    "Open"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }
        }
    }
}

fn status_icon(status: ReportStatus) -> Element {
    match status {
        ReportStatus::Published => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                polyline { points: "22 4 12 14.01 9 11.01" }
            }
        },
        ReportStatus::Draft => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                path { d: "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" }
            }
        },
    }
}

fn source_icon(source: ReportSourceKind) -> Element {
    match source {
        ReportSourceKind::Action => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                polyline { points: "14 2 14 8 20 8" }
                line { x1: "9", y1: "13", x2: "15", y2: "13" }
            }
        },
        ReportSourceKind::Follow => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
            }
        },
        ReportSourceKind::Quiz => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
            }
        },
        ReportSourceKind::Poll => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M18 20V10" }
                path { d: "M12 20V4" }
                path { d: "M6 20v-6" }
            }
        },
    }
}
