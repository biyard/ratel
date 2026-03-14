use super::*;

#[component]
pub fn CreatorPage(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        OverviewContent {
            space_id,
            editable: true,
        }
    }
}
