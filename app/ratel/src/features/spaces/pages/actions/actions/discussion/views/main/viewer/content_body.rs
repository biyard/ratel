use crate::common::components::TiptapEditor;
use crate::features::spaces::pages::actions::actions::discussion::*;

#[component]
pub fn DiscussionContentBody(html_contents: String) -> Element {
    if html_contents.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "disc-body",
            div { class: "disc-body__content",
                TiptapEditor {
                    class: "w-full bg-transparent",
                    content: html_contents,
                    editable: false,
                }
            }
        }
    }
}
