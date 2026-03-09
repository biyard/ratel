use dioxus::prelude::*;

#[component]
pub fn Home(teamname: String) -> Element {
    rsx! {
        crate::features::posts::components::TeamPosts { key: "{teamname}-posts", teamname }
    }
}
