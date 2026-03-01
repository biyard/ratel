use crate::views::TeamDaoTranslate;
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn ViewerPage(teamname: String) -> Element {
    let tr: TeamDaoTranslate = use_translate();
    rsx! {
        div { class: "flex flex-col w-full max-w-[1152px] items-center justify-center min-h-[400px]",
            div { class: "text-center",
                h2 { class: "text-2xl font-bold text-text-primary mb-2", "{tr.admin_only}" }
                p { class: "text-text-secondary", "{tr.admin_only_description}" }
                p { class: "mt-2 text-text-tertiary text-sm", "{teamname}" }
            }
        }
    }
}
