use dioxus::prelude::*;

// ─── TokenRow ─────────────────────────────────────────────────────────────────
// A single row entry for the token table.

#[derive(Clone, PartialEq)]
pub struct TokenRow {
    /// Token name (e.g. "--ratel-color-generic-primary")
    pub name: &'static str,
    /// CSS variable or literal value (e.g. "#F79800")
    pub value: &'static str,
    /// Human-readable description
    pub description: &'static str,
    /// Optional category tag shown as a small badge
    pub category: Option<&'static str>,
}

impl TokenRow {
    pub const fn new(
        name: &'static str,
        value: &'static str,
        description: &'static str,
    ) -> Self {
        Self { name, value, description, category: None }
    }

    pub const fn with_category(mut self, cat: &'static str) -> Self {
        self.category = Some(cat);
        self
    }
}

// ─── TokenTable ───────────────────────────────────────────────────────────────
// Renders a formatted table listing token names, values, and descriptions.
//
// Usage:
//   TokenTable {
//       rows: vec![
//           TokenRow::new("--ratel-color-generic-primary", "#F79800", "Brand primary color"),
//       ]
//   }

#[derive(Props, Clone, PartialEq)]
pub struct TokenTableProps {
    pub rows: Vec<TokenRow>,

    /// Column header overrides
    #[props(optional, default = "Token".to_string())]
    pub col_name: String,

    #[props(optional, default = "Value".to_string())]
    pub col_value: String,

    #[props(optional, default = "Description".to_string())]
    pub col_desc: String,

    #[props(optional, default = false)]
    pub show_preview: bool,
}

#[component]
pub fn TokenTable(props: TokenTableProps) -> Element {
    rsx! {
        div {
            class: "rounded-ratel-lg overflow-hidden border",
            style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

            table { class: "ds-token-table",
                thead {
                    tr {
                        if props.show_preview {
                            th { class: "w-12", "" }
                        }
                        th { { props.col_name.as_str() } }
                        th { { props.col_value.as_str() } }
                        th { { props.col_desc.as_str() } }
                    }
                }
                tbody {
                    for row in &props.rows {
                        tr {
                            key: "{row.name}",

                            // Optional color preview swatch — uses CSS variable when token
                            // name starts with --ratel-color-* so the swatch responds to theme
                            if props.show_preview {
                                td { class: "w-10",
                                    if row.name.starts_with("--ratel-color-") {
                                        div {
                                            class: "w-7 h-7 rounded-ratel-md border",
                                            style: format!("background-color: var({}); border-color: var(--ratel-color-border-stroke-neutral-800);", row.name),
                                        }
                                    }
                                }
                            }

                            // Token name
                            td {
                                code { class: "ds-token-pill", { row.name } }
                                if let Some(cat) = row.category {
                                    span {
                                        class: "ml-2 text-label-4 px-1.5 py-0.5 rounded-ratel-xs",
                                        style: "background: var(--ratel-color-border-incard-background-default); color: var(--ratel-color-font-neutral-absolute);",
                                        { cat }
                                    }
                                }
                            }

                            // Value
                            td {
                                code {
                                    class: "text-label-3 font-mono",
                                    style: "color: var(--ratel-color-font-body);",
                                    { row.value }
                                }
                            }

                            // Description
                            td {
                                span {
                                    class: "text-label-3",
                                    style: "color: var(--ratel-color-font-body);",
                                    { row.description }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
