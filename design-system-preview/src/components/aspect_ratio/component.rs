use dioxus::prelude::*;
use dioxus_primitives_core::aspect_ratio::AspectRatioProps;

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    dioxus_primitives_core::aspect_ratio::AspectRatio(props)
}
