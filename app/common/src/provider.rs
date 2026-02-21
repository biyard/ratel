use crate::*;

#[component]
pub fn Provider() -> Element {
    rsx! {
        document::Script { src: asset!("/assets/ratel-common.js") }
    }
}
