use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, CodeBlock};
use crate::router::Route;

#[component]
pub fn Overview() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Ratel Design System".to_string(),
                subtitle: "A token-driven, Dioxus-native component library for building consistent, accessible web interfaces. All values are sourced from local design token files — no hardcoded colors, spacings, or typography.".to_string(),
                badge:    "v0.1 · Phase 2".to_string(),
            }

            // ── Quick start ───────────────────────────────────────────────
            DocSection {
                title: "Quick Start".to_string(),
                description: "Add ratel-ds as a Cargo dependency and import the stylesheet. All component classes are backed by CSS custom properties defined in tokens.css.".to_string(),

                CodeBlock {
                    lang: "toml".to_string(),
                    title: "Cargo.toml".to_string(),
                    code: r#"[dependencies]
ratel-ds = { path = "../ratel-ds" }
dioxus   = { version = "0.6", features = ["web", "router"] }"#.to_string(),
                }
            }

            // ── Token pipeline ────────────────────────────────────────────
            DocSection {
                title: "Token Pipeline".to_string(),
                description: "Design tokens flow from Figma variable exports → local JSON files → generated CSS custom properties → Tailwind utility classes.".to_string(),

                div { class: "rounded-ratel-xl border overflow-hidden",
                    style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

                    div { class: "p-6",
                        style: "background: var(--ratel-color-border-background-neutral-850);",

                        // Pipeline diagram
                        div { class: "flex flex-wrap items-center gap-2",
                            for (i, step) in PIPELINE_STEPS.iter().enumerate() {
                                PipelineStep {
                                    label:       step.0,
                                    description: step.1,
                                    color:       step.2,
                                }
                                if i < PIPELINE_STEPS.len() - 1 {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-4 h-4 shrink-0",
                                        style: "color: var(--ratel-color-font-neutral-absolute);",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M9 5l7 7-7 7",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Token sources ─────────────────────────────────────────────
            DocSection {
                title: "Token Sources".to_string(),
                description: "All design decisions are encoded in the following source files. Run `node scripts/build-tokens.js` to regenerate tokens.css after changes.".to_string(),

                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                    for file in TOKEN_FILES {
                        TokenFileCard { info: file }
                    }
                }
            }

            // ── What's next ───────────────────────────────────────────────
            DocSection {
                title: "What's in Phase 2".to_string(),
                description: "This phase establishes the full documentation infrastructure. Phase 3 will bring live component implementations.".to_string(),

                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    for item in PHASE_ITEMS {
                        PhaseCard { item }
                    }
                }
            }

            // ── Navigate ──────────────────────────────────────────────────
            DocSection {
                title: "Explore the System".to_string(),
                description: "Use the sidebar to browse foundations and components. Each page shows token references, usage guidelines, and code examples.".to_string(),

                div { class: "flex flex-wrap gap-3",
                    QuickLink { label: "Colors",       to: Route::Colors,     color: "var(--ratel-color-generic-primary)" }
                    QuickLink { label: "Typography",   to: Route::Typography, color: "var(--ratel-color-generic-info)" }
                    QuickLink { label: "Spacing",      to: Route::Spacing,    color: "var(--ratel-color-generic-success)" }
                    QuickLink { label: "Components →", to: Route::ComponentsOverview, color: "var(--ratel-color-generic-primary)" }
                }
            }
        }
    }
}

// ─── Pipeline step ────────────────────────────────────────────────────────────

static PIPELINE_STEPS: &[(&str, &str, &str)] = &[
    ("Figma Variables", "Source exported as JSON", "var(--ratel-color-generic-info)"),
    ("variants/ JSON",  "Local token files",       "var(--ratel-color-generic-primary)"),
    ("tokens.css",      "CSS custom properties",   "var(--ratel-color-generic-success)"),
    ("tailwind.config", "Utility class aliases",   "var(--ratel-color-generic-primary)"),
    ("Dioxus RSX",      "Component classes",        "var(--ratel-color-font-neutral-absolute)"),
];

#[derive(Props, Clone, PartialEq)]
struct PipelineStepProps {
    label:       &'static str,
    description: &'static str,
    color:       &'static str,
}

#[component]
fn PipelineStep(props: PipelineStepProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1 px-3 py-2 rounded-ratel-lg border",
            style: "background: var(--ratel-color-border-incard-background-default); border-color: var(--ratel-color-border-stroke-neutral-800);",

            span {
                class: "text-label-3 font-semibold",
                style: "color: {props.color};",
                { props.label }
            }
            span {
                class: "text-label-4",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { props.description }
            }
        }
    }
}

// ─── Token file card ──────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct TokenFileInfo {
    path:        &'static str,
    description: &'static str,
    kind:        &'static str,
}

static TOKEN_FILES: &[TokenFileInfo] = &[
    TokenFileInfo { path: "variants/primitive/primitive-color.json", description: "Raw color palette: white, black, neutral, brand, status scales", kind: "Primitive" },
    TokenFileInfo { path: "variants/primitive/radius.json",          description: "Border-radius scale: xs(2px) → full(9999px)", kind: "Primitive" },
    TokenFileInfo { path: "variants/primitive/stroke.json",          description: "Stroke/border-width scale: 0.5px → 3px", kind: "Primitive" },
    TokenFileInfo { path: "variants/primitive/token.json",           description: "Numeric spacing scale: 0 → 1536px", kind: "Primitive" },
    TokenFileInfo { path: "variants/primitive/margin.json",          description: "Margin token aliases (references spacing scale)", kind: "Primitive" },
    TokenFileInfo { path: "variants/primitive/padding.json",         description: "Padding token aliases (references spacing scale)", kind: "Primitive" },
    TokenFileInfo { path: "variants/ratel-brand/color/Light.tokens.json", description: "Semantic light-theme colors: font, border, surface, status", kind: "Semantic" },
    TokenFileInfo { path: "variants/ratel-brand/color/Dark.tokens.json",  description: "Semantic dark-theme colors: mirrors light with dark values",  kind: "Semantic" },
    TokenFileInfo { path: "variants/ratel-brand/typography.json",         description: "Type scale + weights: Raleway, Title→Body, 100→900",           kind: "Semantic" },
];

#[derive(Props, Clone, PartialEq)]
struct TokenFileCardProps {
    info: &'static TokenFileInfo,
}

#[component]
fn TokenFileCard(props: TokenFileCardProps) -> Element {
    let kind_color = if props.info.kind == "Primitive" {
        "var(--ratel-color-generic-info)"
    } else {
        "var(--ratel-color-generic-primary)"
    };

    rsx! {
        div {
            class: "rounded-ratel-lg p-4 border",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800);",

            div { class: "flex items-start justify-between gap-2 mb-1.5",
                code {
                    class: "text-label-4 font-mono block",
                    style: "color: var(--ratel-color-font-body);",
                    { props.info.path }
                }
                span {
                    class: "text-label-5 px-1.5 py-0.5 rounded-ratel-xs shrink-0",
                    style: "background: var(--ratel-color-border-incard-background-default); color: {kind_color};",
                    { props.info.kind }
                }
            }
            p {
                class: "text-label-3",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { props.info.description }
            }
        }
    }
}

// ─── Phase checklist cards ────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct PhaseItem {
    title:    &'static str,
    desc:     &'static str,
    done:     bool,
}

static PHASE_ITEMS: &[PhaseItem] = &[
    PhaseItem { title: "Token build script",    desc: "scripts/build-tokens.js reads all variants/ JSON and emits tokens.css + token-manifest.json", done: true },
    PhaseItem { title: "CSS custom properties", desc: "tokens.css generated from all token files — primitive + semantic light/dark", done: true },
    PhaseItem { title: "Tailwind integration",  desc: "tailwind.config.js maps all token CSS vars to named utilities (ratel-primary, ratel-radius-lg, …)", done: true },
    PhaseItem { title: "Routing & layout",      desc: "Dioxus Router with Shell + Sidebar + content area, theme toggle, 404 page", done: true },
    PhaseItem { title: "Docs infrastructure",   desc: "DocSection, ComponentPreview, CodeBlock, TokenTable, ColorSwatch, DoDont components", done: true },
    PhaseItem { title: "Foundation pages",      desc: "Colors, Typography, Spacing, Radius, Stroke, Shadows — all token-referenced", done: true },
    PhaseItem { title: "UI Components (Phase 3)", desc: "Button, Badge, Input, Card — live previews + variant tables + code examples", done: false },
    PhaseItem { title: "Figma Component Import", desc: "Node 10033:20598 component implementation once Figma access is available", done: false },
];

#[derive(Props, Clone, PartialEq)]
struct PhaseCardProps {
    item: &'static PhaseItem,
}

#[component]
fn PhaseCard(props: PhaseCardProps) -> Element {
    rsx! {
        div {
            class: "flex gap-3 p-4 rounded-ratel-lg border",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800);",

            // Check/circle
            div {
                class: "w-5 h-5 rounded-full shrink-0 mt-0.5 flex items-center justify-center",
                style: if props.item.done {
                    "background: var(--ratel-color-generic-success);"
                } else {
                    "background: var(--ratel-color-border-incard-background-default); border: 1.5px solid var(--ratel-color-border-stroke-neutral-800);"
                },
                if props.item.done {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-3 h-3",
                        style: "color: white;",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "3",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M5 13l4 4L19 7" }
                    }
                }
            }

            div {
                div {
                    class: "text-label-2 font-semibold mb-0.5",
                    style: "color: var(--ratel-color-font-default);",
                    { props.item.title }
                }
                div {
                    class: "text-label-3",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    { props.item.desc }
                }
            }
        }
    }
}

// ─── Quick link button ────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct QuickLinkProps {
    label: &'static str,
    to:    Route,
    color: &'static str,
}

#[component]
fn QuickLink(props: QuickLinkProps) -> Element {
    rsx! {
        Link {
            to: props.to.clone(),
            class: "inline-flex items-center gap-2 px-4 py-2 rounded-ratel-lg border text-label-2 font-medium transition-colors duration-150",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800); color: {props.color};",
            { props.label }
        }
    }
}
