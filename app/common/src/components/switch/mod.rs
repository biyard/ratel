use crate::*;

#[component]
pub fn Switch(active: bool, on_toggle: EventHandler<MouseEvent>) -> Element {
    let container_class = if active {
        "relative inline-flex items-center w-11 h-6 rounded-full transition-colors bg-primary"
    } else {
        "relative inline-flex items-center w-11 h-6 rounded-full transition-colors bg-neutral-700"
    };
    let knob_class = if active {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-white transition-transform translate-x-5"
    } else {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-white transition-transform"
    };

    rsx! {
        button { class: container_class, onclick: on_toggle,
            span { class: knob_class }
        }
    }
}
