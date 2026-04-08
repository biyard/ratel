use dioxus::prelude::*;

// ─── ComponentPreview ─────────────────────────────────────────────────────────
// A bordered preview box with a canvas area (dotted background) and a
// bottom label bar. Wraps the live component(s) being documented.
//
// Usage:
//   ComponentPreview { label: "Primary button — all states",
//       button { class: "btn-primary", "Click me" }
//   }

#[derive(Props, Clone, PartialEq)]
pub struct ComponentPreviewProps {
    /// Short label shown in the bottom bar
    #[props(optional)]
    pub label: Option<String>,

    /// Override canvas padding / layout via extra CSS classes
    #[props(optional, default = String::new())]
    pub canvas_class: String,

    /// If true, render canvas without the dot-grid background
    #[props(optional, default = false)]
    pub plain: bool,

    pub children: Element,
}

#[component]
pub fn ComponentPreview(props: ComponentPreviewProps) -> Element {
    let canvas_style = if props.plain {
        "background-color: var(--ratel-color-border-background-neutral-850);"
    } else {
        // dot-grid defined in tailwind.css .ds-preview-canvas
        ""
    };

    rsx! {
        div { class: "ds-preview-box",
            // Canvas
            div {
                class: if props.canvas_class.is_empty() {
                    "ds-preview-canvas".to_string()
                } else {
                    format!("ds-preview-canvas {}", props.canvas_class)
                },
                style: canvas_style,
                { props.children }
            }
            // Bottom label
            if let Some(label) = &props.label {
                div { class: "ds-preview-label", { label.as_str() } }
            }
        }
    }
}

// ─── VariantRow ───────────────────────────────────────────────────────────────
// A horizontal strip of variants labeled on the left. Use inside PreviewCanvas
// when you want to show a set of related states in a row.

#[derive(Props, Clone, PartialEq)]
pub struct VariantRowProps {
    pub label: String,
    pub children: Element,
}

#[component]
pub fn VariantRow(props: VariantRowProps) -> Element {
    rsx! {
        div { class: "flex items-center gap-4 w-full",
            span {
                class: "text-label-3 shrink-0 w-24 text-right",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { props.label.as_str() }
            }
            div { class: "flex items-center gap-3 flex-wrap",
                { props.children }
            }
        }
    }
}
