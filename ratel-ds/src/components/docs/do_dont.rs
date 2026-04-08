use dioxus::prelude::*;

// ─── DoDont ───────────────────────────────────────────────────────────────────
// A single Do or Don't guidance block.

#[derive(Clone, PartialEq, Debug)]
pub enum DoDontKind {
    Do,
    Dont,
}

#[derive(Clone, PartialEq)]
pub struct DoDont {
    pub kind:        DoDontKind,
    pub heading:     &'static str,
    pub description: &'static str,
    /// Optional inline example (plain HTML string — injected via dangerous_inner_html)
    pub example:     Option<&'static str>,
}

impl DoDont {
    pub const fn do_(heading: &'static str, description: &'static str) -> Self {
        Self { kind: DoDontKind::Do, heading, description, example: None }
    }

    pub const fn dont(heading: &'static str, description: &'static str) -> Self {
        Self { kind: DoDontKind::Dont, heading, description, example: None }
    }

    pub const fn with_example(mut self, html: &'static str) -> Self {
        self.example = Some(html);
        self
    }
}

// ─── DoDontGrid ───────────────────────────────────────────────────────────────
// Side-by-side grid of Do/Don't blocks. Usually pairs of [do, dont].
//
// Usage:
//   DoDontGrid { items: vec![
//       DoDont::do_("Use semantic token names", "…"),
//       DoDont::dont("Use raw hex values", "…"),
//   ]}

#[derive(Props, Clone, PartialEq)]
pub struct DoDontGridProps {
    pub items: Vec<DoDont>,
}

#[component]
pub fn DoDontGrid(props: DoDontGridProps) -> Element {
    rsx! {
        div { class: "ds-do-dont-grid",
            for item in &props.items {
                DoDontCard { item: item.clone() }
            }
        }
    }
}

// ─── DoDontCard (internal) ────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct DoDontCardProps {
    item: DoDont,
}

#[component]
fn DoDontCard(props: DoDontCardProps) -> Element {
    let is_do = props.item.kind == DoDontKind::Do;

    rsx! {
        div {
            class: if is_do { "ds-do-block" } else { "ds-dont-block" },

            // Header
            div { class: if is_do { "ds-do-header" } else { "ds-dont-header" },
                // Icon
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "w-4 h-4 shrink-0",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    if is_do {
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M5 13l4 4L19 7",
                        }
                    } else {
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 18L18 6M6 6l12 12",
                        }
                    }
                }
                span { { props.item.heading } }
            }

            // Body
            div { class: "ds-do-body",
                if let Some(example_html) = props.item.example {
                    div {
                        class: "mb-3 p-4 rounded-ratel-md",
                        style: "background: var(--ratel-color-border-incard-background-default);",
                        dangerous_inner_html: example_html,
                    }
                }
                p { { props.item.description } }
            }
        }
    }
}
