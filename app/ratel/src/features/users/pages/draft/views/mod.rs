use crate::features::posts::components::{CreatePostButton, MyDrafts};
use crate::*;

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        div { class: "flex flex-row",
            div { class: "flex flex-col flex-1",
                SuspenseBoundary { MyDrafts {} }
            }
            div { class: "w-full h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static min-w-[280px]",
                CreatePostButton {}
            }
        }
    }
}
