use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, TokenTable, TokenRow, DoDont, DoDontGrid};

#[component]
pub fn Shadows() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Shadows & Elevation".to_string(),
                subtitle: "Shadows communicate elevation and depth. The Ratel elevation system has four levels (none, sm, md, lg) derived from the spacing/numeric token file. Shadow values reference token primitives — opacity levels use the numeric fractional tokens (-0.8, 0.5, etc.).".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Visual elevation ──────────────────────────────────────────
            DocSection {
                title: "Elevation Scale".to_string(),
                id:    "scale".to_string(),
                description: "Four elevation tiers. Higher elevation = more blur, larger offset, higher opacity. Dark theme uses lighter shadow colors.".to_string(),

                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 py-2",
                    for level in SHADOW_LEVELS {
                        ShadowCard { level }
                    }
                }
            }

            // ── Token reference ───────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                description: "Shadow values are composed from primitive spacing tokens (offset, blur, spread) and opacity tokens. These are applied via Tailwind's shadow utilities.".to_string(),

                TokenTable { rows: SHADOW_TOKEN_ROWS.to_vec() }
            }

            // ── Elevation mapping ─────────────────────────────────────────
            DocSection {
                title: "Elevation Mapping".to_string(),
                id:    "mapping".to_string(),
                description: "Match elevation level to component role — not visual preference. Higher elevation draws attention and implies interactivity.".to_string(),

                ElevationTable {}
            }

            // ── Guidelines ────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: SHADOW_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Shadow level data ────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct ShadowLevel {
    name:        &'static str,
    css_shadow:  &'static str,
    description: &'static str,
    tailwind:    &'static str,
}

static SHADOW_LEVELS: &[ShadowLevel] = &[
    ShadowLevel {
        name:        "none",
        css_shadow:  "none",
        description: "Flat surface — no elevation. Use for inline elements, table rows, disabled states.",
        tailwind:    "shadow-none",
    },
    ShadowLevel {
        name:        "sm",
        css_shadow:  "0 1px 2px rgba(0,0,0,0.06), 0 1px 3px rgba(0,0,0,0.10)",
        description: "Subtle lift — buttons, badges, small cards at rest.",
        tailwind:    "shadow-sm",
    },
    ShadowLevel {
        name:        "md",
        css_shadow:  "0 4px 6px rgba(0,0,0,0.07), 0 2px 4px rgba(0,0,0,0.06)",
        description: "Default elevation — cards, panels, input focus states.",
        tailwind:    "shadow-md",
    },
    ShadowLevel {
        name:        "lg",
        css_shadow:  "0 10px 15px rgba(0,0,0,0.10), 0 4px 6px rgba(0,0,0,0.05)",
        description: "High elevation — modals, drawers, popovers, floating actions.",
        tailwind:    "shadow-lg",
    },
];

#[derive(Props, Clone, PartialEq)]
struct ShadowCardProps { level: &'static ShadowLevel }

#[component]
fn ShadowCard(props: ShadowCardProps) -> Element {
    let l = props.level;
    rsx! {
        div { class: "flex flex-col items-center gap-4 py-6",
            style: "background: var(--ratel-color-border-background-neutral-950);",

            // Preview box
            div {
                class: "w-24 h-24 rounded-ratel-xl",
                style: "background: var(--ratel-color-border-background-neutral-850); box-shadow: {l.css_shadow};",
            }

            // Labels
            div { class: "text-center",
                div {
                    class: "text-label-2 font-semibold",
                    style: "color: var(--ratel-color-font-default);",
                    { l.name }
                }
                div {
                    class: "text-label-3 mt-0.5",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    { l.description }
                }
                code {
                    class: "text-label-4 font-mono mt-1 block",
                    style: "color: var(--ratel-color-generic-primary);",
                    { l.tailwind }
                }
            }
        }
    }
}

// ─── Token rows ───────────────────────────────────────────────────────────────

static SHADOW_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-space-1",   "1px",   "Y offset for sm shadow (from token.json)"),
    TokenRow::new("--ratel-space-2",   "2px",   "Blur radius for sm shadow"),
    TokenRow::new("--ratel-space-4",   "4px",   "Y offset for md shadow"),
    TokenRow::new("--ratel-space-6",   "6px",   "Blur for md shadow"),
    TokenRow::new("--ratel-space-10",  "10px",  "Blur for lg shadow"),
    TokenRow::new("--ratel-space-15",  "15px",  "Extended blur for lg shadow"),
    // Opacity values from token.json fractionals
    TokenRow::new("token: 0,06",  "0.06 (6%)",  "Shadow opacity — very subtle (sm)"),
    TokenRow::new("token: 0,08",  "0.08 (8%)",  "Shadow opacity — default (md)"),
    TokenRow::new("token: 0,10",  "0.10 (10%)", "Shadow opacity — elevated (lg)"),
];

// ─── Elevation mapping table ──────────────────────────────────────────────────

#[component]
fn ElevationTable() -> Element {
    static MAPPING: &[(&str, &str, &str)] = &[
        ("Flat text, table row, disabled element", "none (shadow-none)", "0"),
        ("Button at rest, badge, avatar",          "sm (shadow-sm)",     "1"),
        ("Card, panel, input focused",             "md (shadow-md)",     "2"),
        ("Modal, drawer, dropdown, tooltip",       "lg (shadow-lg)",     "3"),
    ];

    rsx! {
        div {
            class: "rounded-ratel-lg overflow-hidden border",
            style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

            table { class: "ds-token-table w-full",
                thead {
                    tr {
                        th { "Component Context" }
                        th { "Shadow Level" }
                        th { "z-tier" }
                    }
                }
                tbody {
                    for &(component, shadow, z) in MAPPING {
                        tr {
                            td { { component } }
                            td { code { class: "ds-token-pill", { shadow } } }
                            td {
                                span {
                                    class: "text-label-3",
                                    style: "color: var(--ratel-color-font-body);",
                                    { z }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

static SHADOW_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Reserve lg elevation for floating elements",
        "Only modals, drawers, dropdowns, and tooltips should use shadow-lg. Cards and panels use shadow-md to signal containment without competing with overlays."
    ),
    DoDont::dont(
        "Use shadow for decoration or depth styling",
        "Shadows encode elevation/interactivity. Don't add shadow-lg to a heading or decorative card just for visual effect — it misrepresents the element's role."
    ),
    DoDont::do_(
        "Remove shadow in dark mode where appropriate",
        "On very dark surfaces, shadows become invisible. Use border-ratel-1 with border-color tokens to preserve separation instead."
    ),
    DoDont::dont(
        "Apply hardcoded box-shadow values",
        "Don't write style=\"box-shadow: 0 8px 30px #000\". Use Tailwind's shadow-* classes backed by token-derived values. If a new shadow tier is needed, add it to the token system."
    ),
];
