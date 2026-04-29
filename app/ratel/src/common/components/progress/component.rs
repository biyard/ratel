use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressIndicatorProps, ProgressProps};

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            class: "progress",
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        progress::ProgressIndicator { class: "progress-indicator", attributes: props.attributes, {props.children} }
    }
}
