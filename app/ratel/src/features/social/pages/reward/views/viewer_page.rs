use dioxus::prelude::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full gap-2",
            h1 { class: "text-2xl font-bold text-text-primary", "No permission" }
            p { class: "text-gray-500", "You don't have permission to view team rewards." }
            p { class: "text-gray-400", "team: {username}" }
        }
    }
}
