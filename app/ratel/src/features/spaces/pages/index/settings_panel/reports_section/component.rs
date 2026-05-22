//! Reports section of the space settings sidebar — surfaces every
//! published report under its own "Reports" group, sibling to the
//! Installed Apps + Available Apps lists. Members click `Download` to
//! jump to the report detail page, where the existing top-bar PDF
//! Download action opens the system print dialog.
//!
//! Hidden entirely while the report list is empty so the sidebar stays
//! tidy on spaces that haven't shipped a report yet.

use crate::features::spaces::pages::index::*;
use crate::features::spaces::pages::report::controllers::list_reports;
use crate::features::spaces::pages::report::types::{ReportListItem, ReportStatus};

#[component]
pub fn ReportsSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    let reports_loader = use_loader(move || async move {
        list_reports(space_id(), None, Some(ReportStatus::Published)).await
    })?;
    let reports: Vec<ReportListItem> = reports_loader().items;

    if reports.is_empty() {
        return rsx! {};
    }

    let count = reports.len();

    rsx! {
        section {
            // Reuse the `apps-section` class so all the
            // `--apps-surface-*` / `--apps-border` / `--apps-text` CSS
            // variables that `.app-row` and `.app-row-btn` consume are
            // defined in scope. Without it the rows render unstyled
            // because those vars are only declared under
            // `.apps-section`.
            class: "settings-section apps-section reports-section",
            "data-testid": "reports-section",
            div { class: "settings-section__sublabel",
                span { class: "settings-section__sublabel-text", "{tr.reports_section_label}" }
                span { class: "settings-section__sublabel-count", "{count}" }
            }
            for report in reports {
                ReportRow {
                    key: "{report.id}",
                    space_id,
                    report: report.clone(),
                }
            }
        }
    }
}

#[component]
fn ReportRow(space_id: ReadSignal<SpacePartition>, report: ReportListItem) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let nav = use_navigator();
    let report_id = report.id.clone();
    let title = report.title.clone();
    let description = report.description.clone();
    let download_testid = format!("download-report-{}", report.id);

    rsx! {
        div { class: "app-row",
            div { class: "app-row__icon app-row__icon--general",
                svg {
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.6",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                    polyline { points: "14 2 14 8 20 8" }
                    line {
                        x1: "16",
                        x2: "8",
                        y1: "13",
                        y2: "13",
                    }
                    line {
                        x1: "16",
                        x2: "8",
                        y1: "17",
                        y2: "17",
                    }
                    polyline { points: "10 9 9 9 8 9" }
                }
            }
            div { class: "app-row__info",
                div { class: "app-row__name-line",
                    span { class: "app-row__name", "{title}" }
                }
                div { class: "app-row__desc", "{description}" }
            }
            div { class: "app-row__action",
                button {
                    class: "app-row-btn",
                    "data-testid": "{download_testid}",
                    onclick: move |_| {
                        nav.push(crate::Route::ReportDetailPage {
                            space_id: space_id(),
                            report_id: report_id.clone(),
                        });
                    },
                    "{tr.report_download_btn}"
                }
            }
        }
    }
}
