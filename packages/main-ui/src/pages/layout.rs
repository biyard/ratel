#![allow(non_snake_case)]
use dioxus::prelude::*;
// use dioxus_popup::PopupZone;
use dioxus_translate::*;

use super::components::*;
use crate::{components::footer::Footer, route::Route};
use by_components::loaders::cube_loader::CubeLoader;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full h-full bg-background text-white",
            Header { lang }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        CubeLoader {}
                    }
                },
            }
            div { class: "w-full overflow-x-hidden scroll-smooth", Outlet::<Route> {} }
            div { class: "max-w-[1440px] w-full",
                Footer { lang }
            }
        }
    }
}
