use super::*;

#[component]
pub fn CandidatePage(space_id: SpacePartition) -> Element {
    rsx! {
        ViewerPage { space_id }
    }
}
