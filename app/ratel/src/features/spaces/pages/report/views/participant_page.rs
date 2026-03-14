use super::*;

#[component]
pub fn ParticipantPage(space_id: SpacePartition) -> Element {
    rsx! {
        ViewerPage { space_id }
    }
}
