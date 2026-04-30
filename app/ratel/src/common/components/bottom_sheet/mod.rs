use crate::common::*;

/// A mobile-friendly bottom sheet overlay that slides up from the bottom of the screen.
///
/// Use this component for mobile-only panels that need a backdrop overlay and
/// slide-up animation. The sheet is positioned fixed at the bottom and uses
/// CSS transitions for smooth open/close animations.
///
/// # Example
/// ```rust
/// BottomSheet {
///     open: is_open(),
///     on_close: move |_| is_open.set(false),
///     BottomSheetHeader {
///         title: "Settings",
///     }
///     // content here
/// }
/// ```
#[component]
pub fn BottomSheet(
    open: bool,
    on_close: EventHandler<MouseEvent>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            class: "bottom-sheet-backdrop",
            onclick: move |e| on_close.call(e),
        }
        // Panel
        div {
            class: "bottom-sheet-panel {class}",
            {children}
        }
    }
}

/// Header section for a BottomSheet with an optional title and close button.
#[component]
pub fn BottomSheetHeader(
    #[props(default)] title: String,
    #[props(default)] show_close: bool,
    on_close: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Row {
            class: "w-full",
            main_axis_align: MainAxisAlign::Between,
            cross_axis_align: CrossAxisAlign::Center,
            if !title.is_empty() {
                span { class: "text-base font-semibold text-foreground", "{title}" }
            }
            if show_close {
                if let Some(handler) = on_close {
                    button {
                        class: "p-1 rounded-md cursor-pointer hover:bg-card-bg transition-colors",
                        onclick: move |e| handler.call(e),
                        lucide_dioxus::X {
                            class: "w-5 h-5 text-foreground-muted",
                        }
                    }
                }
            }
        }
    }
}
