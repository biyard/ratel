use super::data_picker::DataPicker;
use super::doc_canvas::DocCanvas;
use super::outline::Outline;
use super::publish_modal::PublishConfirmModal;
use super::slash_popup::SlashPopup;
use super::top_bar::TopBar;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Report detail page — installs `UseReportDetailContext` and arranges
/// the sub-components. The page also owns the command bridge `<input>`
/// that the per-page JS uses to push figure-button click events
/// (swap / delete) back into Rust. The picker and slash popup are
/// always rendered; they show themselves based on context signals.
#[component]
pub fn ReportDetailPage(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Element {
    let mut ctx = use_report_detail_context_provider(space_id, report_id)?;

    // Access enforcement lives in `get_report` — non-admins requesting
    // an unpublished draft get a server-side `NoPermission` error and
    // the provider's loader surfaces that. The client only branches the
    // rendering mode (edit vs. viewer chrome) based on the user's role.
    let can_edit = ctx.can_edit_value();
    let root_class = if can_edit {
        "report-detail"
    } else {
        "report-detail report-detail--viewer"
    };

    rsx! {
        div { class: "{root_class}",
            // Hidden bridge: JS writes "swap:<chart-id>" / "delete:<id>"
            // here, Dioxus dispatches to the context.
            input {
                class: "report-detail__cmd-bridge",
                r#type: "text",
                hidden: true,
                "aria-hidden": "true",
                tabindex: "-1",
                oninput: move |evt| {
                    let raw = evt.value();
                    let Some((act, arg)) = raw.split_once(':') else {
                        return;
                    };
                    match act {
                        "swap" => ctx.open_chart_swap(arg),
                        "delete" => ctx.delete_chart(arg),
                        "slash-down" => ctx.move_slash_selection(1),
                        "slash-up" => ctx.move_slash_selection(-1),
                        "slash-enter" => ctx.apply_slash_selected(),
                        "slash-close" => ctx.close_slash(),
                        _ => {}
                    }
                },
            }

            TopBar {}
            div { class: "report-detail__grid",
                DocCanvas {}
                Outline {}
            }
            DataPicker {}
            SlashPopup {}
            PublishConfirmModal {}
        }
    }
}
