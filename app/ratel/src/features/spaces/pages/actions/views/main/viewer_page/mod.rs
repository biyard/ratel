use crate::*;

#[component]
pub fn ViewerPage(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        div { id: "viewer-page", "Viewer page" }
    }
}
