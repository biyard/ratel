use super::super::*;
use dioxus::prelude::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full py-10 text-center",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.no_permission_title}" }
            p { class: "mt-2 text-text-secondary", "{tr.no_permission_description}" }
            p { class: "mt-2 text-gray-400", "team: {username}" }
        }
    }
}
