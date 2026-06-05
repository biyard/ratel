use crate::common::*;
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};

#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    // Use the bare `.popover` class only — its CSS sets
    // `position: relative; display: inline-block`, which is the
    // anchoring context the `[data-side="bottom"]` content rule needs
    // (`left: 50%` is interpreted relative to the trigger column, not
    // the page). The previous `flex flex-1 w-full grow` made the root
    // stretch to the parent's full width, so the popover landed in the
    // middle of the page instead of below the trigger button — that
    // was the "선행 액션" selector visual bug. Callers that need a
    // flex/full-width root can wrap PopoverRoot in their own element.
    rsx! {
        popover::PopoverRoot {
            class: "popover",
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        popover::PopoverTrigger { class: "popover-trigger", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        popover::PopoverContent {
            class: "popover-content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
