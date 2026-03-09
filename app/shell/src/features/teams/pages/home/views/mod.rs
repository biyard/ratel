use dioxus::prelude::*;

#[component]
pub fn Home(teamname: String) -> Element {
    rsx! {
        ratel_post::components::TeamPosts { key: "{teamname}-posts", teamname }
    }
}
