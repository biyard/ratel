#![allow(non_snake_case)]
use super::ButtonSize;
use bdk::prelude::*;

#[component]
pub fn OutlinedButton(
    #[props(default = Default::default())] size: ButtonSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,

    children: Element,

    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = "hover:bg-btn-p-hover text-btn-p-text hover:text-btn-p-hover-text cursor-pointer";

    let padding = match size {
        ButtonSize::Normal => "px-40 py-20 rounded-[10px]",
        ButtonSize::Small => "px-20 py-10 rounded-[10px]",
    };

    rsx! {
        button {
            class: "font-bold gap-10 flex items-center justify-center text-base {bg} {padding}",
            disabled,
            onclick,
            ..attributes,
            {children}
        }
    }
}
