use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Bottom-floating format toolbar — matches the Slack-style pill in
/// the mockup. Format actions fire `document.execCommand` against the
/// currently focused contenteditable block via small inline JS calls;
/// the primary "Insert data" button opens the picker through context.
#[component]
pub fn FormatToolbar() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();
    rsx! {
        div { class: "report-detail__fmt-bar", role: "toolbar",
            FmtBtn { cmd: "bold".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.4",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                    path { d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" }
                }
            }
            FmtBtn { cmd: "italic".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "19", y1: "4", x2: "10", y2: "4" }
                    line { x1: "14", y1: "20", x2: "5", y2: "20" }
                    line { x1: "15", y1: "4", x2: "9", y2: "20" }
                }
            }
            FmtBtn { cmd: "underline".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" }
                    line { x1: "4", y1: "21", x2: "20", y2: "21" }
                }
            }
            FmtBtn { cmd: "strikeThrough".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M16 4H9a3 3 0 0 0-2.83 4" }
                    path { d: "M14 12a4 4 0 0 1 0 8H6" }
                    line { x1: "4", y1: "12", x2: "20", y2: "12" }
                }
            }
            span { class: "report-detail__fmt-sep" }
            FmtBtn { cmd: "insertUnorderedList".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "8", y1: "6", x2: "21", y2: "6" }
                    line { x1: "8", y1: "12", x2: "21", y2: "12" }
                    line { x1: "8", y1: "18", x2: "21", y2: "18" }
                    line { x1: "3", y1: "6", x2: "3.01", y2: "6" }
                    line { x1: "3", y1: "12", x2: "3.01", y2: "12" }
                    line { x1: "3", y1: "18", x2: "3.01", y2: "18" }
                }
            }
            FmtBtn { cmd: "insertOrderedList".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line { x1: "10", y1: "6", x2: "21", y2: "6" }
                    line { x1: "10", y1: "12", x2: "21", y2: "12" }
                    line { x1: "10", y1: "18", x2: "21", y2: "18" }
                    path { d: "M4 6h1v4" }
                    path { d: "M4 10h2" }
                    path { d: "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" }
                }
            }
            FmtBtn { cmd: "createLink".to_string(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                    path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                }
            }
            span { class: "report-detail__fmt-sep" }
            button {
                class: "report-detail__fmt-btn report-detail__fmt-btn--insert",
                r#type: "button",
                onclick: move |_| ctx.open_data_picker(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "22 12 18 12 15 21 9 3 6 12 2 12" }
                }
                span { "{tr.insert_data}" }
            }
        }
    }
}

/// Generic format command button. `cmd` is passed to a tiny inline JS
/// snippet that runs `document.execCommand(cmd)` on the current
/// selection — same approach the shared Editor uses.
#[component]
fn FmtBtn(cmd: String, children: Element) -> Element {
    let js = format!(
        "document.execCommand('{}', false, null); event.preventDefault();",
        cmd.replace('\'', "")
    );
    rsx! {
        button {
            class: "report-detail__fmt-btn",
            r#type: "button",
            "data-cmd": "{cmd}",
            "onmousedown": "{js}",
            {children}
        }
    }
}
