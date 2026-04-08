use dioxus::prelude::*;

// ─── Variant ──────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,   // "line" in Figma
    Ghost,     // "base" in Figma
    Destructive,
}

impl ButtonVariant {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Primary     => "btn-primary",
            Self::Secondary   => "btn-secondary",
            Self::Outline     => "btn-outline",
            Self::Ghost       => "btn-ghost",
            Self::Destructive => "btn-destructive",
        }
    }
}

// ─── Size ─────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Xs => "btn-xs",
            Self::Sm => "btn-sm",
            Self::Md => "btn-md",
            Self::Lg => "btn-lg",
        }
    }
}

// ─── Radius ───────────────────────────────────────────────────────────────────

/// Controls corner radius shape AND whether the button is icon-only.
///
/// - `Square`     — sharp radius per size (default)
/// - `Round`      — pill (border-radius: 9999px), with text
/// - `SquareIcon` — icon-only, sharp radius
/// - `RoundIcon`  — icon-only, circle
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonRadius {
    #[default]
    Square,
    Round,
    SquareIcon,
    RoundIcon,
}

impl ButtonRadius {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Square     => "",
            Self::Round      => "btn-round",
            Self::SquareIcon => "btn-icon",
            Self::RoundIcon  => "btn-icon btn-round",
        }
    }

    pub fn is_icon(&self) -> bool {
        matches!(self, Self::SquareIcon | Self::RoundIcon)
    }
}

// ─── Props ────────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style — default: Primary
    #[props(optional, default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    /// Padding / font-size tier — default: Md
    #[props(optional, default = ButtonSize::Md)]
    pub size: ButtonSize,

    /// Corner radius shape and icon-only mode — default: Square
    #[props(optional, default = ButtonRadius::Square)]
    pub radius: ButtonRadius,

    /// Renders as disabled
    #[props(optional, default = false)]
    pub disabled: bool,

    /// Shows an inline spinner; disables interaction
    #[props(optional, default = false)]
    pub loading: bool,

    /// Optional click handler
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Merged into the class string for layout overrides (e.g. "w-full")
    #[props(optional, default = String::new())]
    pub class: String,

    /// Optional icon rendered before children (leading icon)
    #[props(optional)]
    pub leading_icon: Option<Element>,

    /// Optional icon rendered after children (trailing icon)
    #[props(optional)]
    pub trailing_icon: Option<Element>,

    pub children: Element,
}

// ─── Component ────────────────────────────────────────────────────────────────

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let inactive = props.disabled || props.loading;
    let onclick  = props.onclick.clone();

    let radius_cls = props.radius.css_class();
    let extra      = props.class.trim();

    let class = match (radius_cls.is_empty(), extra.is_empty()) {
        (true,  true)  => format!("btn {} {}", props.variant.css_class(), props.size.css_class()),
        (false, true)  => format!("btn {} {} {}", props.variant.css_class(), props.size.css_class(), radius_cls),
        (true,  false) => format!("btn {} {} {}", props.variant.css_class(), props.size.css_class(), extra),
        (false, false) => format!("btn {} {} {} {}", props.variant.css_class(), props.size.css_class(), radius_cls, extra),
    };

    rsx! {
        button {
            class:           "{class}",
            disabled:        inactive,
            "aria-disabled": if inactive { "true" } else { "false" },
            "aria-busy":     if props.loading { "true" } else { "false" },
            onclick: move |evt| {
                if let Some(ref h) = onclick { h.call(evt); }
            },

            if props.loading {
                svg {
                    class:    "animate-spin shrink-0",
                    xmlns:    "http://www.w3.org/2000/svg",
                    fill:     "none",
                    view_box: "0 0 24 24",
                    style:    "width: 1em; height: 1em;",
                    circle {
                        cx: "12", cy: "12", r: "10",
                        stroke: "currentColor", stroke_width: "4",
                        style: "opacity: 0.25;",
                    }
                    path {
                        fill: "currentColor",
                        style: "opacity: 0.75;",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                    }
                }
            }

            if let Some(icon) = props.leading_icon { {icon} }
            { props.children }
            if let Some(icon) = props.trailing_icon { {icon} }
        }
    }
}
