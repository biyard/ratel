use super::*;
use crate::components::DashboardGrid;

#[component]
pub fn ParticipantPage(
    space_id: SpacePartition,
    extensions: Vec<DashboardExtension>,
) -> Element {
    if extensions.is_empty() {
        rsx! {
            div { "No dashboard extensions available." }
        }
    } else {
        rsx! {
            div {
                class: "w-full h-full min-h-0",
                DashboardGrid { extensions }
            }
        }
    }
}
