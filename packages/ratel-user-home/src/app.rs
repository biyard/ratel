use crate::*;

#[component]
pub fn App(username: String, rest: Vec<String>) -> Element {
    rsx! {
        Router::<Route> {}
    }
}
