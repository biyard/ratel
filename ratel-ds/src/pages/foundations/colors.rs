use dioxus::prelude::*;
use crate::components::docs::{
    PageIntro, DocSection,
    ColorSwatch, ColorSwatchGrid,
    PaletteStep, PaletteStrip,
    TokenRow, TokenTable,
    DoDont, DoDontGrid,
};

#[component]
pub fn Colors() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Color".to_string(),
                subtitle: "The Ratel color system is built on two layers: primitive colors (raw palette) and semantic tokens (contextual meaning). Always use semantic tokens in components; use primitive colors only to define new semantic tokens.".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Brand / generic ───────────────────────────────────────────
            DocSection {
                title: "Brand Color".to_string(),
                id:    "brand".to_string(),
                description: "The Ratel brand is anchored by a warm amber-orange (#F79800). It is used for primary actions, active states, links, and key highlights. All opacity variants are pre-computed tokens.".to_string(),

                div { class: "mb-6",
                    ColorSwatchGrid { swatches: BRAND_SWATCHES.to_vec() }
                }

                TokenTable {
                    rows: BRAND_TOKEN_ROWS.to_vec(),
                    show_preview: true,
                }
            }

            // ── Status ────────────────────────────────────────────────────
            DocSection {
                title: "Status Colors".to_string(),
                id:    "status".to_string(),
                description: "Semantic colors for communicating state to users. Each has a full token and 5%/10% opacity variants for backgrounds and borders.".to_string(),

                div { class: "mb-6",
                    ColorSwatchGrid { swatches: STATUS_SWATCHES.to_vec() }
                }

                TokenTable {
                    rows: STATUS_TOKEN_ROWS.to_vec(),
                    show_preview: true,
                }
            }

            // ── Neutral palette ───────────────────────────────────────────
            DocSection {
                title: "Neutral Palette".to_string(),
                id:    "neutral".to_string(),
                description: "The neutral scale from 50 (near-white) to 950 (near-black) forms the backbone of surfaces, text, and border colors. Semantic tokens alias into this scale.".to_string(),

                PaletteStrip { title: "Neutral Scale".to_string(), steps: NEUTRAL_STEPS.to_vec() }
            }

            // ── Semantic: font ────────────────────────────────────────────
            DocSection {
                title: "Semantic Tokens — Font".to_string(),
                id:    "font-tokens".to_string(),
                description: "Font color tokens map text roles to neutral or brand palette values. They change between light and dark themes automatically.".to_string(),

                TokenTable {
                    rows: FONT_TOKEN_ROWS.to_vec(),
                    show_preview: true,
                }
            }

            // ── Semantic: border ──────────────────────────────────────────
            DocSection {
                title: "Semantic Tokens — Border & Surface".to_string(),
                id:    "border-tokens".to_string(),
                description: "Surface and border tokens define background layers, stroke colors, and dividers. They switch between light and dark theme values automatically.".to_string(),

                TokenTable {
                    rows: BORDER_TOKEN_ROWS.to_vec(),
                    show_preview: true,
                }
            }

            // ── Do / Don't ────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: COLOR_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Data ─────────────────────────────────────────────────────────────────────

static BRAND_SWATCHES: &[ColorSwatch] = &[
    ColorSwatch::new("Primary",    "#F79800", "#F79800").with_token("--ratel-color-generic-primary"),
    ColorSwatch::new("Primary 75%","rgba(247,152,0,0.75)", "75% opacity").with_token("--ratel-color-generic-primary-opacity-75%"),
    ColorSwatch::new("Primary 50%","rgba(247,152,0,0.50)", "50% opacity").with_token("--ratel-color-generic-primary-opacity-50%"),
    ColorSwatch::new("Primary 25%","rgba(247,152,0,0.25)", "25% opacity").with_token("--ratel-color-generic-primary-opacity-25%"),
    ColorSwatch::new("Primary 15%","rgba(247,152,0,0.15)", "15% opacity").with_token("--ratel-color-generic-primary-opacity-15%"),
    ColorSwatch::new("Primary 10%","rgba(247,152,0,0.10)", "10% opacity").with_token("--ratel-color-generic-primary-opacity-10%"),
    ColorSwatch::new("Primary 5%", "rgba(247,152,0,0.05)", "5% opacity").with_token("--ratel-color-generic-primary-opacity-5%"),
];

static BRAND_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-color-generic-primary",            "#F79800",                 "Brand orange — primary actions, focus rings, active states"),
    TokenRow::new("--ratel-color-generic-primary-opacity-75%","rgba(247,152,0,0.75)",    "Hover-state tint for primary elements"),
    TokenRow::new("--ratel-color-generic-primary-opacity-25%","rgba(247,152,0,0.25)",    "Border highlight on focused fields"),
    TokenRow::new("--ratel-color-generic-primary-opacity-10%","rgba(247,152,0,0.10)",    "Active nav link background"),
    TokenRow::new("--ratel-color-generic-primary-opacity-5%", "rgba(247,152,0,0.05)",    "Subtle selected-row background"),
];

static STATUS_SWATCHES: &[ColorSwatch] = &[
    ColorSwatch::new("Error",   "#FB2C36", "#FB2C36").with_token("--ratel-color-generic-error"),
    ColorSwatch::new("Info",    "#2B7FFF", "#2B7FFF").with_token("--ratel-color-generic-info"),
    ColorSwatch::new("Success", "#00C950", "#00C950").with_token("--ratel-color-generic-success"),
];

static STATUS_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-color-generic-error",           "#FB2C36", "Destructive actions, validation errors, alerts"),
    TokenRow::new("--ratel-color-generic-error-opacity-5%","rgba(239,68,68,0.05)", "Error field background tint"),
    TokenRow::new("--ratel-color-generic-error-opacity-10%","rgba(239,68,68,0.10)","Error badge background"),
    TokenRow::new("--ratel-color-generic-info",            "#2B7FFF", "Informational messages, links, help text"),
    TokenRow::new("--ratel-color-generic-info-opacity-10%","rgba(43,127,255,0.10)","Info badge background"),
    TokenRow::new("--ratel-color-generic-success",         "#00C950", "Confirmations, completed states, positive feedback"),
    TokenRow::new("--ratel-color-generic-success-opacity-10%","rgba(34,197,94,0.10)","Success badge background"),
];

// Neutral scale from tokens (neutral/50 → neutral/950)
static NEUTRAL_STEPS: &[PaletteStep] = &[
    PaletteStep { label: "50",  color: "#FAFAFA", hex: "#FAFAFA" },
    PaletteStep { label: "100", color: "#F5F5F5", hex: "#F5F5F5" },
    PaletteStep { label: "200", color: "#E5E5E5", hex: "#E5E5E5" },
    PaletteStep { label: "300", color: "#D4D4D4", hex: "#D4D4D4" },
    PaletteStep { label: "400", color: "#A1A1A1", hex: "#A1A1A1" },
    PaletteStep { label: "500", color: "#737373", hex: "#737373" },
    PaletteStep { label: "600", color: "#525252", hex: "#525252" },
    PaletteStep { label: "700", color: "#404040", hex: "#404040" },
    PaletteStep { label: "800", color: "#262626", hex: "#262626" },
    PaletteStep { label: "900", color: "#171717", hex: "#171717" },
    PaletteStep { label: "950", color: "#0A0A0A", hex: "#0A0A0A" },
];

static FONT_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-color-font-default",         "#262626 / #A1A1A1", "Primary text — headings, body, labels"),
    TokenRow::new("--ratel-color-font-primary",         "#F79800",           "Primary-colored text and icon labels"),
    TokenRow::new("--ratel-color-font-secondary",       "#FFFFFF / #171717", "Inverted surface text"),
    TokenRow::new("--ratel-color-font-header",          "#525252 / #8C8C8C", "Section headers, column headings"),
    TokenRow::new("--ratel-color-font-body",            "#525252 / #D4D4D4", "Body copy, descriptions, secondary labels"),
    TokenRow::new("--ratel-color-font-disable",         "#404040",           "Disabled text (same in both themes)"),
    TokenRow::new("--ratel-color-font-neutral-absolute","#8C8C8C",           "Muted labels, placeholders (same in both themes)"),
    TokenRow::new("--ratel-color-font-black-absolute",  "#0A0A0A",           "Absolute black — always dark regardless of theme"),
    TokenRow::new("--ratel-color-font-white-absolute",  "#FFFFFF",           "Absolute white — always light regardless of theme"),
    TokenRow::new("--ratel-color-font-invert-black",    "#FFFFFF / #0A0A0A", "Text on dark fills in light / dark mode"),
    TokenRow::new("--ratel-color-font-invert-white",    "#171717 / #FFFFFF", "Text on light fills in light / dark mode"),
];

static BORDER_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-color-border-background-neutral-850", "#FFFFFF",  "Primary card/panel background surface"),
    TokenRow::new("--ratel-color-border-background-neutral-950", "#FAFAFA",  "Page-level background (outermost layer)"),
    TokenRow::new("--ratel-color-border-background-neutral-800", "#F0F0F0",  "Subtle secondary surface (in-card header)"),
    TokenRow::new("--ratel-color-border-incard-background-default","#F5F5F5","In-card nested background"),
    TokenRow::new("--ratel-color-border-stroke-neutral-800",     "#E5E5E5",  "Default border on cards and inputs"),
    TokenRow::new("--ratel-color-border-stroke-neutral-700",     "#E5E5E5",  "Slightly stronger border on containers"),
    TokenRow::new("--ratel-color-border-stroke-primary",         "#F79800",  "Focus ring and primary-state border"),
    TokenRow::new("--ratel-color-divider-netural-800",           "#E5E5E5",  "Horizontal / vertical divider lines"),
    TokenRow::new("--ratel-color-divider-netural-700",           "#525252",  "Stronger divider (dark mode / dark surfaces)"),
];

static COLOR_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Use semantic token names",
        "Reference --ratel-color-* tokens via Tailwind aliases (text-ratel-text, bg-ratel-bg-white). Semantic tokens switch automatically between light and dark themes."
    ),
    DoDont::dont(
        "Use raw hex values in components",
        "Never write color: #F79800 or bg-[#F79800] directly in a component. Use the token alias (text-ratel-primary) so the value is maintainable and theme-aware."
    ),
    DoDont::do_(
        "Layer surfaces using the background scale",
        "Page bg → ratel-bg-muted (#FAFAFA), Cards → ratel-bg-white (#FFFFFF), In-card nested → ratel-bg-subtle (#F5F5F5). Three clear levels of elevation."
    ),
    DoDont::dont(
        "Use primitive colors for semantic roles",
        "Avoid aliasing directly to primitive-color.json values like neutral/800 in component code. Instead, define or use the appropriate semantic token from Light/Dark.tokens.json."
    ),
];
