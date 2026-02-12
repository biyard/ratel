use super::*;

#[component]
pub fn ParticipantPage(
    space_id: SpacePartition,
    extensions: Vec<DashboardExtension>,
) -> Element {
    rsx! {
        ViewerPage { space_id, extensions }
    }
}
