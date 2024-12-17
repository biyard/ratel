#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    layouts::header::Header,
    route::{Language, Route},
    theme::Theme,
};

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: Theme = use_context();
    let theme = theme.get_data();

    rsx! {
        div {
            class: "flex flex-col items-center justify-start w-full min-h-[100vh] text-white",
            style: "background: {theme.background}",
            div {
                class: "max-w-[1440px] grid-cols-[repeat(16,_minmax(0,_1fr))] w-full",
                Header {}
            }
            Outlet::<Route> {}
        }
    }
}
