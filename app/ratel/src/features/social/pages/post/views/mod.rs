use crate::common::*;

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        crate::features::posts::components::MyPosts { username }
    }
}
