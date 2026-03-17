use super::*;

#[component]
pub fn ViewerPage(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        OverviewContent {
            space_id,
            editable: false,
        }
    }
}
