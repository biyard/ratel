use dioxus::prelude::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full py-10 text-center",
            h1 { class: "text-2xl font-bold text-text-primary", "No permission" }
            p { class: "mt-2 text-text-secondary", "You do not have permission to view team drafts." }
            p { class: "mt-2 text-gray-400", "team: {username}" }
        }
    }
}
