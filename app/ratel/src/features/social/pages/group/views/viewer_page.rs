use super::super::*;
use dioxus::prelude::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    let tr: TeamGroupTranslate = use_translate();
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full gap-2",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.no_permission_title}" }
            p { class: "text-gray-500", "{tr.no_permission_description}" }
            p { class: "text-gray-400", "team: {username}" }
        }
    }
}
