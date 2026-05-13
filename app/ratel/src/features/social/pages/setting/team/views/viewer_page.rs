use super::super::*;
use crate::common::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col justify-center items-center py-10 w-full h-full text-center",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.no_permission_title}" }
            p { class: "mt-2 text-text-secondary", "{tr.no_permission_description}" }
            p { class: "mt-2 text-gray-400", "team: {username}" }
        }
    }
}
