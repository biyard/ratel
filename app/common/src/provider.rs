use crate::*;

#[component]
pub fn Provider() -> Element {
    rsx! {
        document::Script { src: "https://cdn.jsdelivr.net/npm/lucide@0.575.0/dist/cjs/lucide.min.js" }
        document::Script { src: asset!("/assets/ratel-common.js") }
    }
}
