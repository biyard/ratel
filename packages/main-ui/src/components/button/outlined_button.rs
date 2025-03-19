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
    let bg = "hover:bg-btn-o-hover-bg text-btn-o-text hover:text-btn-o-hover-text cursor-pointer border border-btn-o hover:border-btn-o-hover";

    let padding = match size {
        ButtonSize::Normal => "px-40 py-20 rounded-[10px]",
        ButtonSize::Small => "px-20 py-10 rounded-[10px]",
    };

    rsx! {
        button {
            class: "font-bold gap-10 flex items-center justify-center text-base {bg} {padding}",
            onclick,
            ..attributes,
            {children}
        }
    }
}
