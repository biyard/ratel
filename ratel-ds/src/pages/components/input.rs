use dioxus::prelude::*;
use crate::components::ui::{Input, InputSize, InputState};
use crate::components::docs::{
    PageIntro, DocSection, SubSection,
    ComponentPreview,
    CodeBlock, TokenRow, TokenTable,
    DoDont, DoDontGrid,
};

// ─── Page ─────────────────────────────────────────────────────────────────────

#[component]
pub fn InputDocs() -> Element {
    rsx! {
        div { class: "ds-page-content",
            PageIntro {
                title:    "Input".to_string(),
                subtitle: "A flexible text field component. Supports single-line, multiline, and icon-decorated variants across three sizes. All states — default, focus, valid, error, disabled — are fully token-driven.".to_string(),
                badge:    "Phase 3 · Stable".to_string(),
            }

            // ── Overview ──────────────────────────────────────────────────
            DocSection {
                title: "Overview".to_string(),
                id:    "overview".to_string(),
                description: "Default appearance with a label, placeholder, and helper text.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "Default — medium size with label and helper".to_string(),
                        div { class: "w-full max-w-sm",
                            Input {
                                label:       "Email address".to_string(),
                                placeholder: "you@example.com".to_string(),
                                helper:      "We'll never share your email.".to_string(),
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::Input;

rsx! {
    Input {
        label:       "Email address".to_string(),
        placeholder: "you@example.com".to_string(),
        helper:      "We'll never share your email.".to_string(),
    }
}"#.to_string(),
                    }
                }
            }

            // ── Sizes ─────────────────────────────────────────────────────
            DocSection {
                title: "Sizes".to_string(),
                id:    "sizes".to_string(),
                description: "Three sizes map to the Label type scale — sm for compact UI, md for general forms, lg for prominent search fields or primary CTAs.".to_string(),

                div { class: "space-y-3",
                    SizeMatrix {}
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Input, InputSize};

rsx! {
    Input { size: InputSize::Sm, placeholder: "Small".to_string() }
    Input { size: InputSize::Md, placeholder: "Medium (default)".to_string() }
    Input { size: InputSize::Lg, placeholder: "Large".to_string() }
}"#.to_string(),
                    }
                }
            }

            // ── States ────────────────────────────────────────────────────
            DocSection {
                title: "States".to_string(),
                id:    "states".to_string(),
                description: "Focus is applied by the browser on interaction. Error, valid, and disabled are controlled via props. Valid shows a green border for confirmed success.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "default / error / valid / disabled".to_string(),
                        canvas_class: "flex-col items-start".to_string(),
                        div { class: "w-full max-w-sm space-y-5",
                            Input {
                                label:       "Default".to_string(),
                                placeholder: "Type something…".to_string(),
                                helper:      "Helper text appears here.".to_string(),
                            }
                            Input {
                                label:     "Error".to_string(),
                                value:     "invalid@".to_string(),
                                has_error: true,
                                helper:    "Enter a valid email address.".to_string(),
                            }
                            Input {
                                label:  "Valid".to_string(),
                                value:  "ada@example.com".to_string(),
                                state:  InputState::Valid,
                                helper: "Looks good!".to_string(),
                            }
                            Input {
                                label:       "Disabled".to_string(),
                                placeholder: "Cannot edit this field".to_string(),
                                disabled:    true,
                                helper:      "This field is read-only.".to_string(),
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"use crate::components::ui::{Input, InputState};

rsx! {
    // Error — red border, red helper text
    Input {
        label:     "Email".to_string(),
        has_error: true,
        helper:    "Enter a valid email address.".to_string(),
    }

    // Valid — green border, success helper text
    Input {
        label:  "Email".to_string(),
        value:  "ada@example.com".to_string(),
        state:  InputState::Valid,
        helper: "Looks good!".to_string(),
    }

    // Disabled
    Input {
        label:    "Account ID".to_string(),
        value:    "usr_48291".to_string(),
        disabled: true,
    }
}"#.to_string(),
                    }
                }
            }

            // ── Round shape ───────────────────────────────────────────────
            DocSection {
                title: "Round Shape".to_string(),
                id:    "round".to_string(),
                description: "Set rounded=true for a pill-shaped input. Pairs well with a leading search icon for search bars and filter fields.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "rounded — all sizes".to_string(),
                        canvas_class: "flex-col items-start gap-4".to_string(),
                        div { class: "w-full max-w-sm space-y-3",
                            Input {
                                size:        InputSize::Sm,
                                rounded:     true,
                                placeholder: "Small pill".to_string(),
                            }
                            Input {
                                rounded:     true,
                                placeholder: "Medium pill (default)".to_string(),
                            }
                            Input {
                                size:        InputSize::Lg,
                                rounded:     true,
                                placeholder: "Large pill".to_string(),
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"rsx! {
    Input {
        rounded:     true,
        placeholder: "Search…".to_string(),
    }
}"#.to_string(),
                    }
                }
            }

            // ── With Icons ────────────────────────────────────────────────
            DocSection {
                title: "With Icons".to_string(),
                id:    "icons".to_string(),
                description: "Use leading_icon and trailing_icon to place Phosphor icons inside the input field. Useful for search bars, clear buttons, and currency indicators.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "leading / trailing / search bar".to_string(),
                        canvas_class: "flex-col items-start gap-4".to_string(),
                        div { class: "w-full max-w-sm space-y-4",
                            Input {
                                label:        "Search".to_string(),
                                placeholder:  "Search anything…".to_string(),
                                leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
                            }
                            Input {
                                label:         "Amount".to_string(),
                                placeholder:   "0.00".to_string(),
                                input_type:    "number".to_string(),
                                leading_icon:  rsx! { i { class: "ph ph-currency-dollar" } },
                            }
                            Input {
                                placeholder:   "you@example.com".to_string(),
                                leading_icon:  rsx! { i { class: "ph ph-envelope-simple" } },
                                trailing_icon: rsx! { i { class: "ph ph-check-circle" } },
                            }
                            // Search bar: round + leading icon
                            Input {
                                rounded:      true,
                                placeholder:  "Round search bar…".to_string(),
                                leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"rsx! {
    // Leading icon
    Input {
        placeholder:  "Search…".to_string(),
        leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
    }

    // Trailing icon
    Input {
        placeholder:   "you@example.com".to_string(),
        trailing_icon: rsx! { i { class: "ph ph-check-circle" } },
    }

    // Round search bar
    Input {
        rounded:      true,
        placeholder:  "Search…".to_string(),
        leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
    }
}"#.to_string(),
                    }
                }
            }

            // ── Password ──────────────────────────────────────────────────
            DocSection {
                title: "Password".to_string(),
                id:    "password".to_string(),
                description: "Use a signal to toggle visibility between password and text types. Pass an interactive trailing icon to trigger the toggle.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "password with show / hide toggle".to_string(),
                        div { class: "w-full max-w-sm",
                            PasswordDemo {}
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"let mut visible = use_signal(|| false);

rsx! {
    Input {
        label:      "Password".to_string(),
        input_type: if *visible.read() { "text".to_string() }
                    else               { "password".to_string() },
        placeholder: "••••••••".to_string(),
        trailing_icon: rsx! {
            span {
                style:   "cursor: pointer;",
                onclick: move |_| visible.set(!*visible.read()),
                i { class: if *visible.read() { "ph ph-eye-slash" } else { "ph ph-eye" } }
            }
        },
    }
}"#.to_string(),
                    }
                }
            }

            // ── Textarea / Multiline ──────────────────────────────────────
            DocSection {
                title: "Textarea".to_string(),
                id:    "textarea".to_string(),
                description: "Set multiline=true to render a <textarea> element. Use for descriptions, notes, or any long-form text. The field is vertically resizable by default.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "multiline textarea — default and with helper".to_string(),
                        canvas_class: "flex-col items-start gap-4".to_string(),
                        div { class: "w-full max-w-sm space-y-4",
                            Input {
                                label:       "Description".to_string(),
                                multiline:   true,
                                placeholder: "Write a short description…".to_string(),
                                helper:      "Max 200 characters.".to_string(),
                            }
                            Input {
                                label:     "Notes".to_string(),
                                multiline: true,
                                has_error: true,
                                value:     "This field is required.".to_string(),
                                helper:    "Please add a note before continuing.".to_string(),
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"rsx! {
    Input {
        label:       "Description".to_string(),
        multiline:   true,
        placeholder: "Write a short description…".to_string(),
        helper:      "Max 200 characters.".to_string(),
    }
}"#.to_string(),
                    }
                }
            }

            // ── Input types ───────────────────────────────────────────────
            DocSection {
                title: "Input Types".to_string(),
                id:    "types".to_string(),
                description: "The input_type prop maps to the native HTML type attribute. Visual styles are shared — only browser behaviour changes.".to_string(),

                div { class: "space-y-3",
                    ComponentPreview { label: "text / email / number".to_string(),
                        canvas_class: "flex-col items-start".to_string(),
                        div { class: "w-full max-w-sm space-y-4",
                            Input {
                                label:       "Text".to_string(),
                                placeholder: "Plain text".to_string(),
                            }
                            Input {
                                label:       "Email".to_string(),
                                input_type:  "email".to_string(),
                                placeholder: "you@example.com".to_string(),
                            }
                            Input {
                                label:       "Number".to_string(),
                                input_type:  "number".to_string(),
                                placeholder: "0".to_string(),
                            }
                        }
                    }
                    CodeBlock {
                        lang: "rust".to_string(),
                        code: r#"rsx! {
    Input { label: "Email".to_string(),  input_type: "email".to_string(),  placeholder: "you@example.com".to_string() }
    Input { label: "Number".to_string(), input_type: "number".to_string(), placeholder: "0".to_string() }
}"#.to_string(),
                    }
                }
            }

            // ── Token reference ───────────────────────────────────────────
            DocSection {
                title: "Token Reference".to_string(),
                id:    "tokens".to_string(),
                description: "Every visual property of Input resolves to a design token. Swatches reflect the current theme.".to_string(),

                TokenTable { rows: INPUT_TOKEN_ROWS.to_vec(), show_preview: true }
            }

            // ── Usage guidelines ──────────────────────────────────────────
            DocSection {
                title: "Usage Guidelines".to_string(),
                id:    "usage".to_string(),
                DoDontGrid { items: INPUT_DO_DONT.to_vec() }
            }
        }
    }
}

// ─── Password demo (needs local signal) ──────────────────────────────────────

#[component]
fn PasswordDemo() -> Element {
    let mut visible = use_signal(|| false);
    let is_visible = *visible.read();
    rsx! {
        Input {
            label:       "Password".to_string(),
            input_type:  if is_visible { "text".to_string() } else { "password".to_string() },
            placeholder: "Enter your password".to_string(),
            trailing_icon: rsx! {
                span {
                    style:   "cursor: pointer;",
                    onclick: move |_| { let v = *visible.read(); visible.set(!v); },
                    i { class: if is_visible { "ph ph-eye-slash" } else { "ph ph-eye" } }
                }
            },
        }
    }
}

// ─── Size comparison matrix ───────────────────────────────────────────────────

#[component]
fn SizeMatrix() -> Element {
    rsx! {
        div {
            class: "rounded-ratel-xl overflow-hidden border",
            style: "border-color: var(--ratel-color-border-stroke-neutral-800);",

            // Header
            div {
                class: "grid text-label-3 font-semibold uppercase tracking-widest",
                style: "grid-template-columns: 100px 1fr 2fr; \
                        background: var(--ratel-color-border-incard-background-default); \
                        border-bottom: var(--ratel-stroke-1) solid var(--ratel-color-border-stroke-neutral-800); \
                        color: var(--ratel-color-font-neutral-absolute);",
                div { class: "px-5 py-3", "Size" }
                div { class: "px-5 py-3", "Preview" }
                div { class: "px-5 py-3", "When to use" }
            }

            SizeRow {
                name: "Sm",
                usage: "Compact UI — table filter rows, inline form controls, toolbars.",
                input: rsx! {
                    Input {
                        size:        InputSize::Sm,
                        placeholder: "Small input".to_string(),
                    }
                },
            }
            SizeRow {
                name: "Md",
                usage: "General-purpose. Default for all standard forms.",
                input: rsx! {
                    Input {
                        size:        InputSize::Md,
                        placeholder: "Medium input".to_string(),
                    }
                },
            }
            SizeRow {
                name: "Lg",
                usage: "Prominent search fields, hero email capture, primary CTAs.",
                input: rsx! {
                    Input {
                        size:        InputSize::Lg,
                        placeholder: "Large input".to_string(),
                    }
                },
                last: true,
            }
        }
    }
}

// ─── SizeRow ──────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct SizeRowProps {
    name:  &'static str,
    usage: &'static str,
    input: Element,
    #[props(optional, default = false)]
    last:  bool,
}

#[component]
fn SizeRow(props: SizeRowProps) -> Element {
    let border = if props.last {
        "".to_string()
    } else {
        "border-bottom: var(--ratel-stroke-1) solid var(--ratel-color-border-stroke-neutral-800);".to_string()
    };

    rsx! {
        div {
            class: "grid items-start",
            style: format!("grid-template-columns: 100px 1fr 2fr; {border}"),

            // Size badge
            div { class: "px-5 py-5",
                span {
                    class: "text-label-2 font-semibold",
                    style: "color: var(--ratel-color-font-default);",
                    { props.name }
                }
            }

            // Input preview
            div { class: "px-5 py-4 max-w-xs", { props.input } }

            // Usage description
            div {
                class: "px-5 py-5 text-label-3",
                style: "color: var(--ratel-color-font-neutral-absolute);",
                { props.usage }
            }
        }
    }
}

// ─── Static data ──────────────────────────────────────────────────────────────

static INPUT_TOKEN_ROWS: &[TokenRow] = &[
    // Layout
    TokenRow::new("--ratel-radius-md",                              "6px",                           "Border radius — all sizes and states").with_category("Radius"),
    TokenRow::new("--ratel-stroke-1",                               "1px",                           "Default border width").with_category("Stroke"),
    TokenRow::new("--ratel-stroke-2",                               "2px",                           "Focus ring spread width").with_category("Stroke"),
    // Spacing — vertical / horizontal padding per size
    TokenRow::new("--ratel-space-6 / 10",                           "6px / 10px",                    "Padding — sm").with_category("Spacing"),
    TokenRow::new("--ratel-space-8 / 12",                           "8px / 12px",                    "Padding — md").with_category("Spacing"),
    TokenRow::new("--ratel-space-12 / 16",                          "12px / 16px",                   "Padding — lg").with_category("Spacing"),
    TokenRow::new("--ratel-space-6",                                "6px",                           "Gap between label, input, and helper text").with_category("Spacing"),
    // Typography
    TokenRow::new("--ratel-text-label-label-3-*",                   "13px / 16px / -0.14px",         "Font — sm size / lh / ls").with_category("Type"),
    TokenRow::new("--ratel-text-label-label-2-*",                   "15px / 18px / -0.16px",         "Font — md size / lh / ls").with_category("Type"),
    TokenRow::new("--ratel-text-label-label-1-*",                   "17px / 20px / -0.18px",         "Font — lg size / lh / ls").with_category("Type"),
    TokenRow::new("--ratel-font-weight-medium",                     "500",                           "Label font weight").with_category("Type"),
    // Colors — default
    TokenRow::new("--ratel-color-border-background-neutral-850",    "#FFFFFF / #1A1A1A",             "Input fill (light / dark)").with_category("Color"),
    TokenRow::new("--ratel-color-border-stroke-neutral-800",        "#E5E5E5 / #262626",             "Default border (light / dark)").with_category("Color"),
    TokenRow::new("--ratel-color-font-default",                     "#262626 / #A1A1A1",             "Input text (light / dark)").with_category("Color"),
    TokenRow::new("--ratel-color-font-placeholder-neutral-400",     "#A1A1A1",                       "Placeholder text").with_category("Color"),
    // Colors — focus
    TokenRow::new("--ratel-color-generic-primary",                  "#F79800 / #FCB300",             "Focus border color (light / dark)").with_category("Color"),
    TokenRow::new("--ratel-color-generic-primary-opacity-25%",      "rgba(247,152,0,0.25)",          "Focus ring glow").with_category("Color"),
    // Colors — error
    TokenRow::new("--ratel-color-generic-error",                    "#FB2C36",                       "Error border color").with_category("Color"),
    TokenRow::new("--ratel-color-generic-error-opacity-10%",        "rgba(239,68,68,0.10)",          "Error focus ring glow").with_category("Color"),
    // Colors — valid
    TokenRow::new("--ratel-color-generic-success",                  "#00C951",                       "Valid border color").with_category("Color"),
    TokenRow::new("--ratel-color-generic-success-opacity-5%",       "rgba(0,201,81,0.05)",           "Valid focus ring glow").with_category("Color"),
    // Colors — disabled
    TokenRow::new("--ratel-color-button-disable",                   "#E5E5E5 / #212121",             "Disabled fill (light / dark)").with_category("Color"),
    TokenRow::new("--ratel-color-font-disable",                     "#404040",                       "Disabled text and placeholder").with_category("Color"),
    // Helper text
    TokenRow::new("--ratel-text-label-label-4-*",                   "12px / 14px / -0.12px",         "Helper text font (size / lh / ls)").with_category("Type"),
    TokenRow::new("--ratel-color-font-neutral-absolute",            "#8C8C8C",                       "Default helper text color").with_category("Color"),
];

static INPUT_DO_DONT: &[DoDont] = &[
    DoDont::do_(
        "Always pair an input with a visible label",
        "Placeholder text disappears when the user types. A persistent label (above or beside the input) keeps the field identifiable throughout filling. Use the label prop — don't rely on placeholder as a substitute."
    ),
    DoDont::dont(
        "Show errors before the user has had a chance to type",
        "Validate on blur or on submit, not on every keystroke from empty. Showing has_error=true on a blank field that hasn't been touched is alarming and unhelpful."
    ),
    DoDont::do_(
        "Use helper text for constraints, not warnings",
        "Helper text (the helper prop) is for proactive guidance: 'Must be 8 characters or more', 'Format: DD/MM/YYYY'. Keep it brief and always visible — don't hide it until an error occurs."
    ),
    DoDont::dont(
        "Use size=Lg in dense forms",
        "Large inputs are visually heavy. Use Lg only for standalone prominent fields (site search, hero email capture). Standard forms use Md; compact tables and toolbars use Sm."
    ),
    DoDont::do_(
        "Set input_type to the correct HTML type",
        "type=\"email\" enables mobile keyboard optimisation and browser autofill. type=\"password\" masks the value. type=\"number\" enables native increment controls. The Input component applies the same styles regardless of type."
    ),
    DoDont::dont(
        "Use disabled to lock fields the user should not see",
        "Disabled fields are visible but uneditable — appropriate for read-only values like an account ID. If the content is truly not relevant to the current user or state, hide the field entirely rather than disabling it."
    ),
];
