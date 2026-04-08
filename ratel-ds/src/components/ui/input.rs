use dioxus::prelude::*;

// ─── Size ─────────────────────────────────────────────────────────────────────

/// Padding / font-size tier for the input.
/// Maps to `.inp-{size}` CSS classes in tailwind.css.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Sm => "inp-sm",
            Self::Md => "inp-md",
            Self::Lg => "inp-lg",
        }
    }
}

// ─── State ───────────────────────────────────────────────────────────────────

/// Explicit visual state override.
/// Default resolves automatically from `disabled` and `has_error` props —
/// pass this only when you need to force a specific appearance (e.g. in docs).
#[derive(Clone, PartialEq, Default, Debug)]
pub enum InputState {
    /// No override — derive state from disabled/has_error props
    #[default]
    Default,
    /// Error border + error-colored helper text
    Error,
    /// Green border + success-colored helper text
    Valid,
    /// Disabled appearance (also set `disabled: true` to block interaction)
    Disabled,
}

impl InputState {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Default  => "",
            Self::Error    => "inp-error",
            Self::Valid    => "inp-valid",
            Self::Disabled => "inp-disabled",
        }
    }
}

// ─── Props ────────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Text displayed above the input
    #[props(optional)]
    pub label: Option<String>,

    /// Native placeholder text
    #[props(optional, default = String::new())]
    pub placeholder: String,

    /// Controlled value — bind to a signal for controlled input
    #[props(optional, default = String::new())]
    pub value: String,

    /// Called on every keystroke with the new value
    #[props(optional)]
    pub oninput: Option<EventHandler<String>>,

    /// Size tier — default Md
    #[props(optional, default = InputSize::Md)]
    pub size: InputSize,

    /// Explicit state override — typically leave Default and use `has_error` / `disabled`
    #[props(optional, default = InputState::Default)]
    pub state: InputState,

    /// Shows error styling and optionally an error message below the input
    #[props(optional, default = false)]
    pub has_error: bool,

    /// Helper or error message shown below the input
    #[props(optional)]
    pub helper: Option<String>,

    /// Disables the input — also applies disabled CSS class
    #[props(optional, default = false)]
    pub disabled: bool,

    /// Merged into the wrapper div's class string (e.g. "w-full")
    #[props(optional, default = String::new())]
    pub class: String,

    /// native input `type` attribute — default "text"
    #[props(optional, default = "text".to_string())]
    pub input_type: String,

    /// Applies pill border-radius (9999px)
    #[props(optional, default = false)]
    pub rounded: bool,

    /// Optional icon rendered at the start of the input (inside left edge)
    #[props(optional)]
    pub leading_icon: Option<Element>,

    /// Optional icon rendered at the end of the input (inside right edge)
    #[props(optional)]
    pub trailing_icon: Option<Element>,

    /// Renders a <textarea> instead of <input> for multiline text
    #[props(optional, default = false)]
    pub multiline: bool,
}

// ─── Component ────────────────────────────────────────────────────────────────

#[component]
pub fn Input(props: InputProps) -> Element {
    // Resolve the effective state modifier class:
    // explicit `state` prop wins; otherwise derive from `disabled` / `has_error`.
    let state_class = if props.state != InputState::Default {
        props.state.css_class()
    } else if props.disabled {
        "inp-disabled"
    } else if props.has_error {
        "inp-error"
    } else {
        ""
    };

    let round_class    = if props.rounded               { "inp-round"        } else { "" };
    let has_leading    = props.leading_icon.is_some();
    let has_trailing   = props.trailing_icon.is_some();
    let lead_cls       = if has_leading                 { "inp-has-leading"  } else { "" };
    let trail_cls      = if has_trailing                { "inp-has-trailing" } else { "" };
    let textarea_cls   = if props.multiline             { "inp-textarea"     } else { "" };

    let parts = [
        "inp",
        props.size.css_class(),
        state_class,
        round_class,
        lead_cls,
        trail_cls,
        textarea_cls,
    ];
    let input_class: String = parts.iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ");

    let wrapper_class = {
        let extra = props.class.trim();
        if extra.is_empty() { "inp-field".to_string() }
        else                { format!("inp-field {extra}") }
    };

    let is_error = props.has_error || props.state == InputState::Error;
    // Clone oninput for each render branch that needs it (only one branch executes)
    let oninput_ta = props.oninput.clone();  // textarea branch
    let oninput_in = props.oninput.clone();  // input branch

    rsx! {
        div { class: "{wrapper_class}",

            // Label
            if let Some(label) = &props.label {
                label { class: "inp-label", { label.as_str() } }
            }

            // Textarea (multiline)
            if props.multiline {
                textarea {
                    class:          "{input_class}",
                    placeholder:    "{props.placeholder}",
                    value:          "{props.value}",
                    disabled:       props.disabled,
                    "aria-invalid": if is_error { "true" } else { "false" },
                    oninput: move |evt| {
                        if let Some(ref h) = oninput_ta { h.call(evt.value()); }
                    },
                }
            }

            // Single-line input (with optional icon wrapper)
            if !props.multiline {
                div { class: if has_leading || has_trailing { "inp-wrapper" } else { "" },

                    if let Some(icon) = props.leading_icon {
                        span { class: "inp-icon-leading", {icon} }
                    }

                    input {
                        r#type:         "{props.input_type}",
                        class:          "{input_class}",
                        placeholder:    "{props.placeholder}",
                        value:          "{props.value}",
                        disabled:       props.disabled,
                        "aria-invalid": if is_error { "true" } else { "false" },
                        oninput: move |evt| {
                            if let Some(ref h) = oninput_in { h.call(evt.value()); }
                        },
                    }

                    if let Some(icon) = props.trailing_icon {
                        span { class: "inp-icon-trailing", {icon} }
                    }
                }
            }

            // Helper / error text
            if let Some(helper) = &props.helper {
                span {
                    class: if is_error { "inp-helper inp-helper-error" } else { "inp-helper" },
                    { helper.as_str() }
                }
            }
        }
    }
}
