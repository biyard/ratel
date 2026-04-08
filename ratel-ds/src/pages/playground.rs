use dioxus::prelude::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, ButtonRadius, Input, InputSize, InputState};

// ─── Example registry ─────────────────────────────────────────────────────────
//
// Each entry has a stable `id`, a display `label`, a `group` (for the header),
// and the Rust source code shown in the editor panel.
//
// `render_example(id)` is the matching live render function.
// To add a new example: push a row to EXAMPLES and add a match arm to render_example().

struct Example {
    id:    &'static str,
    group: &'static str,
    label: &'static str,
    code:  &'static str,
}

static EXAMPLES: &[Example] = &[
    // ── Button ────────────────────────────────────────────────────────────────
    Example {
        id:    "btn-variants",
        group: "Button",
        label: "Variants",
        code: r#"rsx! {
    Button { "Primary" }

    Button {
        variant: ButtonVariant::Secondary,
        "Secondary"
    }

    Button {
        variant: ButtonVariant::Outline,
        "Outline"
    }

    Button {
        variant: ButtonVariant::Ghost,
        "Ghost"
    }
}"#,
    },
    Example {
        id:    "btn-sizes",
        group: "Button",
        label: "Sizes",
        code: r#"rsx! {
    Button { size: ButtonSize::Xs, "X-Small" }
    Button { size: ButtonSize::Sm, "Small" }
    Button { size: ButtonSize::Md, "Medium" }  // default
    Button { size: ButtonSize::Lg, "Large" }
}"#,
    },
    Example {
        id:    "btn-radius",
        group: "Button",
        label: "Radius",
        code: r#"rsx! {
    // Square (default per size)
    Button { "Square" }

    // Full pill
    Button { radius: ButtonRadius::Round, "Round pill" }

    // Icon-square (fixed 44×44)
    Button {
        radius: ButtonRadius::SquareIcon,
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none", view_box: "0 0 24 24",
            stroke: "currentColor", stroke_width: "2",
            class: "w-5 h-5",
            path {
                stroke_linecap: "round",
                d: "M12 4v16m8-8H4",
            }
        }
    }

    // Icon-round (circle)
    Button {
        radius: ButtonRadius::RoundIcon,
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none", view_box: "0 0 24 24",
            stroke: "currentColor", stroke_width: "2",
            class: "w-5 h-5",
            path {
                stroke_linecap: "round",
                d: "M12 4v16m8-8H4",
            }
        }
    }
}"#,
    },
    Example {
        id:    "btn-icons",
        group: "Button",
        label: "With Icons",
        code: r#"rsx! {
    Button {
        leading_icon: rsx! { i { class: "ph ph-plus" } },
        "Add Item"
    }

    Button {
        variant:       ButtonVariant::Secondary,
        trailing_icon: rsx! { i { class: "ph ph-arrow-right" } },
        "Continue"
    }

    Button {
        variant:       ButtonVariant::Outline,
        leading_icon:  rsx! { i { class: "ph ph-magnifying-glass" } },
        trailing_icon: rsx! { i { class: "ph ph-caret-down" } },
        "Search"
    }
}"#,
    },
    Example {
        id:    "btn-states",
        group: "Button",
        label: "States",
        code: r#"rsx! {
    // Normal
    Button { "Normal" }

    // Disabled
    Button { disabled: true, "Disabled" }

    // Loading — blocks interaction, shows spinner
    Button { loading: true, "Loading…" }
}"#,
    },
    // ── Input ─────────────────────────────────────────────────────────────────
    Example {
        id:    "inp-basic",
        group: "Input",
        label: "Basic",
        code: r#"rsx! {
    div { class: "w-full max-w-sm space-y-4",
        Input {
            label:       "Full name".to_string(),
            placeholder: "Ada Lovelace".to_string(),
            helper:      "As it appears on your passport.".to_string(),
        }

        Input {
            label:       "Email".to_string(),
            input_type:  "email".to_string(),
            placeholder: "you@example.com".to_string(),
        }
    }
}"#,
    },
    Example {
        id:    "inp-sizes",
        group: "Input",
        label: "Sizes",
        code: r#"rsx! {
    div { class: "w-full max-w-sm space-y-3",
        Input { size: InputSize::Sm, placeholder: "Small".to_string() }
        Input { size: InputSize::Md, placeholder: "Medium (default)".to_string() }
        Input { size: InputSize::Lg, placeholder: "Large".to_string() }
    }
}"#,
    },
    Example {
        id:    "inp-states",
        group: "Input",
        label: "States",
        code: r#"rsx! {
    div { class: "w-full max-w-sm space-y-4",
        Input {
            label:       "Default".to_string(),
            placeholder: "Type something…".to_string(),
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
            placeholder: "Cannot edit".to_string(),
            disabled:    true,
        }
    }
}"#,
    },
    Example {
        id:    "inp-icons",
        group: "Input",
        label: "With Icons",
        code: r#"rsx! {
    div { class: "w-full max-w-sm space-y-4",
        Input {
            placeholder:  "Search…".to_string(),
            leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
        }

        Input {
            label:         "Amount".to_string(),
            input_type:    "number".to_string(),
            placeholder:   "0.00".to_string(),
            leading_icon:  rsx! { i { class: "ph ph-currency-dollar" } },
        }

        Input {
            rounded:      true,
            placeholder:  "Round search bar".to_string(),
            leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
        }
    }
}"#,
    },
    Example {
        id:    "inp-textarea",
        group: "Input",
        label: "Textarea",
        code: r#"rsx! {
    div { class: "w-full max-w-sm",
        Input {
            label:       "Description".to_string(),
            multiline:   true,
            placeholder: "Write a short description…".to_string(),
            helper:      "Max 200 characters.".to_string(),
        }
    }
}"#,
    },
];

// ─── Live render function ─────────────────────────────────────────────────────
//
// Returns the rendered Element for the given example id.
// Returns an error message Element for unknown ids.

fn render_example(id: &str) -> Element {
    match id {
        "btn-variants" => rsx! {
            Button { "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline,   "Outline" }
            Button { variant: ButtonVariant::Ghost,     "Ghost" }
        },
        "btn-sizes" => rsx! {
            div { class: "flex items-end gap-3 flex-wrap",
                Button { size: ButtonSize::Xs, "X-Small" }
                Button { size: ButtonSize::Sm, "Small" }
                Button { size: ButtonSize::Md, "Medium" }
                Button { size: ButtonSize::Lg, "Large" }
            }
        },
        "btn-radius" => rsx! {
            Button { "Square" }
            Button { radius: ButtonRadius::Round, "Round pill" }
            Button {
                radius: ButtonRadius::SquareIcon,
                svg {
                    xmlns: "http://www.w3.org/2000/svg", fill: "none",
                    view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                    class: "w-5 h-5",
                    path { stroke_linecap: "round", d: "M12 4v16m8-8H4" }
                }
            }
            Button {
                radius: ButtonRadius::RoundIcon,
                svg {
                    xmlns: "http://www.w3.org/2000/svg", fill: "none",
                    view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                    class: "w-5 h-5",
                    path { stroke_linecap: "round", d: "M12 4v16m8-8H4" }
                }
            }
        },
        "btn-icons" => rsx! {
            Button { leading_icon: rsx! { i { class: "ph ph-plus" } }, "Add Item" }
            Button {
                variant:       ButtonVariant::Secondary,
                trailing_icon: rsx! { i { class: "ph ph-arrow-right" } },
                "Continue"
            }
            Button {
                variant:       ButtonVariant::Outline,
                leading_icon:  rsx! { i { class: "ph ph-magnifying-glass" } },
                trailing_icon: rsx! { i { class: "ph ph-caret-down" } },
                "Search"
            }
        },
        "btn-states" => rsx! {
            Button { "Normal" }
            Button { disabled: true, "Disabled" }
            Button { loading: true, "Loading…" }
        },
        "inp-basic" => rsx! {
            div { class: "w-full max-w-sm space-y-4",
                Input {
                    label:       "Full name".to_string(),
                    placeholder: "Ada Lovelace".to_string(),
                    helper:      "As it appears on your passport.".to_string(),
                }
                Input {
                    label:       "Email".to_string(),
                    input_type:  "email".to_string(),
                    placeholder: "you@example.com".to_string(),
                }
            }
        },
        "inp-sizes" => rsx! {
            div { class: "w-full max-w-sm space-y-3",
                Input { size: InputSize::Sm, placeholder: "Small".to_string() }
                Input { size: InputSize::Md, placeholder: "Medium (default)".to_string() }
                Input { size: InputSize::Lg, placeholder: "Large".to_string() }
            }
        },
        "inp-states" => rsx! {
            div { class: "w-full max-w-sm space-y-4",
                Input { label: "Default".to_string(), placeholder: "Type something…".to_string() }
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
                    placeholder: "Cannot edit".to_string(),
                    disabled:    true,
                }
            }
        },
        "inp-icons" => rsx! {
            div { class: "w-full max-w-sm space-y-4",
                Input {
                    placeholder:  "Search…".to_string(),
                    leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
                }
                Input {
                    label:        "Amount".to_string(),
                    input_type:   "number".to_string(),
                    placeholder:  "0.00".to_string(),
                    leading_icon: rsx! { i { class: "ph ph-currency-dollar" } },
                }
                Input {
                    rounded:      true,
                    placeholder:  "Round search bar".to_string(),
                    leading_icon: rsx! { i { class: "ph ph-magnifying-glass" } },
                }
            }
        },
        "inp-textarea" => rsx! {
            div { class: "w-full max-w-sm",
                Input {
                    label:       "Description".to_string(),
                    multiline:   true,
                    placeholder: "Write a short description…".to_string(),
                    helper:      "Max 200 characters.".to_string(),
                }
            }
        },
        _ => rsx! {
            div { class: "pg-error", "Unknown example id: {id}" }
        },
    }
}

// ─── Page ─────────────────────────────────────────────────────────────────────

#[component]
pub fn Playground() -> Element {
    // Active example id
    let mut active_id = use_signal(|| "btn-variants");
    // Code in the left panel — synced from active example, editable by user
    let initial_code = EXAMPLES[0].code;
    let mut code = use_signal(|| initial_code.to_string());
    // Whether the preview is live (set to true after "Run Preview")
    let mut live = use_signal(|| true);

    let current_id   = *active_id.read();
    let current_code = code.read().clone();

    rsx! {
        div { class: "pg-layout",

            // ── Left panel: code editor ───────────────────────────────────
            div { class: "pg-panel",

                // Header: title + group tabs
                div { class: "pg-panel-header gap-3",
                    span { class: "pg-panel-title shrink-0", "Code" }

                    // Example tabs — grouped
                    div { class: "pg-tabs",
                        for ex in EXAMPLES {
                            button {
                                class: if ex.id == current_id { "pg-tab active" } else { "pg-tab" },
                                onclick: {
                                    let id = ex.id;
                                    let ex_code = ex.code;
                                    move |_| {
                                        active_id.set(id);
                                        code.set(ex_code.to_string());
                                        live.set(true);
                                    }
                                },
                                // Group badge + label
                                span {
                                    class: "text-label-4 opacity-60 mr-1",
                                    { ex.group }
                                }
                                { ex.label }
                            }
                        }
                    }
                }

                // Code editor body
                div { class: "pg-panel-body flex flex-col",
                    textarea {
                        class:    "pg-code flex-1",
                        value:    "{current_code}",
                        spellcheck: "false",
                        autocomplete: "off",
                        oninput: move |evt| {
                            code.set(evt.value());
                            live.set(false);
                        },
                    }

                    // Action bar
                    div {
                        class: "flex items-center gap-2 px-4 py-3 shrink-0",
                        style: "background-color: var(--ratel-color-border-background-neutral-850); border-top: 1px solid var(--ratel-color-border-stroke-neutral-800);",

                        // "Run Preview" note: since Dioxus compiles to WASM we can't eval
                        // arbitrary Rust at runtime. This button re-renders the current
                        // selected predefined example.
                        button {
                            class: "btn btn-primary btn-sm",
                            onclick: move |_| live.set(true),
                            i { class: "ph ph-play", style: "font-size: 14px;" }
                            "Run Preview"
                        }
                        button {
                            class: "btn btn-ghost btn-sm",
                            onclick: {
                                let ex = EXAMPLES.iter().find(|e| e.id == current_id).unwrap_or(&EXAMPLES[0]);
                                let reset_code = ex.code;
                                move |_| {
                                    code.set(reset_code.to_string());
                                    live.set(true);
                                }
                            },
                            "Reset"
                        }

                        // Live indicator
                        div { class: "ml-auto flex items-center gap-1.5",
                            div {
                                class: "w-1.5 h-1.5 rounded-full",
                                style: if *live.read() {
                                    "background-color: var(--ratel-color-generic-success);"
                                } else {
                                    "background-color: var(--ratel-color-font-neutral-absolute);"
                                },
                            }
                            span {
                                class: "text-label-4",
                                style: "color: var(--ratel-color-font-neutral-absolute);",
                                if *live.read() { "Live" } else { "Edited — click Run to update" }
                            }
                        }
                    }
                }
            }

            // ── Right panel: live preview ─────────────────────────────────
            div { class: "pg-panel",

                // Header
                div { class: "pg-panel-header",
                    span { class: "pg-panel-title", "Preview" }

                    // Example info
                    if let Some(ex) = EXAMPLES.iter().find(|e| e.id == current_id) {
                        span {
                            class: "text-label-3",
                            style: "color: var(--ratel-color-font-neutral-absolute);",
                            "{ ex.group } · { ex.label }"
                        }
                    }
                }

                // Preview canvas
                div { class: "pg-panel-body",
                    div { class: "pg-canvas",
                        if *live.read() {
                            { render_example(current_id) }
                        } else {
                            div {
                                class: "flex flex-col items-center justify-center w-full gap-3",
                                style: "min-height: 120px; color: var(--ratel-color-font-neutral-absolute);",
                                i { class: "ph ph-code", style: "font-size: 32px;" }
                                span { class: "text-label-3", "Click \"Run Preview\" to render your changes." }
                            }
                        }
                    }
                }
            }
        }
    }
}
