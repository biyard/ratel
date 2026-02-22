use dioxus::prelude::*;
use ratel_post::components::{CreatePostButton, MyDrafts};

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        div { class: "flex flex-col flex-1 gap-5",
            CreatePostButton {}
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "text-center text-gray-400 py-4", "Loading drafts..." }
                },
                MyDrafts {}
            }
        }
    }
}
