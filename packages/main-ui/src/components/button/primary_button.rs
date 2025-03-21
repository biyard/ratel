#![allow(non_snake_case)]
use super::ButtonSize;
use bdk::prelude::*;

#[component]
pub fn PrimaryButton(
    #[props(default = Default::default())] size: ButtonSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,

    #[props(default = false)] disabled: bool,
    children: Element,

    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if !disabled {
        "bg-btn-p hover:bg-btn-p-hover text-btn-p-text hover:text-btn-p-hover-text cursor-pointer"
    } else {
        "bg-btn-p-disabled text-btn-p-disabled-text cursor-not-allowed"
    };
    let padding = match size {
        ButtonSize::Normal => "px-40 py-20 rounded-[10px]",
        ButtonSize::Small => "px-20 py-10 rounded-[10px]",
        ButtonSize::Mobile => "px-[10px] py-[20px] rounded-[10px]",
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
