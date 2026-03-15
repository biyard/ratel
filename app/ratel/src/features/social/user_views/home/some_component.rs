use dioxus::prelude::*;

#[component]
pub fn SomeComponent() -> Element {
    rsx! {
        div { class: "p-4 border rounded-lg",
            p { class: "text-sm text-gray-600", "This is an example component." }
        }
    }
}
