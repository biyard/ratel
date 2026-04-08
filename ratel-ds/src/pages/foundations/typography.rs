use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, TokenRow, TokenTable, DoDont, DoDontGrid};

#[component]
pub fn Typography() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Typography".to_string(),
                subtitle: "The Ratel type system uses Raleway as the single font family across all weights and sizes. The scale is divided into Title, Heading, Label, and Body categories, each with precise size, line-height, and letter-spacing values sourced from typography.json.".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Font family ───────────────────────────────────────────────
            DocSection {
                title: "Font Family".to_string(),
                id:    "family".to_string(),
                description: "One font, Raleway, used for all text. It provides 9 weight steps from Thin (100) to Black (900). The CSS custom property is --ratel-font-family.".to_string(),

                div {
                    class: "rounded-ratel-xl border p-8 text-center",
                    style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800); font-family: var(--ratel-font-family);",

                    div {
                        class: "text-title-2 font-bold mb-2",
                        style: "color: var(--ratel-color-font-default);",
                        "Raleway"
                    }
                    div {
                        class: "text-body-1",
                        style: "color: var(--ratel-color-font-body);",
                        "Aa Bb Cc Dd Ee Ff Gg Hh Ii Jj Kk Ll Mm"
                    }
                    div {
                        class: "text-body-2 mt-1",
                        style: "color: var(--ratel-color-font-neutral-absolute);",
                        "0123456789 !@#$%^&*()_+-=[]{{}}|;':\",./<>?"
                    }
                }
            }

            // ── Weight scale ──────────────────────────────────────────────
            DocSection {
                title: "Weight Scale".to_string(),
                id:    "weights".to_string(),
                description: "All nine Raleway weights are tokenized. Use semantic weight names (--ratel-font-weight-bold) not numeric values in component code.".to_string(),

                div {
                    class: "rounded-ratel-xl border overflow-hidden",
                    style: "border-color: var(--ratel-color-border-stroke-neutral-800);",
                    for weight in WEIGHT_SCALE {
                        WeightRow { entry: weight, }
                    }
                }
            }

            // ── Title scale ───────────────────────────────────────────────
            DocSection {
                title: "Title Scale".to_string(),
                id:    "titles".to_string(),
                description: "Three display sizes for hero sections and marketing content. Title-1 is 64px; rarely used outside landing pages.".to_string(),

                div { class: "space-y-4",
                    for e in TITLE_SCALE {
                        TypeScaleRow {
                            label: e.label, css_size: e.css_size, px: e.px,
                            lh: e.lh, ls: e.ls, token: e.token, weight: e.weight,
                        }
                    }
                }
            }

            // ── Heading scale ─────────────────────────────────────────────
            DocSection {
                title: "Heading Scale".to_string(),
                id:    "headings".to_string(),
                description: "Four heading levels for page titles and section headers. H1–H4 map to 28px→20px with tight letter-spacing.".to_string(),

                div { class: "space-y-3",
                    for e in HEADING_SCALE {
                        TypeScaleRow {
                            label: e.label, css_size: e.css_size, px: e.px,
                            lh: e.lh, ls: e.ls, token: e.token, weight: e.weight,
                        }
                    }
                }
            }

            // ── Label scale ───────────────────────────────────────────────
            DocSection {
                title: "Label Scale".to_string(),
                id:    "labels".to_string(),
                description: "Five label sizes for UI text: button labels, form labels, navigation items, table headers. Label-3 (13px) is the most common UI size.".to_string(),

                div { class: "space-y-2",
                    for e in LABEL_SCALE {
                        TypeScaleRow {
                            label: e.label, css_size: e.css_size, px: e.px,
                            lh: e.lh, ls: e.ls, token: e.token, weight: e.weight,
                        }
                    }
                }
            }

            // ── Body scale ────────────────────────────────────────────────
            DocSection {
                title: "Body Scale".to_string(),
                id:    "body".to_string(),
                description: "Two body sizes for prose content. Body-1 (17px/28px) for primary reading; Body-2 (15px/22px) for denser UI copy.".to_string(),

                div { class: "space-y-3",
                    for e in BODY_SCALE {
                        TypeScaleRow {
                            label: e.label, css_size: e.css_size, px: e.px,
                            lh: e.lh, ls: e.ls, token: e.token, weight: e.weight,
                        }
                    }
                }
            }

            // ── Token table ───────────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                description: "All typography values as CSS custom properties. Generated from variants/ratel-brand/typography.json.".to_string(),

                TokenTable { rows: TYPO_TOKEN_ROWS.to_vec() }
            }

            // ── Do / Don't ────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: TYPO_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Weight row component ─────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct WeightEntry {
    name:    &'static str,
    weight:  &'static str,   // numeric weight string
    token:   &'static str,
}

static WEIGHT_SCALE: &[WeightEntry] = &[
    WeightEntry { name: "Thin",       weight: "100", token: "--ratel-font-weight-thin" },
    WeightEntry { name: "ExtraLight", weight: "200", token: "--ratel-font-weight-extralight" },
    WeightEntry { name: "Light",      weight: "300", token: "--ratel-font-weight-light" },
    WeightEntry { name: "Regular",    weight: "400", token: "--ratel-font-weight-regular" },
    WeightEntry { name: "Medium",     weight: "500", token: "--ratel-font-weight-medium" },
    WeightEntry { name: "SemiBold",   weight: "600", token: "--ratel-font-weight-semibold" },
    WeightEntry { name: "Bold",       weight: "700", token: "--ratel-font-weight-bold" },
    WeightEntry { name: "ExtraBold",  weight: "800", token: "--ratel-font-weight-extrabold" },
    WeightEntry { name: "Black",      weight: "900", token: "--ratel-font-weight-black" },
];

#[component]
fn WeightRow(entry: &'static WeightEntry) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between px-5 py-3",
            style: "border-bottom: 1px solid var(--ratel-color-border-stroke-neutral-800); background: var(--ratel-color-border-background-neutral-850);",
            span {
                class: "text-h3",
                style: "font-weight: {entry.weight}; color: var(--ratel-color-font-default); font-family: var(--ratel-font-family);",
                { entry.name }
            }
            div { class: "flex items-center gap-4",
                code {
                    class: "text-label-3 font-mono",
                    style: "color: var(--ratel-color-generic-primary);",
                    { entry.token }
                }
                span {
                    class: "text-label-3 w-8 text-right",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    { entry.weight }
                }
            }
        }
    }
}

// ─── Type scale row ───────────────────────────────────────────────────────────

struct TypeScaleEntry {
    label:    &'static str,   // e.g. "Title 1"
    css_size: &'static str,   // e.g. "var(--ratel-text-title-title-1-size)"
    px:       &'static str,   // e.g. "64px"
    lh:       &'static str,   // e.g. "70px"
    ls:       &'static str,   // e.g. "-0.8px"
    token:    &'static str,
    weight:   &'static str,   // CSS font-weight value
}

static TITLE_SCALE: &[TypeScaleEntry] = &[
    TypeScaleEntry { label: "Title 1", css_size: "var(--ratel-text-title-title-1-size)", px: "64px", lh: "70px", ls: "-0.8px",  token: "text-title-1", weight: "700" },
    TypeScaleEntry { label: "Title 2", css_size: "var(--ratel-text-title-title-2-size)", px: "40px", lh: "48px", ls: "-0.64px", token: "text-title-2", weight: "700" },
    TypeScaleEntry { label: "Title 3", css_size: "var(--ratel-text-title-title-3-size)", px: "32px", lh: "36px", ls: "-0.6px",  token: "text-title-3", weight: "700" },
];

static HEADING_SCALE: &[TypeScaleEntry] = &[
    TypeScaleEntry { label: "Heading H1", css_size: "var(--ratel-text-heading-h1-size)", px: "28px", lh: "32px", ls: "-0.56px", token: "text-h1", weight: "600" },
    TypeScaleEntry { label: "Heading H2", css_size: "var(--ratel-text-heading-h2-size)", px: "26px", lh: "30px", ls: "-0.26px", token: "text-h2", weight: "600" },
    TypeScaleEntry { label: "Heading H3", css_size: "var(--ratel-text-heading-h3-size)", px: "24px", lh: "28px", ls: "-0.24px", token: "text-h3", weight: "600" },
    TypeScaleEntry { label: "Heading H4", css_size: "var(--ratel-text-heading-h4-size)", px: "20px", lh: "24px", ls: "-0.2px",  token: "text-h4", weight: "600" },
];

static LABEL_SCALE: &[TypeScaleEntry] = &[
    TypeScaleEntry { label: "Label 1", css_size: "var(--ratel-text-label-label-1-size)", px: "17px", lh: "20px", ls: "-0.18px", token: "text-label-1", weight: "500" },
    TypeScaleEntry { label: "Label 2", css_size: "var(--ratel-text-label-label-2-size)", px: "15px", lh: "18px", ls: "-0.16px", token: "text-label-2", weight: "500" },
    TypeScaleEntry { label: "Label 3", css_size: "var(--ratel-text-label-label-3-size)", px: "13px", lh: "16px", ls: "-0.14px", token: "text-label-3", weight: "500" },
    TypeScaleEntry { label: "Label 4", css_size: "var(--ratel-text-label-label-4-size)", px: "12px", lh: "14px", ls: "-0.12px", token: "text-label-4", weight: "500" },
    TypeScaleEntry { label: "Label 5", css_size: "var(--ratel-text-label-label-5-size)", px: "11px", lh: "14px", ls: "-0.10px", token: "text-label-5", weight: "500" },
];

static BODY_SCALE: &[TypeScaleEntry] = &[
    TypeScaleEntry { label: "Body 1", css_size: "var(--ratel-text-body-body-1-size)", px: "17px", lh: "28px", ls: "0px", token: "text-body-1", weight: "400" },
    TypeScaleEntry { label: "Body 2", css_size: "var(--ratel-text-body-body-2-size)", px: "15px", lh: "22px", ls: "0px", token: "text-body-2", weight: "400" },
];

#[derive(Props, Clone, PartialEq)]
struct TypeScaleRowProps {
    label:    &'static str,
    css_size: &'static str,
    px:       &'static str,
    lh:       &'static str,
    ls:       &'static str,
    token:    &'static str,
    weight:   &'static str,
}

#[component]
fn TypeScaleRow(props: TypeScaleRowProps) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between gap-4 rounded-ratel-lg px-5 py-4 border",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800);",

            // Sample text
            span {
                class: "flex-1 min-w-0 truncate",
                style: "font-size: {props.css_size}; line-height: {props.lh}; letter-spacing: {props.ls}; font-weight: {props.weight}; color: var(--ratel-color-font-default); font-family: var(--ratel-font-family);",
                { props.label }
            }

            // Spec
            div { class: "flex items-center gap-5 shrink-0",
                SpecItem { label: "size", val: props.px }
                SpecItem { label: "lh",   val: props.lh }
                SpecItem { label: "ls",   val: props.ls }
                code {
                    class: "text-label-4 font-mono hidden sm:block",
                    style: "color: var(--ratel-color-generic-primary);",
                    { props.token }
                }
            }
        }
    }
}

#[component]
fn SpecItem(label: &'static str, val: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-end",
            span {
                class: "text-label-5",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { label }
            }
            span {
                class: "text-label-3 font-mono",
                style: "color: var(--ratel-color-font-body);",
                { val }
            }
        }
    }
}

static TYPO_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-font-family",              "Raleway, sans-serif", "Primary font family"),
    TokenRow::new("--ratel-font-weight-thin",         "100",                  "Thin weight"),
    TokenRow::new("--ratel-font-weight-regular",      "400",                  "Regular weight"),
    TokenRow::new("--ratel-font-weight-medium",       "500",                  "Medium weight (default for labels)"),
    TokenRow::new("--ratel-font-weight-semibold",     "600",                  "SemiBold (headings)"),
    TokenRow::new("--ratel-font-weight-bold",         "700",                  "Bold (titles)"),
    TokenRow::new("--ratel-text-title-title-1-size",  "64px",                 "Title 1 font size"),
    TokenRow::new("--ratel-text-title-title-1-lh",    "70px",                 "Title 1 line height"),
    TokenRow::new("--ratel-text-title-title-1-ls",    "-0.8px",               "Title 1 letter spacing"),
    TokenRow::new("--ratel-text-heading-h1-size",     "28px",                 "Heading H1 font size"),
    TokenRow::new("--ratel-text-heading-h1-lh",       "32px",                 "Heading H1 line height"),
    TokenRow::new("--ratel-text-label-label-3-size",  "13px",                 "Label 3 font size (most common UI size)"),
    TokenRow::new("--ratel-text-body-body-1-size",    "17px",                 "Body 1 font size"),
    TokenRow::new("--ratel-text-body-body-1-lh",      "28px",                 "Body 1 line height (relaxed reading)"),
];

static TYPO_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Use Tailwind text-* aliases",
        "Write class=\"text-h2 font-semibold\" — Tailwind maps text-h2 to font-size, line-height, and letter-spacing tokens from typography.json."
    ),
    DoDont::dont(
        "Set font sizes in raw pixels",
        "Avoid style=\"font-size: 26px\". Always use the token alias (text-h2) so the value stays in sync with the token file and is theme-aware."
    ),
    DoDont::do_(
        "Pair labels with semibold weight",
        "Navigation links, form labels, and button text should be font-medium (500) or font-semibold (600) to maintain legibility at small sizes."
    ),
    DoDont::dont(
        "Use font families other than Raleway",
        "All text in the Ratel system uses Raleway (--ratel-font-family). Do not introduce secondary typefaces without updating the token file."
    ),
];
