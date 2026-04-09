use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes, separator};

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    #[props(default)]
    variant: SeparatorVariant,
    #[props(default = true)]
    horizontal: bool,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = false)]
    decorative: bool,

    /// Additional attributes to apply to the separator element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the separator component.
    children: Element,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let base = attributes!(div {
        class: "separator {props.variant}"
    });
    let attributes = merge_attributes(vec![base, props.attributes]);

    rsx! {
        separator::Separator {
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes,
            {props.children}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::EnumString, Default)]
pub enum SeparatorVariant {
    #[default]
    #[strum(serialize = "border-solid")]
    Solid,
    #[strum(serialize = "border-dashed")]
    Dashed,
    #[strum(
        serialize = "border-0 h-px bg-gradient-to-r from-transparent via-[color:rgba(252,179,0,0.4)] to-transparent"
    )]
    Gradient,
    #[strum(
        serialize = "border-0 h-px bg-gradient-to-r from-transparent via-[color:rgba(110,237,216,0.4)] to-transparent"
    )]
    GradientAccent,
}
