#![allow(non_snake_case)]
use super::ButtonSize;
use dioxus::prelude::*;

#[component]
pub fn PrimaryButton(
    #[props(default = Default::default())] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    children: Element,

    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let bg = if !disabled {
        "bg-btn-p hover:bg-btn-p-hover text-btn-p-text hover:text-btn-p-hover-text cursor-pointer"
    } else {
        "bg-btn-p-disabled text-btn-p-disabled cursor-not-allowed"
    };
    let padding = match size {
        ButtonSize::Normal => "px-40 py-20",
    };

    rsx! {
        button {
            class: "font-bold gap-10 flex items-center justify-center text-base rounded-[10px] {bg} {padding}",
            disabled,
            onclick: move |evt| {
                if let Some(onclick) = onclick {
                    onclick(evt);
                }
            },
            {children}
        }
    }
}
