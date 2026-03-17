use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        crate::features::posts::components::MyPosts { username }
    }
}
