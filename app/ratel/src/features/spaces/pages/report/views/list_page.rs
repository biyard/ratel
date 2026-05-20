use super::i18n::ReportListTranslate;
use crate::features::spaces::pages::report::*;
use crate::*;

#[component]
pub fn ReportListPage() -> Element {
    let tr: ReportListTranslate = use_translate();
    let nav = use_navigator();
    let mut ctx = use_report_list_context();

    rsx! {
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "reports-arena",
            // ── TOPBAR ────────────────────────────────────────
            div { class: "reports-topbar",
                div { class: "reports-topbar__brand",
                    button {
                        class: "reports-topbar__back",
                        "aria-label": "{tr.back_aria}",
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
                    div { class: "reports-topbar__logo", "{tr.workspace_logo}" }
                    div { class: "reports-topbar__title-col",
                        span { class: "reports-topbar__title", "{tr.workspace_title}" }
                    }
                }
                div { class: "reports-topbar__actions",
                    button {
                        class: "reports-topbar__btn reports-topbar__btn--primary",
                        "aria-label": "{tr.new_report_aria}",
                        "aria-busy": ctx.handle_create.pending(),
                        disabled: ctx.handle_create.pending(),
                        onclick: move |_| {
                            if !ctx.handle_create.pending() {
                                ctx.handle_create.call();
                            }
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
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
                        span { "{tr.new_report_btn}" }
                    }
                }
            }

            // ── SECTION LABEL ────────────────────────────────
            div { class: "section-label",
                span { class: "section-label__dash" }
                span { class: "section-label__title",
                    strong { "{tr.section_title}" }
                }
                span { class: "section-label__dash" }
            }
            div { class: "section-stats",
                span {
                    strong { "{ctx.total_count()}" }
                    "{tr.stat_total}"
                }
                span {
                    strong { "{ctx.drafts_count()}" }
                    "{tr.stat_drafts}"
                }
                span {
                    strong { "{ctx.published_count()}" }
                    "{tr.stat_published}"
                }
            }

            // ── FILTERS ──────────────────────────────────────
            // Each chip mutates `ctx.filter`; the context's
            // `filtered_reports` loader subscribes to that signal in
            // its outer closure, so a click immediately fires a fresh
            // `?status=...` request to the server.
            div { class: "reports-filters",
                button {
                    class: "reports-chip",
                    "data-filter": "all",
                    "aria-selected": ctx.filter_value() == ReportFilter::All,
                    onclick: move |_| ctx.set_filter(ReportFilter::All),
                    "{tr.filter_all}"
                }
                button {
                    class: "reports-chip",
                    "data-filter": "drafts",
                    "aria-selected": ctx.filter_value() == ReportFilter::Drafts,
                    onclick: move |_| ctx.set_filter(ReportFilter::Drafts),
                    "{tr.filter_drafts}"
                }
                button {
                    class: "reports-chip",
                    "data-filter": "published",
                    "aria-selected": ctx.filter_value() == ReportFilter::Published,
                    onclick: move |_| ctx.set_filter(ReportFilter::Published),
                    "{tr.filter_published}"
                }
            }

            // ── CAROUSEL ─────────────────────────────────────
            // `ctx.items()` already returns the server-filtered slice
            // for the current chip selection — no `hidden` toggling
            // needed here.
            div { class: "carousel-wrapper",
                div { class: "carousel-track", id: "carousel-track",
                    // Create card — always first.
                    ReportCreateCard {}

                    for (idx , item) in ctx.items().into_iter().enumerate() {
                        ReportCard {
                            key: "{item.id}",
                            idx: (idx + 1) as i32,
                            item,
                        }
                    }
                }

                // Dots — populated client-side once carousel JS is ported.
                div { class: "carousel-dots", id: "carousel-dots" }
            }

            // Renders only when `ctx.delete_target` is Some.
            DeleteConfirmModal {}
        }
    }
}

/// Modal scrim + panel rendered only when `ctx.delete_target` is Some.
/// Reads the queued report's title for the warning copy, wires the
/// 취소 / 삭제 buttons to `cancel_delete` / `handle_delete.call()`, and
/// disables both while the DELETE is in flight to prevent double-fire.
#[component]
fn DeleteConfirmModal() -> Element {
    let tr: ReportListTranslate = use_translate();
    let mut ctx = use_report_list_context();
    let target = ctx.delete_target_value();
    let Some(target) = target else {
        return rsx! {};
    };
    let mut handle_delete = ctx.handle_delete;
    let pending = handle_delete.pending();

    rsx! {
        div {
            class: "confirm-modal",
            role: "dialog",
            "aria-modal": "true",
            onclick: move |_| {
                // Scrim click cancels — matches the mockup's
                // backdrop-click-to-dismiss behavior.
                if !pending {
                    ctx.cancel_delete();
                }
            },
            div {
                class: "confirm-modal__panel",
                onclick: move |e| e.stop_propagation(),
                span { class: "confirm-modal__eyebrow",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 9v4" }
                        path { d: "M12 17h.01" }
                        path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                    }
                    "{tr.modal_eyebrow}"
                }
                div { class: "confirm-modal__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.8",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "3 6 5 6 21 6" }
                        path { d: "M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
                        path { d: "M10 11v6M14 11v6" }
                        path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                    }
                }
                div { class: "confirm-modal__title", "{tr.modal_title}" }
                div { class: "confirm-modal__body",
                    strong { "{target.title}" }
                    "{tr.modal_body}"
                    span { class: "confirm-modal__note", "{tr.modal_note}" }
                }
                div { class: "confirm-modal__actions",
                    button {
                        class: "confirm-modal__btn confirm-modal__btn--ghost",
                        r#type: "button",
                        disabled: pending,
                        onclick: move |_| ctx.cancel_delete(),
                        "{tr.modal_cancel}"
                    }
                    button {
                        class: "confirm-modal__btn confirm-modal__btn--danger",
                        r#type: "button",
                        disabled: pending,
                        onclick: move |_| {
                            if !pending {
                                handle_delete.call();
                            }
                        },
                        "{tr.modal_confirm}"
                    }
                }
            }
        }
    }
}

#[component]
fn ReportCreateCard() -> Element {
    let tr: ReportListTranslate = use_translate();
    let UseReportListContext {
        mut handle_create, ..
    } = use_report_list_context();
    rsx! {
        a {
            class: "report-card report-card--create",
            href: "#",
            "data-index": "0",
            "data-kind": "create",
            "aria-busy": handle_create.pending(),
            onclick: move |e| {
                e.prevent_default();
                if !handle_create.pending() {
                    handle_create.call();
                }
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
            div { class: "report-card__create-title", "{tr.create_title}" }
            div { class: "report-card__create-sub",
                "{tr.create_sub_prefix}"
                code { "/data:" }
                "{tr.create_sub_suffix}"
            }
            span { class: "report-card__create-cta",
                "{tr.create_cta}"
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

#[component]
fn ReportCard(idx: i32, item: ReportListItem) -> Element {
    let tr: ReportListTranslate = use_translate();
    let mut ctx = use_report_list_context();
    let nav = use_navigator();
    let item_for_navigate = item.clone();
    let item_for_menu = item.clone();
    let item_for_delete = item.clone();
    rsx! {
        // Card root is a plain `<div>` (not `<a href>`) so the inner
        // menu/menu-item buttons don't fight anchor-default behavior.
        // `e.stop_propagation()` on the children is enough — we drive
        // navigation manually from this div's onclick.
        div {
            class: "report-card",
            role: "link",
            tabindex: 0,
            onclick: move |_e| {
                // The menu / menu-item children call `stop_propagation`
                // before this bubbles, so reaching here means the user
                // really clicked the card body. Defensive gates:
                // 1. menu is open → user is interacting with the menu
                // 2. delete confirm is open → the just-fired menu-item
                //    click might still race up here; don't navigate.
                if ctx.is_menu_open_for(&item_for_navigate.id) {
                    return;
                }
                if ctx.delete_target_value().is_some() {
                    return;
                }
                nav.push(Route::ReportDetailPage {
                    space_id: ctx.space_id(),
                    report_id: item_for_navigate.id.clone(),
                });
            },
            "data-index": "{idx}",
            div { class: "report-card__wave" }
            div { class: "report-card__top",
                span {
                    class: match item.status {
                        ReportStatus::Draft => "report-card__badge report-card__badge--draft",
                        ReportStatus::Published => "report-card__badge report-card__badge--published",
                    },
                    {status_icon(item.status)}
                    {
                        match item.status {
                            ReportStatus::Draft => tr.status_draft,
                            ReportStatus::Published => tr.status_published,
                        }
                    }
                }
                // Wrapper carries `position: relative` so the dropdown
                // (`position: absolute`) anchors to the menu button.
                div { class: "report-card__menu-wrap",
                    button {
                        class: "report-card__menu-btn",
                        "aria-label": "{tr.card_menu_aria}",
                        "aria-haspopup": "menu",
                        "aria-expanded": ctx.is_menu_open_for(&item_for_menu.id),
                        onclick: move |e| {
                            // Don't bubble — the card-level `onclick`
                            // would navigate into the report editor.
                            e.stop_propagation();
                            e.prevent_default();
                            ctx.toggle_menu(&item_for_menu.id);
                        },
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
                    if ctx.is_menu_open_for(&item.id) {
                        div { class: "card-menu", role: "menu",
                            button {
                                class: "card-menu__item card-menu__item--danger",
                                r#type: "button",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    e.prevent_default();
                                    ctx.request_delete(&item_for_delete);
                                },
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    polyline { points: "3 6 5 6 21 6" }
                                    path { d: "M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
                                    path { d: "M10 11v6M14 11v6" }
                                }
                                "{tr.menu_delete}"
                            }
                        }
                    }
                }
            }
            div { class: "report-card__identity",
                div { class: "report-card__icon", {report_card_icon()} }
                div { class: "report-card__id",
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
                    "{format_relative_time(item.created_at, &tr)}"
                }
                span { class: "report-card__cta",
                    "{tr.card_open_cta}"
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

/// Client-side "Xm ago" / "방금 전" label. Server returns the raw
/// timestamp in unix milliseconds so the formatting honors the
/// browser's clock and locale rather than the server's. Bucket
/// boundaries match `common::utils::time::time_ago`, but the suffix
/// strings come from the i18n table so KR/EN render correctly.
fn format_relative_time(timestamp_millis: i64, tr: &ReportListTranslate) -> String {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_ms = (now - timestamp_millis).max(0);
    if diff_ms < 60_000 {
        return tr.time_just_now.to_string();
    }
    let mins = diff_ms / 60_000;
    if mins < 60 {
        return format!("{mins}{}", tr.time_minutes_suffix);
    }
    let hours = mins / 60;
    if hours < 24 {
        return format!("{hours}{}", tr.time_hours_suffix);
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{days}{}", tr.time_days_suffix);
    }
    let weeks = days / 7;
    if weeks < 5 {
        return format!("{weeks}{}", tr.time_weeks_suffix);
    }
    let months = days / 30;
    if months < 12 {
        return format!("{months}{}", tr.time_months_suffix);
    }
    let years = days / 365;
    format!("{years}{}", tr.time_years_suffix)
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

/// Single document-style icon used on every card. Reports are
/// source-agnostic (they can mix poll/quiz/discussion/follow content
/// in one document), so a per-source icon dispatcher would be
/// misleading.
fn report_card_icon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
            polyline { points: "14 2 14 8 20 8" }
            line {
                x1: "9",
                y1: "13",
                x2: "15",
                y2: "13",
            }
        }
    }
}
