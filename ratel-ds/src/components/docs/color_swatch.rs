use dioxus::prelude::*;

// ─── ColorSwatch ──────────────────────────────────────────────────────────────
// A single color swatch card showing a filled area + name + hex value.

#[derive(Clone, PartialEq)]
pub struct ColorSwatch {
    /// Display name (e.g. "Primary")
    pub name: &'static str,
    /// CSS color value or var() reference
    pub color: &'static str,
    /// Hex code shown as text
    pub hex: &'static str,
    /// Optional CSS token name
    pub token: Option<&'static str>,
}

impl ColorSwatch {
    pub const fn new(name: &'static str, color: &'static str, hex: &'static str) -> Self {
        Self { name, color, hex, token: None }
    }

    pub const fn with_token(mut self, token: &'static str) -> Self {
        self.token = Some(token);
        self
    }
}

// ─── ColorSwatchGrid ──────────────────────────────────────────────────────────
// Renders a responsive grid of color swatches.
//
// Usage:
//   ColorSwatchGrid { swatches: vec![
//       ColorSwatch::new("Primary", "#F79800", "#F79800")
//           .with_token("--ratel-color-generic-primary"),
//   ]}

#[derive(Props, Clone, PartialEq)]
pub struct ColorSwatchGridProps {
    pub swatches: Vec<ColorSwatch>,
}

#[component]
pub fn ColorSwatchGrid(props: ColorSwatchGridProps) -> Element {
    rsx! {
        div { class: "ds-swatch-grid",
            for swatch in &props.swatches {
                SwatchCard { swatch: swatch.clone() }
            }
        }
    }
}

// ─── SwatchCard (internal) ────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct SwatchCardProps {
    swatch: ColorSwatch,
}

#[component]
fn SwatchCard(props: SwatchCardProps) -> Element {
    let s = &props.swatch;
    rsx! {
        div { class: "ds-swatch-card",
            // Color fill
            div {
                class: "ds-swatch-color",
                style: "background-color: {s.color};",
            }
            // Info
            div { class: "ds-swatch-info",
                div { class: "ds-swatch-name", { s.name } }
                div { class: "ds-swatch-value", { s.hex } }
                if let Some(tok) = s.token {
                    div {
                        class: "ds-swatch-value mt-0.5 truncate",
                        style: "color: var(--ratel-color-generic-primary);",
                        { tok }
                    }
                }
            }
        }
    }
}

// ─── PaletteStrip ─────────────────────────────────────────────────────────────
// A horizontal strip of color steps (e.g. neutral 50→950).

#[derive(Clone, PartialEq)]
pub struct PaletteStep {
    pub label: &'static str,
    pub color: &'static str,
    pub hex:   &'static str,
}

#[derive(Props, Clone, PartialEq)]
pub struct PaletteStripProps {
    pub title:  String,
    pub steps:  Vec<PaletteStep>,
}

#[component]
pub fn PaletteStrip(props: PaletteStripProps) -> Element {
    rsx! {
        div { class: "mb-6",
            h4 {
                class: "text-label-2 font-semibold mb-2",
                style: "color: var(--ratel-color-font-header);",
                { props.title.as_str() }
            }
            div { class: "flex rounded-ratel-lg overflow-hidden border",
                style: "border-color: var(--ratel-color-border-stroke-neutral-800);",
                for step in &props.steps {
                    div {
                        class: "flex-1 flex flex-col",
                        style: "min-width: 0;",
                        // Color block
                        div {
                            class: "h-10",
                            style: "background-color: {step.color};",
                        }
                        // Label
                        div {
                            class: "px-1.5 py-1 text-center",
                            style: "background: var(--ratel-color-border-background-neutral-850); border-top: 1px solid var(--ratel-color-border-stroke-neutral-800);",
                            div {
                                class: "text-label-4 font-medium truncate",
                                style: "color: var(--ratel-color-font-default);",
                                { step.label }
                            }
                            div {
                                class: "text-label-5 truncate",
                                style: "color: var(--ratel-color-font-neutral-absolute); font-family: monospace;",
                                { step.hex }
                            }
                        }
                    }
                }
            }
        }
    }
}
