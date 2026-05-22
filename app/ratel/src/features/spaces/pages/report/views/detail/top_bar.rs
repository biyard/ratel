use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::hooks::SaveStatus;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Trigger the browser's native print dialog, scoped to the report
/// body via the `@media print` styles in `main.css`. The user can pick
/// "Save as PDF" as the destination from the system print dialog —
/// no extra dependency needed, the printed HTML is the same content
/// the browser already renders so figures (charts / tables) survive
/// without rasterization.
fn trigger_pdf_print() {
    let _ = dioxus::document::eval(
        r#"
        // Use a tiny delay so any pending Dioxus re-render settles
        // before the print snapshot is taken. Without this, charts
        // freshly inserted via execCommand can flicker in the printed
        // output.
        setTimeout(function () { window.print(); }, 50);
        "#,
    );
}

/// Top chrome row — back arrow + breadcrumb + autosave + Share /
/// Export / Publish buttons. Consumes detail context for the report
/// title (breadcrumb) and the autosave state (mock for now).
#[component]
pub fn TopBar() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let nav = use_navigator();
    let ctx = use_report_detail_context();
    let autosave_label = match ctx.save_status() {
        SaveStatus::Idle | SaveStatus::Saved => tr.autosave_just_now.to_string(),
        SaveStatus::Unsaved => tr.autosave_unsaved.to_string(),
        SaveStatus::Saving => tr.autosave_saving.to_string(),
    };

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
                span { class: "report-detail__autosave", "{autosave_label}" }
            }
            div { class: "report-detail__topbar-right",
                button {
                    class: "report-detail__topbar-btn",
                    "aria-label": tr.pdf_download_btn,
                    onclick: move |_| trigger_pdf_print(),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                        polyline { points: "14 2 14 8 20 8" }
                        line {
                            x1: "9",
                            y1: "15",
                            x2: "15",
                            y2: "15",
                        }
                        line {
                            x1: "12",
                            y1: "12",
                            x2: "12",
                            y2: "18",
                        }
                        polyline { points: "10 16 12 18 14 16" }
                    }
                    span { "{tr.pdf_download_btn}" }
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
