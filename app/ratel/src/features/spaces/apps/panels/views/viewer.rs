use super::*;

#[component]
pub fn PanelViewerPage(space_id: SpacePartition) -> Element {
    let _ = space_id;

    rsx! {
        div { class: "flex w-full flex-col gap-5", "Panel Viewer Page" }
    }
}
