use dioxus::prelude::*;

#[component]
pub fn AdminMainPage() -> Element {
    rsx! {
        div {
            h1 { "Admin Dashboard" }
        }
    }
}
