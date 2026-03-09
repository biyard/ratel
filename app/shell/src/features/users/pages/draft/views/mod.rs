use dioxus::prelude::*;
use ratel_post::components::{CreatePostButton, MyDrafts};

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        div { class: "flex flex-row",
            div { class: "flex flex-col flex-1",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "text-center text-gray-400 py-4", "Loading drafts..." }
                    },
                    MyDrafts {}
                }
            }
            div { class: "h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static min-w-[280px] w-full",
                CreatePostButton {}
            }
        }
    }
}
