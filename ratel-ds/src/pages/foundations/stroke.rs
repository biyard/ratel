use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, TokenRow, TokenTable, DoDont, DoDontGrid};

#[component]
pub fn Stroke() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Stroke".to_string(),
                subtitle: "Stroke tokens define border widths used across all components. Values range from 0.5px (hairline) to 3px (heavy emphasis). Source: variants/primitive/stroke.json.".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Visual scale ──────────────────────────────────────────────
            DocSection {
                title: "Stroke Scale".to_string(),
                id:    "scale".to_string(),
                description: "All eleven stroke widths visualized. Each row shows the stroke weight applied to a sample line and border.".to_string(),

                div {
                    class: "rounded-ratel-xl border overflow-hidden",
                    style: "border-color: var(--ratel-color-border-stroke-neutral-800); background: var(--ratel-color-border-background-neutral-850);",
                    for step in STROKE_SCALE {
                        StrokeRow { entry: step }
                    }
                }
            }

            // ── Token table ───────────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                TokenTable { rows: STROKE_TOKEN_ROWS.to_vec() }
            }

            // ── Usage ─────────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: STROKE_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Stroke scale row ─────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct StrokeEntry {
    token:       &'static str,   // e.g. "--ratel-stroke-1"
    px:          &'static str,   // e.g. "1px"
    tw:          &'static str,   // e.g. "border-ratel-1"
    description: &'static str,
}

static STROKE_SCALE: &[StrokeEntry] = &[
    StrokeEntry { token: "--ratel-stroke-0p5",  px: "0.5px", tw: "border-ratel-05",  description: "Hairline — subtle dividers, ghost inputs" },
    StrokeEntry { token: "--ratel-stroke-0p75", px: "0.75px",tw: "border-ratel-075", description: "Fine — light separator lines" },
    StrokeEntry { token: "--ratel-stroke-1",    px: "1px",   tw: "border-ratel-1",   description: "Default — card borders, input fields, table rows" },
    StrokeEntry { token: "--ratel-stroke-1p25", px: "1.25px",tw: "border-ratel-125", description: "Slightly stronger border" },
    StrokeEntry { token: "--ratel-stroke-1p5",  px: "1.5px", tw: "border-ratel-15",  description: "Medium — hover/selected borders on cards" },
    StrokeEntry { token: "--ratel-stroke-1p75", px: "1.75px",tw: "border-ratel-175", description: "Medium-heavy" },
    StrokeEntry { token: "--ratel-stroke-2",    px: "2px",   tw: "border-ratel-2",   description: "Heavy — focus rings, active states, emphasis" },
    StrokeEntry { token: "--ratel-stroke-2p25", px: "2.25px",tw: "border-ratel-225", description: "Extra emphasis" },
    StrokeEntry { token: "--ratel-stroke-2p5",  px: "2.5px", tw: "border-ratel-25",  description: "Strong — primary CTA active border" },
    StrokeEntry { token: "--ratel-stroke-2p75", px: "2.75px",tw: "border-ratel-275", description: "Very heavy" },
    StrokeEntry { token: "--ratel-stroke-3",    px: "3px",   tw: "border-ratel-3",   description: "Maximum — decorative borders, section dividers" },
];

#[derive(Props, Clone, PartialEq)]
struct StrokeRowProps { entry: &'static StrokeEntry }

#[component]
fn StrokeRow(props: StrokeRowProps) -> Element {
    let e = props.entry;
    // px → float for visual line height
    let px_f: f32 = e.px.trim_end_matches("px").parse().unwrap_or(1.0);
    let line_h = (px_f * 2.0).max(2.0);   // scale up so thin lines are visible

    rsx! {
        div {
            class: "flex items-center gap-6 px-5 py-3",
            style: "border-bottom: 1px solid var(--ratel-color-border-stroke-neutral-800);",

            // Visual line
            div {
                class: "shrink-0 w-32 rounded-full",
                style: "height: {line_h}px; background: var(--ratel-color-generic-primary);",
            }

            // Labels
            div { class: "flex items-center gap-6 flex-1 min-w-0",
                span {
                    class: "text-label-2 font-semibold w-14 shrink-0 font-mono",
                    style: "color: var(--ratel-color-font-default);",
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

static STROKE_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-stroke-0p5",  "0.5px", "Hairline border — ghost inputs, subtle dividers"),
    TokenRow::new("--ratel-stroke-1",    "1px",   "Default — card borders, input borders, table rows"),
    TokenRow::new("--ratel-stroke-1p5",  "1.5px", "Selected/hover card border"),
    TokenRow::new("--ratel-stroke-2",    "2px",   "Focus ring, active-state border"),
    TokenRow::new("--ratel-stroke-2p5",  "2.5px", "Primary CTA pressed border"),
    TokenRow::new("--ratel-stroke-3",    "3px",   "Decorative or maximum-emphasis border"),
];

static STROKE_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Use 1px for default borders",
        "The standard card and input border is border-ratel-1 (1px). Heavier strokes are reserved for focus rings (2px) and active states."
    ),
    DoDont::dont(
        "Use thick borders for decoration only",
        "Borders communicate meaning (input, card boundary, focus). Decorative thick borders add visual noise — reserve heavy strokes for interactive state changes."
    ),
    DoDont::do_(
        "Use 2px for focus rings",
        "All focusable elements should have a border-ratel-2 ring in the primary color (border-ratel-stroke-pri) for keyboard accessibility."
    ),
    DoDont::dont(
        "Use fractional pixel values outside the token set",
        "Don't introduce border-[1.3px] or border-[0.9px]. Every needed width exists in the stroke scale; add a token to stroke.json if a new step is truly needed."
    ),
];
