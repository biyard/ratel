use super::*;

#[component]
pub fn CandidatePage(space_id: SpacePartition) -> Element {
    rsx! {
        ParticipantPage { space_id }
    }
}
