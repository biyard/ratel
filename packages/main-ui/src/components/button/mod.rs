#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::theme::Theme;

#[component]
pub fn Button(
    children: Element,
    color: Option<String>,
    background: Option<String>,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let font_theme = theme_service.get_font_theme();

    let color = match color {
        Some(c) => c,
        None => theme.primary00,
    };

    rsx! {
        div {
            class: "{font_theme.bold15} px-[16px] py-[10px] opacity-70 hover:opacity-100 cursor-pointer rounded-[8px]",
            onclick: move |evt| onclick.call(evt),
            style: match background {
                Some(bg) => format!("background-color: {}; color: {}", bg, color),
                None => format!("color: {}", color)
            },
            {children}
        }
    }
}
