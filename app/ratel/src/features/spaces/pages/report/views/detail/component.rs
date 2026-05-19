use super::data_picker::DataPicker;
use super::doc_canvas::DocCanvas;
use super::format_toolbar::FormatToolbar;
use super::outline::Outline;
use super::top_bar::TopBar;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Report detail page — installs `UseReportDetailContext` and arranges
/// the sub-components matching the approved mockup at
/// `assets/design/reports/reports-edit.html`. All client state
/// (picker open/closed, blocks, outline) lives in the context; each
/// sub-component reads its slice via `use_report_detail_context()`.
#[component]
pub fn ReportDetailPage(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Element {
    let _ctx = use_report_detail_context_provider(space_id, report_id)?;

    rsx! {
        div { class: "report-detail",
            TopBar {}
            div { class: "report-detail__grid",
                DocCanvas {}
                Outline {}
            }
            FormatToolbar {}
            DataPicker {}
        }
    }
}
