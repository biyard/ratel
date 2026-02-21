use super::*;

#[component]
pub fn CandidatePage(
    space_id: SpacePartition,
    extensions: Vec<DashboardExtension>,
) -> Element {
    rsx! {
        ViewerPage { space_id, extensions }
    }
}
