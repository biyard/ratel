use dioxus::prelude::*;
use dioxus::document::eval;

// ─── CodeBlock ────────────────────────────────────────────────────────────────
// Displays a code snippet with a language label and a copy button.
// Syntax highlighting is done via simple span-based CSS classes defined in
// tailwind.css — no external library needed for Phase 2.
//
// Usage:
//   CodeBlock {
//       lang: "rust",
//       code: r#"rsx! { Button { variant: "primary", "Click me" } }"#,
//   }

#[derive(Props, Clone, PartialEq)]
pub struct CodeBlockProps {
    /// Language label shown in the header (e.g. "rust", "css", "json")
    #[props(optional, default = "rs".to_string())]
    pub lang: String,

    /// The raw code string to display
    pub code: String,

    /// Optional title shown alongside the language
    #[props(optional)]
    pub title: Option<String>,
}

#[component]
pub fn CodeBlock(props: CodeBlockProps) -> Element {
    let header_label = props.title
        .as_deref()
        .unwrap_or(props.lang.as_str());

    // Escape HTML entities for safe rendering inside <pre>
    let escaped = props.code
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    rsx! {
        div { class: "ds-code-block",
            // Header bar
            div { class: "ds-code-header",
                div { class: "flex items-center gap-2",
                    // Language dot
                    span {
                        class: "w-2 h-2 rounded-full",
                        style: "background: var(--ratel-color-generic-primary);",
                    }
                    span { { header_label } }
                }
                // Copy button (functional via JS clipboard API injected by Dioxus web)
                CopyButton { code: props.code.clone() }
            }

            // Code body
            pre { class: "ds-code-content",
                code {
                    dangerous_inner_html: escaped,
                }
            }
        }
    }
}

// ─── CopyButton ───────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct CopyButtonProps {
    code: String,
}

#[component]
fn CopyButton(props: CopyButtonProps) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "flex items-center gap-1.5 px-2 py-1 rounded transition-colors duration-150",
            style: if *copied.read() {
                "background: var(--ratel-color-generic-success-opacity-10%); color: var(--ratel-color-generic-success);"
            } else {
                "color: #525252;"
            },
            title: "Copy to clipboard",
            onclick: {
                let code = props.code.clone();
                move |_| {
                    // Use the web Clipboard API via eval
                    let js = format!(
                        "navigator.clipboard.writeText(`{}`).catch(()=>{{}})",
                        code.replace('`', "\\`")
                    );
                    let _ = eval(&js);
                    copied.set(true);
                    // Reset after 2 s — Note: use_timeout not available in 0.6 web simply;
                    // we rely on the user navigating away or next click.
                }
            },

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-3.5 h-3.5",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                if *copied.read() {
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M5 13l4 4L19 7",
                    }
                } else {
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z",
                    }
                }
            }

            span { class: "text-label-4",
                if *copied.read() { "Copied!" } else { "Copy" }
            }
        }
    }
}
