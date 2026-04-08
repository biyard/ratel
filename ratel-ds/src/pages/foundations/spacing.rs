use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, TokenRow, TokenTable, DoDont, DoDontGrid};

#[component]
pub fn Spacing() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Spacing".to_string(),
                subtitle: "The Ratel spacing scale is a numeric sequence sourced from token.json, margin.json, and padding.json. All spacing values are CSS custom properties. Use the Tailwind ratel-* spacing aliases or arbitrary value syntax var(--ratel-space-N) in styles.".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Visual scale ──────────────────────────────────────────────
            DocSection {
                title: "Spacing Scale".to_string(),
                id:    "scale".to_string(),
                description: "Key steps from the spacing scale. The full set covers 0–1536px. Use multiples of 4 (4, 8, 12, 16, 24, 32, 40, 48, 64, 80) for most layouts.".to_string(),

                div {
                    class: "rounded-ratel-xl border overflow-hidden",
                    style: "border-color: var(--ratel-color-border-stroke-neutral-800); background: var(--ratel-color-border-background-neutral-850);",

                    for step in SPACING_VISUAL {
                        SpacingVisualRow { entry: step }
                    }
                }
            }

            // ── Token table ───────────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                TokenTable { rows: SPACING_TOKEN_ROWS.to_vec() }
            }

            // ── Usage ─────────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: SPACING_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Spacing visual row ───────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct SpacingStep {
    token:       &'static str,   // e.g. "--ratel-space-8"
    px:          &'static str,   // e.g. "8px"
    tailwind:    &'static str,   // e.g. "p-ratel-8"
    description: &'static str,
}

static SPACING_VISUAL: &[SpacingStep] = &[
    SpacingStep { token: "--ratel-space-4",   px: "4px",   tailwind: "p-ratel-4",   description: "Component inner gap, icon pad" },
    SpacingStep { token: "--ratel-space-8",   px: "8px",   tailwind: "p-ratel-8",   description: "Input padding, compact gaps" },
    SpacingStep { token: "--ratel-space-12",  px: "12px",  tailwind: "p-ratel-12",  description: "Button padding (vertical)" },
    SpacingStep { token: "--ratel-space-16",  px: "16px",  tailwind: "p-ratel-16",  description: "Default padding, card inner" },
    SpacingStep { token: "--ratel-space-20",  px: "20px",  tailwind: "p-ratel-20",  description: "Section gap, form row spacing" },
    SpacingStep { token: "--ratel-space-24",  px: "24px",  tailwind: "p-ratel-24",  description: "Card padding, modal padding" },
    SpacingStep { token: "--ratel-space-32",  px: "32px",  tailwind: "p-ratel-32",  description: "Section content spacing" },
    SpacingStep { token: "--ratel-space-40",  px: "40px",  tailwind: "p-ratel-40",  description: "Page section spacing (compact)" },
    SpacingStep { token: "--ratel-space-48",  px: "48px",  tailwind: "p-ratel-48",  description: "Page section spacing (default)" },
    SpacingStep { token: "--ratel-space-64",  px: "64px",  tailwind: "p-ratel-64",  description: "Large section padding" },
    SpacingStep { token: "--ratel-space-80",  px: "80px",  tailwind: "p-ratel-80",  description: "Between major page sections" },
    SpacingStep { token: "--ratel-space-96",  px: "96px",  tailwind: "p-ratel-96",  description: "Hero sections, major gaps" },
    SpacingStep { token: "--ratel-space-128", px: "128px", tailwind: "p-ratel-128", description: "Full-bleed section divisions" },
];

#[derive(Props, Clone, PartialEq)]
struct SpacingVisualRowProps { entry: &'static SpacingStep }

#[component]
fn SpacingVisualRow(props: SpacingVisualRowProps) -> Element {
    let e = props.entry;
    // Clamp bar width to max 300px for readability
    let px_val: u32 = e.px.trim_end_matches("px").parse().unwrap_or(0);
    let bar_w = px_val.min(300);

    rsx! {
        div {
            class: "flex items-center gap-5 px-5 py-3",
            style: "border-bottom: 1px solid var(--ratel-color-border-stroke-neutral-800);",

            // Bar
            div {
                class: "shrink-0",
                style: "width: {bar_w}px; height: 20px; border-radius: var(--ratel-radius-xs); background: var(--ratel-color-generic-primary-opacity-25%); border: 1.5px solid var(--ratel-color-generic-primary);",
            }

            // Value + token
            div { class: "flex items-center gap-6 flex-1 min-w-0",
                span {
                    class: "text-label-2 font-semibold w-12 shrink-0",
                    style: "color: var(--ratel-color-font-default); font-family: monospace;",
                    { e.px }
                }
                code {
                    class: "text-label-3 font-mono hidden sm:block",
                    style: "color: var(--ratel-color-generic-primary);",
                    { e.token }
                }
                span {
                    class: "text-label-3 truncate",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    { e.description }
                }
            }
        }
    }
}

// ─── Token rows ───────────────────────────────────────────────────────────────

static SPACING_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-space-0",   "0px",   "Zero spacing — no gap or padding"),
    TokenRow::new("--ratel-space-4",   "4px",   "Micro gap — icon padding, between inline labels"),
    TokenRow::new("--ratel-space-6",   "6px",   "Extra-small gap"),
    TokenRow::new("--ratel-space-8",   "8px",   "Small gap — input inner padding"),
    TokenRow::new("--ratel-space-12",  "12px",  "Medium-small — button vertical padding"),
    TokenRow::new("--ratel-space-16",  "16px",  "Default — card inner, form rows"),
    TokenRow::new("--ratel-space-20",  "20px",  "Medium-large — section content gaps"),
    TokenRow::new("--ratel-space-24",  "24px",  "Large — card/modal padding"),
    TokenRow::new("--ratel-space-32",  "32px",  "Extra-large — section spacer"),
    TokenRow::new("--ratel-space-40",  "40px",  "Page content sections (compact)"),
    TokenRow::new("--ratel-space-48",  "48px",  "Page content sections (default)"),
    TokenRow::new("--ratel-space-64",  "64px",  "Major vertical rhythm"),
    TokenRow::new("--ratel-space-80",  "80px",  "Between page-level sections"),
    TokenRow::new("--ratel-space-96",  "96px",  "Hero / feature sections"),
    TokenRow::new("--ratel-space-128", "128px", "Full-bleed section divisions"),
    TokenRow::new("--ratel-space-160", "160px", "Extra-large feature gaps"),
    TokenRow::new("--ratel-space-256", "256px", "Layout-level spacing"),
];

static SPACING_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Use token-backed spacing values",
        "Apply spacing via Tailwind aliases (p-ratel-16, gap-ratel-8) or var(--ratel-space-N) in inline styles. Token values are versioned and maintained in token.json."
    ),
    DoDont::dont(
        "Use arbitrary pixel values",
        "Avoid p-[17px] or margin: 13px. If the exact value doesn't exist in the scale, use the nearest token step — add a new token only if there is a clear systemic need."
    ),
    DoDont::do_(
        "Prefer multiples of 4",
        "Core UI should use 4, 8, 12, 16, 24, 32 as the primary steps. Larger values (40, 48, 64, 80) are for section-level spacing."
    ),
    DoDont::dont(
        "Mix spacing systems",
        "Don't combine ratel tokens with raw Tailwind steps (p-3, p-6) in the same component. Standardize on ratel-* steps throughout."
    ),
];
