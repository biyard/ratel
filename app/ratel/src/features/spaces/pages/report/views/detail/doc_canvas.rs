//! Doc-canvas surface — the editable body of the report. Composed of:
//!
//! 1. `cover` strip (eyebrow label)
//! 2. `<input>` title (autosaves on blur, Enter focuses the body)
//! 3. `<input>` subtitle (same UX as the title)
//! 4. The shared `Editor` component as the single rich-text body. The
//!    editor handles formatting / lists / alignment / link / image /
//!    YouTube / table / undo-redo / selection bubble / IME. Insertion
//!    of analyse-driven chart figures is wired through `on_insert_data`
//!    (data picker) and `on_slash` (`/data:…` autocomplete popup).
//!
//! The Editor's `on_content_change` mirrors the latest body HTML into
//! `body_html` and triggers `handle_save`.

use super::i18n::ReportDetailTranslate;
use crate::common::components::editor::{Editor as RichEditor, EditorSlashSignal};
use crate::features::spaces::pages::report::*;
use crate::*;

#[component]
pub fn DocCanvas() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();

    // Snapshot the body HTML once at first render — the Editor seeds its
    // content from this and never re-applies the prop (re-applying
    // `dangerous_inner_html` would clobber the caret on every keystroke).
    let initial_body = use_hook(|| ctx.initial_body_html());

    rsx! {
        // Per-page JS that wires figure-action click delegation (swap +
        // delete buttons embedded inside chart figures).
        document::Script { defer: true, src: asset!("./detail_actions.js") }

        div { class: "report-detail__doc",
            div { class: "report-detail__doc-inner",
                div { class: "report-detail__cover",
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
                    span { "{ctx.eyebrow()}" }
                }
                input {
                    class: "report-detail__title",
                    placeholder: tr.title_placeholder,
                    value: "{ctx.title_value()}",
                    oninput: move |e| {
                        ctx.title.set(e.value());
                        ctx.mark_unsaved();
                    },
                    onkeydown: move |e| {
                        if matches!(e.key(), Key::Enter) {
                            e.prevent_default();
                            focus_body_editor();
                        }
                    },
                }
                input {
                    class: "report-detail__subtitle",
                    placeholder: tr.subtitle_placeholder,
                    value: "{ctx.subtitle_value()}",
                    oninput: move |e| {
                        ctx.subtitle.set(e.value());
                        ctx.mark_unsaved();
                    },
                    onkeydown: move |e| {
                        if matches!(e.key(), Key::Enter) {
                            e.prevent_default();
                            focus_body_editor();
                        }
                    },
                }
                RichEditor {
                    class: "report-detail__editor",
                    content: initial_body,
                    editable: true,
                    placeholder: tr.body_placeholder.to_string(),
                    insert_data_label: tr.insert_data.to_string(),
                    on_content_change: move |html: String| {
                        ctx.body_html.set(html);
                        ctx.mark_unsaved();
                    },
                    on_insert_data: move |_| ctx.open_data_picker(),
                    on_slash: move |sig: EditorSlashSignal| {
                        ctx.handle_slash_signal(sig);
                    },
                }
            }
        }
    }
}

/// Place the caret inside the body editor's contenteditable. Used by
/// the title / subtitle Enter handler to "drop into" the body without
/// triggering a form submission or losing focus to <body>.
fn focus_body_editor() {
    let _ = dioxus::document::eval(
        r#"
        const el = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!el) return;
        el.focus();
        // Collapse caret to end of content so typing continues from
        // there rather than at character 0.
        const range = document.createRange();
        const sel = window.getSelection();
        range.selectNodeContents(el);
        range.collapse(false);
        sel.removeAllRanges();
        sel.addRange(range);
        "#,
    );
}
