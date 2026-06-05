//! Publish confirmation modal — arena glass styling matching the list
//! page's `DeleteConfirmModal`, but in the "purple aurora" (publish)
//! tone instead of red (destructive). Rendered INLINE inside the
//! `.report-detail` container so it inherits the page-scoped
//! `.confirm-modal` CSS rules.
//!
//! Visibility is controlled by `UseReportDetailContext.publish_modal_open`
//! so any sibling component can flip the modal open/closed without
//! threading a separate prop.

use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::*;
use crate::*;

#[component]
pub fn PublishConfirmModal() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();

    if !ctx.is_publish_modal_open() {
        return rsx! {};
    }

    let mut handle_publish = ctx.handle_publish;
    let pending = handle_publish.pending();
    // Publish is a one-shot — once a report is Published it can't be
    // re-published from this surface (the publish button itself is
    // hidden), so the modal only ever shows the first-publish copy.
    let body = tr.publish_modal_body.to_string();

    rsx! {
        div {
            class: "confirm-modal",
            role: "dialog",
            "aria-modal": "true",
            onclick: move |_| {
                if !pending {
                    ctx.close_publish_modal();
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
                        path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                        polyline { points: "22 4 12 14.01 9 11.01" }
                    }
                    "{tr.publish_modal_title}"
                }
                div { class: "confirm-modal__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.8",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
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
                    }
                }
                div { class: "confirm-modal__title", "{tr.publish_modal_title}" }
                div { class: "confirm-modal__body", "{body}" }
                div { class: "confirm-modal__actions",
                    button {
                        class: "confirm-modal__btn confirm-modal__btn--ghost",
                        r#type: "button",
                        disabled: pending,
                        onclick: move |_| ctx.close_publish_modal(),
                        "{tr.publish_modal_cancel}"
                    }
                    button {
                        class: "confirm-modal__btn confirm-modal__btn--primary",
                        "data-testid": "publish-confirm",
                        r#type: "button",
                        disabled: pending,
                        onclick: move |_| {
                            if !pending {
                                handle_publish.call();
                                ctx.close_publish_modal();
                            }
                        },
                        if pending {
                            "{tr.publishing_label}"
                        } else {
                            "{tr.publish_modal_confirm}"
                        }
                    }
                }
            }
        }
    }
}
