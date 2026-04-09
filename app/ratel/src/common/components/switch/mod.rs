use crate::common::*;

#[component]
pub fn Switch(
    active: bool,
    on_toggle: EventHandler<MouseEvent>,
    #[props(default)] label: String,
    #[props(default)] disabled: bool,
) -> Element {
    let container_class = if disabled {
        "relative inline-flex items-center w-11 h-6 rounded-full border border-black/40 bg-[image:var(--rail-recessed)] shadow-[inset_0_2px_4px_rgba(0,0,0,0.5)] opacity-40 cursor-not-allowed transition-colors"
    } else if active {
        "relative inline-flex items-center w-11 h-6 rounded-full border border-[color:rgba(252,179,0,0.4)] bg-gradient-to-r from-[#a07a30] to-[#fcb300] shadow-[inset_0_2px_4px_rgba(0,0,0,0.3),var(--rim-glow-primary)] cursor-pointer transition-colors"
    } else {
        "relative inline-flex items-center w-11 h-6 rounded-full border border-black/40 bg-[image:var(--rail-recessed)] shadow-[inset_0_2px_4px_rgba(0,0,0,0.5)] cursor-pointer transition-colors"
    };
    let knob_class = if active && !disabled {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-gradient-to-br from-white via-[#f5f5f5] to-[#d4d4d4] shadow-[0_2px_8px_rgba(0,0,0,0.4),0_0_16px_rgba(252,179,0,0.6),inset_0_1px_0_rgba(255,255,255,0.9)] transition-transform translate-x-5"
    } else {
        "absolute left-[2px] top-[2px] w-5 h-5 rounded-full bg-gradient-to-br from-white via-[#f5f5f5] to-[#d4d4d4] shadow-[0_2px_8px_rgba(0,0,0,0.4),inset_0_1px_0_rgba(255,255,255,0.9)] transition-transform"
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
