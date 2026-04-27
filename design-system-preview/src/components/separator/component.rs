use dioxus::prelude::*;
use dioxus_primitives_core::separator::{self, SeparatorProps};

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        separator::Separator {
            class: "separator",
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: props.attributes,
            {props.children}
        }
    }
}
