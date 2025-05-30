#![allow(non_snake_case)]
use super::ButtonSize;
use bdk::prelude::*;

#[component]
pub fn SecondaryButton(
    #[props(default = Default::default())] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    children: Element,

    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let bg = if !disabled {
        "bg-btn-s hover:bg-btn-s-hover text-btn-s-text hover:text-btn-s-hover-text cursor-pointer"
    } else {
        "bg-btn-s-disabled text-btn-s-disabled-text cursor-not-allowed"
    };
    let padding = match size {
        ButtonSize::Normal => "px-40 py-20",
        ButtonSize::Small => "px-20 py-10",
    };

    rsx! {
        button {
            class: "font-bold gap-10 flex items-center justify-center text-base rounded-[10px] {bg} {padding} max-tablet:!w-full max-tablet:!py-15",
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

#[component]
pub fn SecondaryLink(
    #[props(into)] to: NavigationTarget,
    #[props(default = Default::default())] size: ButtonSize,
    children: Element,

    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let bg =
        "bg-btn-s hover:bg-btn-s-hover text-btn-s-text hover:text-btn-s-hover-text cursor-pointer";
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

#[component]
pub fn SecondaryA(
    #[props(default = Default::default())] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    children: Element,
    href: String,

    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let bg = if !disabled {
        "bg-btn-s hover:bg-btn-s-hover text-btn-s-text hover:text-btn-s-hover-text cursor-pointer"
    } else {
        "bg-btn-s-disabled text-btn-s-disabled-text cursor-not-allowed"
    };
    let padding = match size {
        ButtonSize::Normal => "px-40 py-20",
        ButtonSize::Small => "px-20 py-10",
    };

    rsx! {
        a {
            class: "font-bold gap-10 flex items-center justify-center text-base rounded-[10px] {bg} {padding} max-tablet:!w-full max-tablet:!py-15",
            href,
            target: "_blank",
            {children}
        }
    }
}
