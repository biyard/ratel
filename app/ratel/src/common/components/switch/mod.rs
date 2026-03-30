use crate::common::*;

#[component]
pub fn Switch(
    active: bool,
    on_toggle: EventHandler<MouseEvent>,
    #[props(default)] label: String,
    #[props(default)] disabled: bool,
) -> Element {
    let container_class = if disabled {
        "relative inline-flex items-center w-11 h-6 rounded-full transition-colors bg-switch-track opacity-40 cursor-not-allowed"
    } else if active {
        "relative inline-flex items-center w-11 h-6 rounded-full transition-colors bg-primary cursor-pointer"
    } else {
        "relative inline-flex items-center w-11 h-6 rounded-full transition-colors bg-switch-track cursor-pointer"
    };
    let knob_class = if active && !disabled {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-switch-knob transition-transform translate-x-5"
    } else {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-switch-knob transition-transform"
    };

    #[cfg(debug_assertions)]
    if label.is_empty() {
        tracing::debug!("Switch: label prop should be provided for accessibility (aria-label)");
    }

    let aria_label = if label.is_empty() { None } else { Some(label) };

    rsx! {
        button {
            r#type: "button",
            class: container_class,
            role: "switch",
            aria_checked: "{active}",
            aria_label: aria_label,
            disabled,
            onclick: on_toggle,
            span { class: knob_class }
        }
    }
}
