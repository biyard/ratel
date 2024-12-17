use dioxus::prelude::*;

use crate::route::{Language, Route};

#[component]
pub fn root_layout(lang: Language) -> Element {
    rsx! {
        div {
            Outlet::<Route> {}
        }
    }
}
