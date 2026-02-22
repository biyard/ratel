use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    rsx! {
        ratel_post::components::MyDrafts {}
    }
}
