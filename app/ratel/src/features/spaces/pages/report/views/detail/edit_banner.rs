use super::i18n::ReportDetailTranslate;
use crate::*;

/// Purple "Document Editor" eyebrow banner above the doc canvas.
/// Static — text only, no interactive controls.
#[component]
pub fn EditBanner() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    rsx! {
        div { class: "report-detail__banner",
            div { class: "report-detail__banner-inner",
                div { class: "report-detail__banner-icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                        polyline { points: "14 2 14 8 20 8" }
                    }
                }
                div { class: "report-detail__banner-body",
                    div { class: "report-detail__banner-eyebrow", "{tr.banner_eyebrow}" }
                    div { class: "report-detail__banner-title", "{tr.banner_title}" }
                    div { class: "report-detail__banner-text", "{tr.banner_text}" }
                }
            }
        }
    }
}
