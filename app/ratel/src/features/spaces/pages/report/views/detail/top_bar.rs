use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Top chrome row — back arrow + breadcrumb + autosave + Share /
/// Export / Publish buttons. Consumes detail context for the report
/// title (breadcrumb) and the autosave state (mock for now).
#[component]
pub fn TopBar() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let nav = use_navigator();
    let ctx = use_report_detail_context();

    rsx! {
        div { class: "report-detail__topbar",
            div { class: "report-detail__topbar-left",
                button {
                    class: "report-detail__back",
                    "aria-label": tr.back_aria,
                    onclick: move |_| nav.go_back(),
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
                span { class: "report-detail__breadcrumb",
                    span { class: "report-detail__breadcrumb-root", "{tr.breadcrumb_root}" }
                    span { class: "report-detail__breadcrumb-sep", "{tr.breadcrumb_separator}" }
                    span { class: "report-detail__breadcrumb-current", "{ctx.initial_title()}" }
                }
                span { class: "report-detail__autosave", "{tr.autosave_just_now}" }
            }
            div { class: "report-detail__topbar-right",
                button { class: "report-detail__topbar-btn",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "18", cy: "5", r: "3" }
                        circle { cx: "6", cy: "12", r: "3" }
                        circle { cx: "18", cy: "19", r: "3" }
                        line {
                            x1: "8.59",
                            y1: "13.51",
                            x2: "15.42",
                            y2: "17.49",
                        }
                        line {
                            x1: "15.41",
                            y1: "6.51",
                            x2: "8.59",
                            y2: "10.49",
                        }
                    }
                    span { "{tr.share_btn}" }
                }
                button { class: "report-detail__topbar-btn",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line {
                            x1: "12",
                            y1: "15",
                            x2: "12",
                            y2: "3",
                        }
                    }
                    span { "{tr.export_btn}" }
                }
                button { class: "report-detail__topbar-btn report-detail__topbar-btn--primary",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                        polyline { points: "22 4 12 14.01 9 11.01" }
                    }
                    span { "{tr.publish_btn}" }
                }
            }
        }
    }
}
