use crate::*;

#[component]
pub fn App(rest: Vec<String>) -> Element {
    rsx! {
        Router::<Route> {}
    }
}
