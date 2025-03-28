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
    };

    rsx! {
        button {
            class: "font-bold gap-10 flex items-center justify-center text-base {bg} {padding} max-[900px]:!h-50",
            disabled,
            onclick,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PrimaryLink(
    #[props(into)] to: NavigationTarget,
    #[props(default = Default::default())] size: ButtonSize,
    children: Element,

    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let bg =
        "bg-btn-p hover:bg-btn-p-hover text-btn-p-text hover:text-btn-p-hover-text cursor-pointer";
    let padding = match size {
        ButtonSize::Normal => "px-40 py-20",
        ButtonSize::Small => "px-20 py-10",
    };

    rsx! {
        Link {
            to,
            class: "font-bold gap-10 flex items-center justify-center text-base rounded-[10px] {bg} {padding}",
            onclick: move |evt| {
                if let Some(onclick) = onclick {
                    onclick(evt);
                }
            },
            {children}
        }
    }
}
