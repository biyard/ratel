use dioxus::prelude::*;
use crate::components::docs::{PageIntro, DocSection, CodeBlock};

// ─── Component Registry ───────────────────────────────────────────────────────
// Each entry represents a planned component. Implemented entries will link
// to their dedicated page in Phase 3. Unimplemented entries show a "planned"
// badge and the expected token surface.

#[derive(Clone, PartialEq)]
struct ComponentEntry {
    name:        &'static str,
    description: &'static str,
    tokens:      &'static [&'static str],
    status:      ComponentStatus,
    figma_node:  Option<&'static str>,
}

#[derive(PartialEq, Clone, Copy)]
enum ComponentStatus {
    Planned,
    // InProgress,  // used in Phase 3
    // Stable,      // used in Phase 3
}

static COMPONENT_REGISTRY: &[ComponentEntry] = &[
    ComponentEntry {
        name:        "Button",
        description: "Primary interactive element. Supports primary, secondary, outline, ghost, and destructive variants at sm/md/lg sizes with disabled and loading states.",
        tokens:      &["--ratel-color-generic-primary", "--ratel-radius-lg", "--ratel-stroke-2", "--ratel-text-label-label-2-*"],
        status:      ComponentStatus::Planned,
        figma_node:  Some("10033:20598"),
    },
    ComponentEntry {
        name:        "Badge",
        description: "Non-interactive label for status, category, or count. Variants: default, success, warning, error, info.",
        tokens:      &["--ratel-color-generic-*", "--ratel-radius-full", "--ratel-text-label-label-4-*"],
        status:      ComponentStatus::Planned,
        figma_node:  Some("10033:20598"),
    },
    ComponentEntry {
        name:        "Input",
        description: "Text input with label, helper text, and validation states: default, focused, error, disabled.",
        tokens:      &["--ratel-color-border-stroke-*", "--ratel-radius-md", "--ratel-stroke-1", "--ratel-stroke-2"],
        status:      ComponentStatus::Planned,
        figma_node:  Some("10033:20598"),
    },
    ComponentEntry {
        name:        "Card",
        description: "Surface container for grouping related content. Variants: default, bordered, elevated, interactive.",
        tokens:      &["--ratel-color-border-background-*", "--ratel-radius-xl", "--ratel-stroke-1"],
        status:      ComponentStatus::Planned,
        figma_node:  Some("10033:20598"),
    },
    ComponentEntry {
        name:        "Select",
        description: "Dropdown selection control. Extends Input visually with a trigger and floating listbox.",
        tokens:      &["--ratel-color-border-stroke-*", "--ratel-radius-md", "--ratel-radius-xl"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
    ComponentEntry {
        name:        "Checkbox",
        description: "Binary toggle for form selections. Includes indeterminate state.",
        tokens:      &["--ratel-color-generic-primary", "--ratel-radius-xs", "--ratel-stroke-2"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
    ComponentEntry {
        name:        "Toggle",
        description: "Switch between two states. Pill shape using rounded-ratel-full.",
        tokens:      &["--ratel-color-generic-primary", "--ratel-radius-full"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
    ComponentEntry {
        name:        "Avatar",
        description: "User representation with image, initials, or icon fallback.",
        tokens:      &["--ratel-radius-full", "--ratel-space-*"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
    ComponentEntry {
        name:        "Tooltip",
        description: "Hover/focus triggered overlay for contextual information.",
        tokens:      &["--ratel-color-font-invert-*", "--ratel-radius-sm"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
    ComponentEntry {
        name:        "Modal",
        description: "Focus-trapped overlay dialog. Uses shadow-lg elevation.",
        tokens:      &["--ratel-radius-2xl", "shadow-lg", "--ratel-stroke-1"],
        status:      ComponentStatus::Planned,
        figma_node:  None,
    },
];

// ─── Page ─────────────────────────────────────────────────────────────────────

#[component]
pub fn ComponentsOverview() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Components".to_string(),
                subtitle: "All Ratel UI components are built with token-backed Tailwind classes and Dioxus RSX. Each component page includes a live preview, variant table, code example, and token reference. Components are implemented in Phase 3.".to_string(),
                badge:    "Phase 3 →".to_string(),
            }

            // ── Registry ──────────────────────────────────────────────────
            DocSection {
                title: "Component Registry".to_string(),
                id:    "registry".to_string(),
                description: "All planned components with their token surface. Figma node references link to the source design in 4ZMKUI92jk8yJRLsF3ERwD once access is available.".to_string(),

                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    for entry in COMPONENT_REGISTRY {
                        ComponentCard { entry }
                    }
                }
            }

            // ── Architecture note ─────────────────────────────────────────
            DocSection {
                title: "Component Architecture".to_string(),
                id:    "architecture".to_string(),
                description: "Each Phase 3 component follows this structure in src/components/ui/.".to_string(),

                CodeBlock {
                    lang:  "rust".to_string(),
                    title: "src/components/ui/button.rs (Phase 3 template)".to_string(),
                    code:  r#"use dioxus::prelude::*;

/// Button variant tokens map to these Tailwind utility groups:
///   primary  → bg-ratel-primary text-[#0A0A0A] hover:bg-ratel-primary-75
///   outline  → border-ratel-1 border-ratel-stroke-pri text-ratel-primary
///   ghost    → text-ratel-primary hover:bg-ratel-primary-5
///   disabled → text-ratel-text-dis cursor-not-allowed opacity-60

#[derive(Clone, PartialEq, Default)]
pub enum ButtonVariant { #[default] Primary, Secondary, Outline, Ghost, Destructive }

#[derive(Clone, PartialEq, Default)]
pub enum ButtonSize { Sm, #[default] Md, Lg }

#[component]
pub fn Button(
    variant: ButtonVariant,
    size:    ButtonSize,
    #[props(optional)] disabled: bool,
    children: Element,
) -> Element {
    // Token-backed class selection:
    // radius  → rounded-ratel-lg (from --ratel-radius-lg: 8px)
    // stroke  → border-ratel-1   (from --ratel-stroke-1: 1px)
    // spacing → px/py from --ratel-space-* steps
    rsx! {
        button {
            class: "/* resolved from variant + size */",
            disabled,
            { children }
        }
    }
}"#.to_string(),
                }
            }

            // ── Figma integration note ────────────────────────────────────
            DocSection {
                title: "Figma Integration Status".to_string(),
                id:    "figma".to_string(),
                description: "The source Figma file contains the component specifications at node 10033:20598.".to_string(),

                div {
                    class: "rounded-ratel-xl border p-5",
                    style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800);",

                    div { class: "flex items-start gap-4",
                        // Status icon
                        div {
                            class: "w-8 h-8 rounded-ratel-md shrink-0 flex items-center justify-center mt-0.5",
                            style: "background: var(--ratel-color-generic-info-opacity-10%);",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "w-4 h-4",
                                style: "color: var(--ratel-color-generic-info);",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                }
                            }
                        }

                        div {
                            div {
                                class: "text-label-2 font-semibold mb-1",
                                style: "color: var(--ratel-color-font-default);",
                                "Figma access required for component extraction"
                            }
                            p {
                                class: "text-label-3",
                                style: "color: var(--ratel-color-font-body);",
                                "File: 4ZMKUI92jk8yJRLsF3ERwD · Node: 10033:20598"
                            }
                            p {
                                class: "text-label-3 mt-1",
                                style: "color: var(--ratel-color-font-neutral-absolute);",
                                "When access is granted, run execution/fetch_figma_data.js to extract component specs. The architecture is ready to receive Figma-derived variant data without restructuring."
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Component card ───────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct ComponentCardProps {
    entry: &'static ComponentEntry,
}

#[component]
fn ComponentCard(props: ComponentCardProps) -> Element {
    let e = props.entry;
    rsx! {
        div {
            class: "rounded-ratel-xl border p-5",
            style: "background: var(--ratel-color-border-background-neutral-850); border-color: var(--ratel-color-border-stroke-neutral-800);",

            // Header
            div { class: "flex items-start justify-between gap-3 mb-2",
                span {
                    class: "text-label-1 font-semibold",
                    style: "color: var(--ratel-color-font-default);",
                    { e.name }
                }
                span {
                    class: "text-label-4 px-2 py-0.5 rounded-ratel-xs shrink-0",
                    style: "background: var(--ratel-color-border-incard-background-default); color: var(--ratel-color-font-neutral-absolute);",
                    "Planned · Phase 3"
                }
            }

            // Description
            p {
                class: "text-label-3 mb-3",
                style: "color: var(--ratel-color-font-body);",
                { e.description }
            }

            // Tokens used
            div { class: "flex flex-wrap gap-1.5",
                for &tok in e.tokens {
                    span { class: "ds-token-pill", { tok } }
                }
            }

            // Figma link
            if let Some(node) = e.figma_node {
                div {
                    class: "mt-3 flex items-center gap-1.5 text-label-4",
                    style: "color: var(--ratel-color-font-neutral-absolute);",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-3 h-3",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M5 5.5A3.5 3.5 0 018.5 2H12v7H8.5A3.5 3.5 0 015 5.5zM12 2h3.5a3.5 3.5 0 110 7H12V2z" }
                    }
                    span { "Figma node: " }
                    code {
                        class: "font-mono",
                        style: "color: var(--ratel-color-generic-primary);",
                        { node }
                    }
                }
            }
        }
    }
}
