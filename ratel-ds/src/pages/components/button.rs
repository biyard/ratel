use dioxus::prelude::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, ButtonRadius};
use crate::components::docs::{
    PageIntro, DocSection, SubSection,
    ComponentPreview,
    CodeBlock, TokenRow, TokenTable,
    DoDont, DoDontGrid,
};

// ─── Page ─────────────────────────────────────────────────────────────────────

#[component]
pub fn ButtonDocs() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Button".to_string(),
                subtitle: "The primary interactive element in the Ratel system. Four variants × four sizes × four radius modes — every combination is token-driven. Square and round radii apply to both text and icon-only buttons.".to_string(),
                badge:    "Phase 3 · Stable".to_string(),
            }

            // ── Variant overview ──────────────────────────────────────────
            DocSection {
                title: "Variants".to_string(),
                id:    "variants".to_string(),
                description: "Four variants cover the full visual weight range. Primary is the only filled brand-color button — use it once per view.".to_string(),

                div { class: "space-y-3",
                    VariantMatrix {}
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Button, ButtonVariant};

rsx! {
    Button { "Save" }                                         // Primary (default)
    Button { variant: ButtonVariant::Secondary, "Back" }
    Button { variant: ButtonVariant::Outline,   "Cancel" }
    Button { variant: ButtonVariant::Ghost,     "Learn more" }
}"#.to_string(),
                    }
                }
            }

            // ── Radius ────────────────────────────────────────────────────
            DocSection {
                title: "Radius".to_string(),
                id:    "radius".to_string(),
                description: "Square uses a per-size fixed radius. Round uses a full pill (9999px). Both modes are available for text and icon-only buttons.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "square / round — all variants at md size".to_string(),
                        canvas_class: "flex-col items-start gap-5".to_string(),

                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "square" }
                            Button { variant: ButtonVariant::Primary,   "Primary" }
                            Button { variant: ButtonVariant::Secondary, "Secondary" }
                            Button { variant: ButtonVariant::Outline,   "Outline" }
                            Button { variant: ButtonVariant::Ghost,     "Ghost" }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "round" }
                            Button { variant: ButtonVariant::Primary,   radius: ButtonRadius::Round, "Primary" }
                            Button { variant: ButtonVariant::Secondary, radius: ButtonRadius::Round, "Secondary" }
                            Button { variant: ButtonVariant::Outline,   radius: ButtonRadius::Round, "Outline" }
                            Button { variant: ButtonVariant::Ghost,     radius: ButtonRadius::Round, "Ghost" }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Button, ButtonRadius};

rsx! {
    // Square (default) — per-size fixed radius
    Button { "Save" }

    // Round pill
    Button { radius: ButtonRadius::Round, "Subscribe" }
}"#.to_string(),
                    }
                }
            }

            // ── Sizes ─────────────────────────────────────────────────────
            DocSection {
                title: "Sizes".to_string(),
                id:    "sizes".to_string(),
                description: "Four sizes map to the Figma Web/Btn type scale. Xs uses SemiBold 600; all others use Bold 700.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "xs / sm / md / lg — Primary square".to_string(),
                        div { class: "flex items-end gap-3 flex-wrap",
                            Button { size: ButtonSize::Xs, "X-Small" }
                            Button { size: ButtonSize::Sm, "Small" }
                            Button { size: ButtonSize::Md, "Medium" }
                            Button { size: ButtonSize::Lg, "Large" }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Button, ButtonSize};

rsx! {
    Button { size: ButtonSize::Xs, "X-Small" }  // 12px SemiBold · 4/8px pad
    Button { size: ButtonSize::Sm, "Small" }    // 14px Bold    · 8/16px pad
    Button { size: ButtonSize::Md, "Medium" }   // 14px Bold    · 12/20px pad (default)
    Button { size: ButtonSize::Lg, "Large" }    // 16px Bold    · 12/25px pad
}"#.to_string(),
                    }
                }
            }

            // ── Icon buttons ──────────────────────────────────────────────
            DocSection {
                title: "Icon Buttons".to_string(),
                id:    "icon".to_string(),
                description: "Icon-only buttons use fixed square dimensions per size. SquareIcon keeps per-size radius; RoundIcon becomes a circle. Pass any SVG as children.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "icon-square / icon-round — all variants at md".to_string(),
                        canvas_class: "flex-col items-start gap-5".to_string(),

                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-24 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "square icon" }
                            Button { variant: ButtonVariant::Primary,   radius: ButtonRadius::SquareIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Secondary, radius: ButtonRadius::SquareIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Outline,   radius: ButtonRadius::SquareIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Ghost,     radius: ButtonRadius::SquareIcon, IconPlus {} }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-24 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "round icon" }
                            Button { variant: ButtonVariant::Primary,   radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Secondary, radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Outline,   radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { variant: ButtonVariant::Ghost,     radius: ButtonRadius::RoundIcon, IconPlus {} }
                        }

                        // All icon sizes
                        div { class: "flex items-center gap-3 flex-wrap mt-2",
                            span { class: "text-label-3 shrink-0 w-24 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "sizes" }
                            Button { size: ButtonSize::Xs, radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { size: ButtonSize::Sm, radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { size: ButtonSize::Md, radius: ButtonRadius::RoundIcon, IconPlus {} }
                            Button { size: ButtonSize::Lg, radius: ButtonRadius::RoundIcon, IconPlus {} }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Button, ButtonRadius, ButtonSize};

rsx! {
    // Square icon — md (44×44)
    Button {
        radius: ButtonRadius::SquareIcon,
        svg { /* your icon SVG */ }
    }

    // Circle icon — sm (32×32)
    Button {
        radius: ButtonRadius::RoundIcon,
        size:   ButtonSize::Sm,
        svg { /* your icon SVG */ }
    }
}"#.to_string(),
                    }
                }
            }

            // ── With Icons ────────────────────────────────────────────────
            DocSection {
                title: "With Icons".to_string(),
                id:    "icons".to_string(),
                description: "Use leading_icon and trailing_icon props to place Phosphor icons beside the label. Icons scale with button size via font-size.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "leading / trailing / both — all variants".to_string(),
                        canvas_class: "flex-col items-start gap-5".to_string(),

                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "leading" }
                            Button { variant: ButtonVariant::Primary,
                                leading_icon: rsx! { i { class: "ph ph-plus" } },
                                "Add Item"
                            }
                            Button { variant: ButtonVariant::Secondary,
                                leading_icon: rsx! { i { class: "ph ph-download-simple" } },
                                "Download"
                            }
                            Button { variant: ButtonVariant::Outline,
                                leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
                                "Search"
                            }
                            Button { variant: ButtonVariant::Ghost,
                                leading_icon: rsx! { i { class: "ph ph-arrow-left" } },
                                "Back"
                            }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "trailing" }
                            Button { variant: ButtonVariant::Primary,
                                trailing_icon: rsx! { i { class: "ph ph-arrow-right" } },
                                "Continue"
                            }
                            Button { variant: ButtonVariant::Secondary,
                                trailing_icon: rsx! { i { class: "ph ph-caret-right" } },
                                "Next"
                            }
                            Button { variant: ButtonVariant::Outline,
                                trailing_icon: rsx! { i { class: "ph ph-upload-simple" } },
                                "Export"
                            }
                            Button { variant: ButtonVariant::Ghost,
                                trailing_icon: rsx! { i { class: "ph ph-x" } },
                                "Dismiss"
                            }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "both" }
                            Button { variant: ButtonVariant::Primary,
                                leading_icon:  rsx! { i { class: "ph ph-pencil-simple" } },
                                trailing_icon: rsx! { i { class: "ph ph-caret-down" } },
                                "Edit"
                            }
                            Button { variant: ButtonVariant::Outline,
                                leading_icon:  rsx! { i { class: "ph ph-share-network" } },
                                trailing_icon: rsx! { i { class: "ph ph-caret-down" } },
                                "Share"
                            }
                        }
                        // Sizes with icon
                        div { class: "flex items-end gap-3 flex-wrap mt-2",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "sizes" }
                            Button { size: ButtonSize::Xs,
                                leading_icon: rsx! { i { class: "ph ph-plus" } },
                                "XSmall"
                            }
                            Button { size: ButtonSize::Sm,
                                leading_icon: rsx! { i { class: "ph ph-plus" } },
                                "Small"
                            }
                            Button { size: ButtonSize::Md,
                                leading_icon: rsx! { i { class: "ph ph-plus" } },
                                "Medium"
                            }
                            Button { size: ButtonSize::Lg,
                                leading_icon: rsx! { i { class: "ph ph-plus" } },
                                "Large"
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"rsx! {
    // Leading icon
    Button {
        leading_icon: rsx! { i { class: "ph ph-plus" } },
        "Add Item"
    }

    // Trailing icon
    Button {
        variant:       ButtonVariant::Outline,
        trailing_icon: rsx! { i { class: "ph ph-arrow-right" } },
        "Continue"
    }

    // Both icons
    Button {
        leading_icon:  rsx! { i { class: "ph ph-pencil-simple" } },
        trailing_icon: rsx! { i { class: "ph ph-caret-down" } },
        "Edit"
    }
}"#.to_string(),
                    }
                }
            }

            // ── States ────────────────────────────────────────────────────
            DocSection {
                title: "States".to_string(),
                id:    "states".to_string(),
                description: "Hover applies a dark overlay (rgba 0,0,0,0.2) via box-shadow inset. Disabled and loading prevent interaction.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "default / hover / disabled / loading".to_string(),
                        canvas_class: "flex-col items-start gap-5".to_string(),

                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "default" }
                            Button { variant: ButtonVariant::Primary,   "Primary" }
                            Button { variant: ButtonVariant::Secondary, "Secondary" }
                            Button { variant: ButtonVariant::Outline,   "Outline" }
                            Button { variant: ButtonVariant::Ghost,     "Ghost" }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "hover" }
                            Button { variant: ButtonVariant::Primary,   class: "demo-hover".to_string(), "Primary" }
                            Button { variant: ButtonVariant::Secondary, class: "demo-hover".to_string(), "Secondary" }
                            Button { variant: ButtonVariant::Outline,   class: "demo-hover".to_string(), "Outline" }
                            Button { variant: ButtonVariant::Ghost,     class: "demo-hover".to_string(), "Ghost" }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "disabled" }
                            Button { variant: ButtonVariant::Primary,   disabled: true, "Primary" }
                            Button { variant: ButtonVariant::Secondary, disabled: true, "Secondary" }
                            Button { variant: ButtonVariant::Outline,   disabled: true, "Outline" }
                            Button { variant: ButtonVariant::Ghost,     disabled: true, "Ghost" }
                        }
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-label-3 shrink-0 w-20 text-right",
                                style: "color: var(--ratel-color-font-neutral-absolute);", "loading" }
                            Button { variant: ButtonVariant::Primary,   loading: true, "Saving…" }
                            Button { variant: ButtonVariant::Secondary, loading: true, "Loading" }
                            Button { variant: ButtonVariant::Outline,   loading: true, "Fetching" }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"let submitting = use_signal(|| false);

rsx! {
    // Disabled
    Button { disabled: true, "Unavailable" }

    // Loading — disables interaction and shows spinner
    Button {
        loading:  *submitting.read(),
        disabled: !form_valid,
        onclick:  move |_| submitting.set(true),
        "Submit"
    }
}"#.to_string(),
                    }
                }
            }

            // ── Token reference ───────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                description: "All visual properties resolve to design tokens. Swatches reflect the current theme.".to_string(),

                TokenTable { rows: BUTTON_TOKEN_ROWS.to_vec(), show_preview: true }
            }

            // ── Usage guidelines ──────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: BUTTON_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Variant × Radius comparison matrix ──────────────────────────────────────

#[component]
fn VariantMatrix() -> Element {
    rsx! {
        div {
            class: "rounded-ratel-xl overflow-hidden border",
            style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

            // Header
            div {
                class: "grid text-label-3 font-semibold uppercase tracking-widest",
                style: "grid-template-columns: 130px 1fr 1fr 1fr 1fr 2fr; \
                        background: var(--ratel-color-border-incard-background-default); \
                        border-bottom: 1px solid var(--ratel-color-border-stroke-neutral-800); \
                        color: var(--ratel-color-font-neutral-absolute);",
                div { class: "px-5 py-3", "Variant" }
                div { class: "px-3 py-3 text-center", "Xs" }
                div { class: "px-3 py-3 text-center", "Sm" }
                div { class: "px-3 py-3 text-center", "Md" }
                div { class: "px-3 py-3 text-center", "Lg" }
                div { class: "px-5 py-3", "When to use" }
            }

            VRow {
                name: "Primary",
                usage: "Highest-priority action per view. One per screen.",
                xs: rsx! { Button { variant: ButtonVariant::Primary, size: ButtonSize::Xs, "Text" } },
                sm: rsx! { Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, "Text" } },
                md: rsx! { Button { variant: ButtonVariant::Primary, "Text" } },
                lg: rsx! { Button { variant: ButtonVariant::Primary, size: ButtonSize::Lg, "Text" } },
            }
            VRow {
                name: "Secondary",
                usage: "White-fill supporting action. Pairs with Primary.",
                xs: rsx! { Button { variant: ButtonVariant::Secondary, size: ButtonSize::Xs, "Text" } },
                sm: rsx! { Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Text" } },
                md: rsx! { Button { variant: ButtonVariant::Secondary, "Text" } },
                lg: rsx! { Button { variant: ButtonVariant::Secondary, size: ButtonSize::Lg, "Text" } },
            }
            VRow {
                name: "Outline",
                usage: "Transparent + white border. Equal weight to Primary when needed.",
                xs: rsx! { Button { variant: ButtonVariant::Outline, size: ButtonSize::Xs, "Text" } },
                sm: rsx! { Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Text" } },
                md: rsx! { Button { variant: ButtonVariant::Outline, "Text" } },
                lg: rsx! { Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Text" } },
            }
            VRow {
                name: "Ghost",
                usage: "No fill, no border. Table rows, toolbars, inline actions.",
                xs: rsx! { Button { variant: ButtonVariant::Ghost, size: ButtonSize::Xs, "Text" } },
                sm: rsx! { Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Text" } },
                md: rsx! { Button { variant: ButtonVariant::Ghost, "Text" } },
                lg: rsx! { Button { variant: ButtonVariant::Ghost, size: ButtonSize::Lg, "Text" } },
                last: true,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct VRowProps {
    name:  &'static str,
    usage: &'static str,
    xs: Element,
    sm: Element,
    md: Element,
    lg: Element,
    #[props(optional, default = false)]
    last: bool,
}

#[component]
fn VRow(props: VRowProps) -> Element {
    let border = if props.last { "" } else {
        "border-bottom: 1px solid var(--ratel-color-border-stroke-neutral-800);"
    };
    rsx! {
        div {
            class: "grid items-center",
            style: format!("grid-template-columns: 130px 1fr 1fr 1fr 1fr 2fr; {border}"),
            div { class: "px-5 py-4",
                span { class: "text-label-2 font-semibold",
                    style: "color: var(--ratel-color-font-default);",
                    { props.name }
                }
            }
            div { class: "px-3 py-4 flex justify-center", { props.xs } }
            div { class: "px-3 py-4 flex justify-center", { props.sm } }
            div { class: "px-3 py-4 flex justify-center", { props.md } }
            div { class: "px-3 py-4 flex justify-center", { props.lg } }
            div { class: "px-5 py-4 text-label-3",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { props.usage }
            }
        }
    }
}

// ─── Generic icon (plus) used in icon-button previews ─────────────────────────

#[component]
fn IconPlus() -> Element {
    rsx! {
        svg {
            xmlns:    "http://www.w3.org/2000/svg",
            fill:     "none",
            view_box: "0 0 24 24",
            stroke:   "currentColor",
            stroke_width: "2",
            class:    "w-5 h-5 shrink-0",
            path {
                stroke_linecap:  "round",
                stroke_linejoin: "round",
                d: "M12 4v16m8-8H4",
            }
        }
    }
}

// ─── Static data ──────────────────────────────────────────────────────────────

static BUTTON_TOKEN_ROWS: &[TokenRow] = &[
    // Radius
    TokenRow::new("--ratel-radius-xl",                               "12px",                          "Border radius — lg size").with_category("Radius"),
    TokenRow::new("--ratel-radius-1-lg",                             "8px",                           "Border radius — sm size").with_category("Radius"),
    // Colors — Primary
    TokenRow::new("--ratel-color-generic-primary",                   "#FCB300 (dark) / #F79800 (light)", "Primary: fill").with_category("Color"),
    TokenRow::new("--ratel-color-font-black-absolute",               "#0A0A0A",                       "Primary: text").with_category("Color"),
    // Colors — Secondary
    TokenRow::new("--ratel-color-button-background-invert-white",    "#FFFFFF (dark) / #404040 (light)", "Secondary: fill").with_category("Color"),
    TokenRow::new("--ratel-color-font-invert-black",                 "#0A0A0A",                       "Secondary: text").with_category("Color"),
    // Colors — Outline / Ghost
    TokenRow::new("--ratel-color-button-stroke-white",               "#FFFFFF (dark) / #A1A1A1 (light)", "Outline: border").with_category("Color"),
    TokenRow::new("--ratel-color-font-invert-white",                 "#FFFFFF (dark) / #171717 (light)", "Outline / Ghost: text").with_category("Color"),
    // Colors — hover overlays
    TokenRow::new("--ratel-color-button-hover-dark-opacity-20%",     "rgba(0,0,0,0.20)",              "Primary / Secondary hover").with_category("Color"),
    TokenRow::new("--ratel-color-button-hover-light-opacity-5%",     "rgba(255,255,255,0.05)",        "Outline / Ghost hover").with_category("Color"),
    // Colors — Disabled
    TokenRow::new("--ratel-color-button-disable",                    "#212121 (dark) / #E5E5E5 (light)", "Disabled: fill").with_category("Color"),
    TokenRow::new("--ratel-color-font-disable",                      "#404040",                       "Disabled: text").with_category("Color"),
    // Spacing
    TokenRow::new("--ratel-space-25",                                "25px",                          "Horizontal padding — lg").with_category("Spacing"),
    TokenRow::new("--ratel-space-20",                                "20px",                          "Horizontal padding — md").with_category("Spacing"),
    TokenRow::new("--ratel-space-16",                                "16px",                          "Horizontal padding — sm / xs").with_category("Spacing"),
    TokenRow::new("--ratel-space-12",                                "12px",                          "Vertical padding — lg / md").with_category("Spacing"),
];

static BUTTON_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "One Primary button per view",
        "Primary commands attention. Pick the single highest-priority action and make it Primary; everything else is Secondary, Outline, or Ghost."
    ),
    DoDont::dont(
        "Mix radius styles in the same control group",
        "If your form uses square buttons, use square throughout. Mixing round and square in the same row breaks visual rhythm."
    ),
    DoDont::do_(
        "Use icon-only buttons for toolbar and table actions",
        "RoundIcon and SquareIcon buttons have fixed dimensions (44/32/24px) that align perfectly in grids and toolbars."
    ),
    DoDont::dont(
        "Use Xs in standalone CTAs",
        "Xs (12px SemiBold, 4/8px padding) is designed for dense inline contexts like tags or compact toolbars — not for primary page actions."
    ),
    DoDont::do_(
        "Pair loading with async operations",
        "Set loading=true while an async action is in flight. It disables interaction and shows a spinner, preventing double-submits."
    ),
    DoDont::dont(
        "Hardcode colors outside the token system",
        "Never write style=\"background: #FCB300\". Token variables ensure correct light/dark adaptation automatically."
    ),
];
