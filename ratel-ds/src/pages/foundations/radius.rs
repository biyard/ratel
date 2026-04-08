use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, TokenRow, TokenTable, DoDont, DoDontGrid};

#[component]
pub fn Radius() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Border Radius".to_string(),
                subtitle: "The Ratel radius scale maps semantic size names to pixel values from radius.json. Use rounded-ratel-* Tailwind classes or var(--ratel-radius-*) custom properties — never hardcode a corner radius.".to_string(),
                badge:    "Foundation".to_string(),
            }

            // ── Visual scale ──────────────────────────────────────────────
            DocSection {
                title: "Radius Scale".to_string(),
                id:    "scale".to_string(),
                description: "Nine steps from none (0) to full (9999px) pill shape. Each step is represented as a square preview tile.".to_string(),

                div { class: "grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4",
                    for step in RADIUS_SCALE {
                        RadiusTile { entry: step }
                    }
                }
            }

            // ── Token table ───────────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                TokenTable { rows: RADIUS_TOKEN_ROWS.to_vec() }
            }

            // ── Guidelines ────────────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                description: "Match radius to component size: small components use smaller radii; large containers or cards use larger radii.".to_string(),

                UsageTable {}

                div { class: "mt-6",
                    DoDontGrid { items: RADIUS_DO_DONT.to_vec() }
                }
            }
        }
    }
}

// ─── Radius tile ──────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct RadiusEntry {
    name:    &'static str,   // e.g. "none"
    px:      &'static str,   // e.g. "0px"
    token:   &'static str,   // e.g. "--ratel-radius-none"
    tw:      &'static str,   // e.g. "rounded-ratel-none"
}

static RADIUS_SCALE: &[RadiusEntry] = &[
    RadiusEntry { name: "none", px: "0px",    token: "--ratel-radius-none", tw: "rounded-ratel-none" },
    RadiusEntry { name: "xs",   px: "2px",    token: "--ratel-radius-xs",   tw: "rounded-ratel-xs" },
    RadiusEntry { name: "sm",   px: "4px",    token: "--ratel-radius-sm",   tw: "rounded-ratel-sm" },
    RadiusEntry { name: "md",   px: "6px",    token: "--ratel-radius-md",   tw: "rounded-ratel-md" },
    RadiusEntry { name: "lg",   px: "8px",    token: "--ratel-radius-lg",   tw: "rounded-ratel-lg" },
    RadiusEntry { name: "xl",   px: "12px",   token: "--ratel-radius-xl",   tw: "rounded-ratel-xl" },
    RadiusEntry { name: "2xl",  px: "16px",   token: "--ratel-radius-2xl",  tw: "rounded-ratel-2xl" },
    RadiusEntry { name: "3xl",  px: "24px",   token: "--ratel-radius-3xl",  tw: "rounded-ratel-3xl" },
    RadiusEntry { name: "4xl",  px: "32px",   token: "--ratel-radius-4xl",  tw: "rounded-ratel-4xl" },
    RadiusEntry { name: "full", px: "9999px", token: "--ratel-radius-full", tw: "rounded-ratel-full" },
];

#[derive(Props, Clone, PartialEq)]
struct RadiusTileProps { entry: &'static RadiusEntry }

#[component]
fn RadiusTile(props: RadiusTileProps) -> Element {
    let e = props.entry;
    let is_full = e.name == "full";
    rsx! {
        div {
            class: "flex flex-col gap-3 p-4 border",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800); border-radius: var(--ratel-radius-xl);",

            // Preview square
            div {
                class: "mx-auto",
                style: if is_full {
                    "width: 64px; height: 32px; background: var(--ratel-color-generic-primary-opacity-25%); border: 2px solid var(--ratel-color-generic-primary); border-radius: 9999px;".to_string()
                } else {
                    format!("width: 64px; height: 64px; background: var(--ratel-color-generic-primary-opacity-25%); border: 2px solid var(--ratel-color-generic-primary); border-radius: {};", e.px)
                },
            }

            // Labels
            div { class: "text-center",
                div {
                    class: "text-label-2 font-semibold",
                    style: "color: var(--ratel-color-font-default);",
                    { e.name }
                }
                div {
                    class: "text-label-4 font-mono",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    { e.px }
                }
            }
        }
    }
}

// ─── Token table data ─────────────────────────────────────────────────────────

static RADIUS_TOKEN_ROWS: &[TokenRow] = &[
    TokenRow::new("--ratel-radius-none", "0px",    "Square corners — tables, code blocks, banners"),
    TokenRow::new("--ratel-radius-xs",   "2px",    "Minimal rounding — inline badges, tooltips"),
    TokenRow::new("--ratel-radius-sm",   "4px",    "Small rounding — tags, chips, small buttons"),
    TokenRow::new("--ratel-radius-md",   "6px",    "Default input radius"),
    TokenRow::new("--ratel-radius-lg",   "8px",    "Default button radius, nav items"),
    TokenRow::new("--ratel-radius-xl",   "12px",   "Cards, modals, dropdowns, panels"),
    TokenRow::new("--ratel-radius-2xl",  "16px",   "Large cards, dialogs, bottom sheets"),
    TokenRow::new("--ratel-radius-3xl",  "24px",   "Feature cards, expanded containers"),
    TokenRow::new("--ratel-radius-4xl",  "32px",   "Extra-large containers, sidebars"),
    TokenRow::new("--ratel-radius-full", "9999px", "Pill shape — toggles, avatar badges, status dots"),
];

// ─── Usage table ──────────────────────────────────────────────────────────────

#[component]
fn UsageTable() -> Element {
    static USAGE: &[(&str, &str, &str)] = &[
        ("Inline badge / chip",  "rounded-ratel-xs or rounded-ratel-sm", "2–4px"),
        ("Input field",          "rounded-ratel-md",                     "6px"),
        ("Button",               "rounded-ratel-lg",                     "8px"),
        ("Dropdown / menu",      "rounded-ratel-xl",                     "12px"),
        ("Card / modal",         "rounded-ratel-xl or rounded-ratel-2xl","12–16px"),
        ("Large panel / drawer", "rounded-ratel-3xl",                    "24px"),
        ("Toggle / pill button", "rounded-ratel-full",                   "9999px"),
    ];

    rsx! {
        div {
            class: "rounded-ratel-lg overflow-hidden border",
            style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

            table { class: "ds-token-table w-full",
                thead {
                    tr {
                        th { "Component" }
                        th { "Class" }
                        th { "Radius" }
                    }
                }
                tbody {
                    for &(component, class, radius) in USAGE {
                        tr {
                            td { { component } }
                            td { code { class: "ds-token-pill", { class } } }
                            td {
                                code {
                                    class: "text-label-3 font-mono",
                                    style: "color: var(--ratel-color-font-body);",
                                    { radius }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

static RADIUS_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Scale radius with component size",
        "Small badges use rounded-ratel-xs; full-size cards use rounded-ratel-xl. Maintaining this ratio preserves visual consistency."
    ),
    DoDont::dont(
        "Mix radius token levels within one component",
        "Don't apply rounded-ratel-3xl to a button or rounded-ratel-none to a card. Each component has a defined tier — follow the usage table."
    ),
];
