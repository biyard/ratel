use crate::*;

#[component]
pub fn App(space_id: SpacePartition, rest: Vec<String>) -> Element {
    if !rest.is_empty() {
        return rsx! {
            h2 { "Rest: {rest:?}" }
        };
    }

    rsx! {
        Router::<Route> {}
    }
}
