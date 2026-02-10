use super::*;
use crate::components::DashboardGrid;

/// Viewer Dashboard - receives Extension data as props from Space Shell
#[component]
pub fn ViewerPage(
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
                class: "flex flex-col gap-6 w-full",
                DashboardGrid { extensions }
            }
        }
    }
}
