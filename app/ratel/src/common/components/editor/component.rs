use crate::common::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditorProps {
    #[props(default)]
    pub content: String,
    #[props(default = true)]
    pub editable: bool,
    #[props(default = "Type here...".to_string())]
    pub placeholder: String,
    #[props(default)]
    pub on_content_change: Option<EventHandler<String>>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Editor(props: EditorProps) -> Element {
    // Snapshot the initial HTML once at mount and never re-apply it on
    // subsequent renders. Re-applying `dangerous_inner_html` on every
    // render destroys the user's caret position — most visibly during
    // Korean (or any IME) composition where every keystroke triggers
    // `on_content_change` → `content.set` → re-render → innerHTML
    // overwrite → cursor jumps to start. Use `key=` on the parent to
    // force a remount when the editor needs to be reset.
    let content = use_hook(|| props.content.clone());
    let placeholder = props.placeholder.clone();
    let is_editable = props.editable;
    let editable = if is_editable { "true" } else { "false" };
    let extra_class = props.class.clone();

    rsx! {
        document::Script { defer: true, src: asset!("./script.js") }

        div {
            class: "ratel-editor {extra_class}",
            "data-bound": "false",
            "data-placeholder": "{placeholder}",
            "data-editable": "{editable}",
            // Hidden input bridge — JS writes the latest editor HTML into
            // its `value` and dispatches a synthetic `input` event. Going
            // through a real <input> instead of a div CustomEvent is the
            // only reliable way to get the payload across Dioxus's mobile
            // IPC bridge (the WebView serializes form values, not custom
            // event detail fields).
            input {
                class: "re-bridge",
                r#type: "text",
                hidden: true,
                "aria-hidden": "true",
                tabindex: "-1",
                oninput: move |evt| {
                    if let Some(handler) = &props.on_content_change {
                        handler.call(evt.value());
                    }
                },
            }
            if is_editable {
                div {
                    aria_label: "Editor toolbar",
                    class: "re-toolbar",
                    role: "toolbar",
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Undo",
                            class: "re-tb-btn",
                            "data-cmd": "undo",
                            "data-tip": "Undo",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M3 7v6h6" }
                                path { d: "M21 17a9 9 0 0 0-15-6.7L3 13" }
                            }
                        }
                        button {
                            aria_label: "Redo",
                            class: "re-tb-btn",
                            "data-cmd": "redo",
                            "data-tip": "Redo",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M21 7v6h-6" }
                                path { d: "M3 17a9 9 0 0 1 15-6.7L21 13" }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        div { class: "re-block", "data-open": "false",
                            button {
                                aria_expanded: "false",
                                aria_haspopup: "listbox",
                                class: "re-block__btn",
                                r#type: "button",
                                span { class: "re-block__label", "Paragraph" }
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2.5",
                                    view_box: "0 0 24 24",
                                    polyline { points: "6 9 12 15 18 9" }
                                }
                            }
                            div { class: "re-block__menu", role: "listbox",
                                button {
                                    class: "re-block__item",
                                    "data-block": "P",
                                    r#type: "button",
                                    span { "Paragraph" }
                                    span { class: "re-block__item-hint", "P" }
                                }
                                button {
                                    class: "re-block__item re-block__item--h1",
                                    "data-block": "H1",
                                    r#type: "button",
                                    span { "Heading 1" }
                                    span { class: "re-block__item-hint", "H1" }
                                }
                                button {
                                    class: "re-block__item re-block__item--h2",
                                    "data-block": "H2",
                                    r#type: "button",
                                    span { "Heading 2" }
                                    span { class: "re-block__item-hint", "H2" }
                                }
                                button {
                                    class: "re-block__item re-block__item--h3",
                                    "data-block": "H3",
                                    r#type: "button",
                                    span { "Heading 3" }
                                    span { class: "re-block__item-hint", "H3" }
                                }
                                button {
                                    class: "re-block__item re-block__item--quote",
                                    "data-block": "BLOCKQUOTE",
                                    r#type: "button",
                                    span { "Quote" }
                                    span { class: "re-block__item-hint", "\"" }
                                }
                                button {
                                    class: "re-block__item re-block__item--code",
                                    "data-block": "PRE",
                                    r#type: "button",
                                    span { "Code block" }
                                    span { class: "re-block__item-hint", "{{ }}" }
                                }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Bold",
                            class: "re-tb-btn",
                            "data-cmd": "bold",
                            "data-tip": "Bold",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2.4",
                                view_box: "0 0 24 24",
                                path { d: "M6 4h8a4 4 0 0 1 0 8H6z" }
                                path { d: "M6 12h9a4 4 0 0 1 0 8H6z" }
                            }
                        }
                        button {
                            aria_label: "Italic",
                            class: "re-tb-btn",
                            "data-cmd": "italic",
                            "data-tip": "Italic",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "19",
                                    x2: "10",
                                    y1: "4",
                                    y2: "4",
                                }
                                line {
                                    x1: "14",
                                    x2: "5",
                                    y1: "20",
                                    y2: "20",
                                }
                                line {
                                    x1: "15",
                                    x2: "9",
                                    y1: "4",
                                    y2: "20",
                                }
                            }
                        }
                        button {
                            aria_label: "Underline",
                            class: "re-tb-btn",
                            "data-cmd": "underline",
                            "data-tip": "Underline",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M6 4v6a6 6 0 0 0 12 0V4" }
                                line {
                                    x1: "4",
                                    x2: "20",
                                    y1: "20",
                                    y2: "20",
                                }
                            }
                        }
                        button {
                            aria_label: "Strikethrough",
                            class: "re-tb-btn",
                            "data-cmd": "strikeThrough",
                            "data-tip": "Strikethrough",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "4",
                                    x2: "20",
                                    y1: "12",
                                    y2: "12",
                                }
                                path { d: "M16 6c-1.5-1.3-3.5-2-6-2-3 0-5 1.5-5 4 0 2 1.5 3 5 4" }
                                path { d: "M8 18c1.5 1.3 3.5 2 6 2 3 0 5-1.5 5-4 0-1-.4-1.8-1-2.4" }
                            }
                        }
                        button {
                            aria_label: "Inline code",
                            class: "re-tb-btn",
                            "data-cmd": "code-inline",
                            "data-tip": "Inline code",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                polyline { points: "16 18 22 12 16 6" }
                                polyline { points: "8 6 2 12 8 18" }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Align left",
                            class: "re-tb-btn",
                            "data-cmd": "justifyLeft",
                            "data-tip": "Align left",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "17",
                                    x2: "3",
                                    y1: "10",
                                    y2: "10",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "14",
                                    y2: "14",
                                }
                                line {
                                    x1: "17",
                                    x2: "3",
                                    y1: "18",
                                    y2: "18",
                                }
                            }
                        }
                        button {
                            aria_label: "Align center",
                            class: "re-tb-btn",
                            "data-cmd": "justifyCenter",
                            "data-tip": "Align center",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "18",
                                    x2: "6",
                                    y1: "10",
                                    y2: "10",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "14",
                                    y2: "14",
                                }
                                line {
                                    x1: "18",
                                    x2: "6",
                                    y1: "18",
                                    y2: "18",
                                }
                            }
                        }
                        button {
                            aria_label: "Align right",
                            class: "re-tb-btn",
                            "data-cmd": "justifyRight",
                            "data-tip": "Align right",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "21",
                                    x2: "7",
                                    y1: "10",
                                    y2: "10",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "14",
                                    y2: "14",
                                }
                                line {
                                    x1: "21",
                                    x2: "7",
                                    y1: "18",
                                    y2: "18",
                                }
                            }
                        }
                        button {
                            aria_label: "Justify",
                            class: "re-tb-btn",
                            "data-cmd": "justifyFull",
                            "data-tip": "Justify",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "10",
                                    y2: "10",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "14",
                                    y2: "14",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "18",
                                    y2: "18",
                                }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Bullet list",
                            class: "re-tb-btn",
                            "data-cmd": "insertUnorderedList",
                            "data-tip": "Bullet list",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "8",
                                    x2: "21",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "8",
                                    x2: "21",
                                    y1: "12",
                                    y2: "12",
                                }
                                line {
                                    x1: "8",
                                    x2: "21",
                                    y1: "18",
                                    y2: "18",
                                }
                                circle { cx: "4", cy: "6", r: "1" }
                                circle { cx: "4", cy: "12", r: "1" }
                                circle { cx: "4", cy: "18", r: "1" }
                            }
                        }
                        button {
                            aria_label: "Numbered list",
                            class: "re-tb-btn",
                            "data-cmd": "insertOrderedList",
                            "data-tip": "Numbered list",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "10",
                                    x2: "21",
                                    y1: "6",
                                    y2: "6",
                                }
                                line {
                                    x1: "10",
                                    x2: "21",
                                    y1: "12",
                                    y2: "12",
                                }
                                line {
                                    x1: "10",
                                    x2: "21",
                                    y1: "18",
                                    y2: "18",
                                }
                                path { d: "M4 6h1v4" }
                                path { d: "M4 10h2" }
                                path { d: "M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" }
                            }
                        }
                        button {
                            aria_label: "Outdent",
                            class: "re-tb-btn",
                            "data-cmd": "outdent",
                            "data-tip": "Outdent",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "5",
                                    y2: "5",
                                }
                                line {
                                    x1: "21",
                                    x2: "3",
                                    y1: "19",
                                    y2: "19",
                                }
                                line {
                                    x1: "21",
                                    x2: "11",
                                    y1: "9",
                                    y2: "9",
                                }
                                line {
                                    x1: "21",
                                    x2: "11",
                                    y1: "15",
                                    y2: "15",
                                }
                                polyline { points: "7 9 3 12 7 15" }
                            }
                        }
                        button {
                            aria_label: "Indent",
                            class: "re-tb-btn",
                            "data-cmd": "indent",
                            "data-tip": "Indent",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "3",
                                    x2: "21",
                                    y1: "5",
                                    y2: "5",
                                }
                                line {
                                    x1: "3",
                                    x2: "21",
                                    y1: "19",
                                    y2: "19",
                                }
                                line {
                                    x1: "13",
                                    x2: "3",
                                    y1: "9",
                                    y2: "9",
                                }
                                line {
                                    x1: "13",
                                    x2: "3",
                                    y1: "15",
                                    y2: "15",
                                }
                                polyline { points: "17 9 21 12 17 15" }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Insert link",
                            class: "re-tb-btn",
                            "data-cmd": "link",
                            "data-tip": "Insert link",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                                path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                            }
                        }
                        button {
                            aria_label: "Remove link",
                            class: "re-tb-btn",
                            "data-cmd": "unlink",
                            "data-tip": "Remove link",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M18.84 12.25l1.72-1.71a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                                path { d: "M5.16 11.75L3.44 13.46a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                                line {
                                    x1: "2",
                                    x2: "22",
                                    y1: "2",
                                    y2: "22",
                                }
                            }
                        }
                        button {
                            aria_label: "Insert image",
                            class: "re-tb-btn",
                            "data-cmd": "image",
                            "data-tip": "Insert image",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                rect {
                                    height: "18",
                                    rx: "2",
                                    width: "18",
                                    x: "3",
                                    y: "3",
                                }
                                circle { cx: "8.5", cy: "8.5", r: "1.5" }
                                polyline { points: "21 15 16 10 5 21" }
                            }
                        }
                        button {
                            aria_label: "Embed YouTube",
                            class: "re-tb-btn",
                            "data-cmd": "youtube",
                            "data-tip": "Embed YouTube",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M22 8.5a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v7a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4z" }
                                polygon { points: "10 8 16 12 10 16 10 8" }
                            }
                        }
                        button {
                            aria_label: "Insert table",
                            class: "re-tb-btn",
                            "data-cmd": "table",
                            "data-tip": "Insert table",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                rect {
                                    height: "18",
                                    rx: "2",
                                    width: "18",
                                    x: "3",
                                    y: "3",
                                }
                                line {
                                    x1: "3",
                                    x2: "21",
                                    y1: "9",
                                    y2: "9",
                                }
                                line {
                                    x1: "3",
                                    x2: "21",
                                    y1: "15",
                                    y2: "15",
                                }
                                line {
                                    x1: "9",
                                    x2: "9",
                                    y1: "3",
                                    y2: "21",
                                }
                                line {
                                    x1: "15",
                                    x2: "15",
                                    y1: "3",
                                    y2: "21",
                                }
                            }
                        }
                        button {
                            aria_label: "Horizontal rule",
                            class: "re-tb-btn",
                            "data-cmd": "hr",
                            "data-tip": "Horizontal rule",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                line {
                                    x1: "4",
                                    x2: "20",
                                    y1: "12",
                                    y2: "12",
                                }
                            }
                        }
                    }
                    div { class: "re-toolbar__group",
                        button {
                            aria_label: "Clear formatting",
                            class: "re-tb-btn",
                            "data-cmd": "removeFormat",
                            "data-tip": "Clear formatting",
                            r#type: "button",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M4 7V4h16v3" }
                                line {
                                    x1: "5",
                                    x2: "11",
                                    y1: "20",
                                    y2: "20",
                                }
                                line {
                                    x1: "13",
                                    x2: "8",
                                    y1: "4",
                                    y2: "20",
                                }
                                line {
                                    x1: "3",
                                    x2: "21",
                                    y1: "3",
                                    y2: "21",
                                }
                            }
                        }
                    }
                }
            }
            div {
                autocapitalize: "sentences",
                class: "re-content",
                contenteditable: "{editable}",
                "data-empty": "true",
                spellcheck: "true",
                dangerous_inner_html: "{content}",
            }
            if is_editable {
                div { class: "re-statusbar",
                    div { class: "re-statusbar__chips",
                        span { class: "re-statusbar__chip",
                            "Words "
                            strong { class: "re-word-count", "0" }
                        }
                        span { class: "re-statusbar__chip",
                            "Chars "
                            strong { class: "re-char-count", "0" }
                        }
                    }
                    div { class: "re-statusbar__chip re-ime-state", "composing…" }
                }
                div { class: "re-modal-mask", "data-modal": "link",
                    div {
                        aria_label: "Insert link",
                        class: "re-modal",
                        role: "dialog",
                        div { class: "re-modal__title", "Insert link" }
                        div { class: "re-modal__field",
                            label { "URL" }
                            input {
                                autocomplete: "off",
                                class: "re-link-url",
                                placeholder: "https://example.com",
                                r#type: "url",
                            }
                        }
                        div { class: "re-modal__actions",
                            button {
                                class: "re-btn",
                                "data-close-modal": "false",
                                r#type: "button",
                                "Cancel"
                            }
                            button {
                                class: "re-btn re-btn--primary re-link-confirm",
                                r#type: "button",
                                "Insert"
                            }
                        }
                    }
                }
                div { class: "re-modal-mask", "data-modal": "image",
                    div {
                        aria_label: "Insert image",
                        class: "re-modal",
                        role: "dialog",
                        div { class: "re-modal__title", "Insert image" }

                        label { class: "re-dropzone", "data-dragging": "false",
                            input {
                                class: "re-image-file",
                                accept: "image/*",
                                hidden: true,
                                r#type: "file",
                            }
                            svg {
                                class: "re-dropzone__icon",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect {
                                    x: "3",
                                    y: "3",
                                    width: "18",
                                    height: "18",
                                    rx: "2",
                                }
                                circle { cx: "8.5", cy: "8.5", r: "1.5" }
                                polyline { points: "21 15 16 10 5 21" }
                            }
                            div { class: "re-dropzone__title", "Drag & drop an image" }
                            div { class: "re-dropzone__hint",
                                "or tap to browse — PNG, JPG, GIF, WebP"
                            }
                        }

                        label { class: "re-camera-btn",
                            input {
                                class: "re-image-camera",
                                accept: "image/*",
                                capture: "environment",
                                hidden: true,
                                r#type: "file",
                            }
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.8",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z" }
                                circle { cx: "12", cy: "13", r: "4" }
                            }
                            span { "Take a photo" }
                        }

                        div { class: "re-modal__divider",
                            span { "OR PASTE A URL" }
                        }

                        div { class: "re-modal__field",
                            input {
                                class: "re-image-url",
                                placeholder: "https://example.com/image.png",
                                r#type: "url",
                            }
                        }

                        div { class: "re-modal__actions",
                            button {
                                class: "re-btn",
                                "data-close-modal": "false",
                                r#type: "button",
                                "Cancel"
                            }
                            button {
                                class: "re-btn re-btn--primary re-image-confirm",
                                r#type: "button",
                                "Insert URL"
                            }
                        }
                    }
                }
                div { class: "re-modal-mask", "data-modal": "youtube",
                    div {
                        aria_label: "Embed YouTube",
                        class: "re-modal",
                        role: "dialog",
                        div { class: "re-modal__title", "Embed YouTube video" }
                        div { class: "re-modal__field",
                            label { "YouTube URL or ID" }
                            input {
                                class: "re-youtube-url",
                                placeholder: "https://youtu.be/dQw4w9WgXcQ",
                                r#type: "text",
                            }
                        }
                        div { class: "re-modal__actions",
                            button {
                                class: "re-btn",
                                "data-close-modal": "false",
                                r#type: "button",
                                "Cancel"
                            }
                            button {
                                class: "re-btn re-btn--primary re-youtube-confirm",
                                r#type: "button",
                                "Embed"
                            }
                        }
                    }
                }
                div { class: "re-modal-mask", "data-modal": "table",
                    div {
                        aria_label: "Insert table",
                        class: "re-modal",
                        role: "dialog",
                        div { class: "re-modal__title", "Insert table" }
                        div { class: "re-modal__row",
                            div { class: "re-modal__field",
                                label { "Rows" }
                                input {
                                    class: "re-table-rows",
                                    max: "20",
                                    min: "1",
                                    r#type: "number",
                                    value: "3",
                                }
                            }
                            div { class: "re-modal__field",
                                label { "Columns" }
                                input {
                                    class: "re-table-cols",
                                    max: "10",
                                    min: "1",
                                    r#type: "number",
                                    value: "3",
                                }
                            }
                        }
                        div { class: "re-modal__field",
                            label { class: "re-modal__checkbox",
                                input {
                                    checked: "false",
                                    class: "re-table-header",
                                    r#type: "checkbox",
                                }
                                " First row is header\n        "
                            }
                        }
                        div { class: "re-modal__actions",
                            button {
                                class: "re-btn",
                                "data-close-modal": "false",
                                r#type: "button",
                                "Cancel"
                            }
                            button {
                                class: "re-btn re-btn--primary re-table-confirm",
                                r#type: "button",
                                "Insert"
                            }
                        }
                    }
                }
            }
        }
    }
}
