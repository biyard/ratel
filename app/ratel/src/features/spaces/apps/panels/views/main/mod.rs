mod creator;
mod viewer;

pub use super::*;
pub use crate::features::spaces::apps::panels::components::*;
use creator::CreatorPage;
use viewer::ViewerPage;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    if role == SpaceUserRole::Creator {
        rsx! {
            CreatorPage { space_id }
        }
    } else {
        rsx! {
            ViewerPage { space_id }
        }
    }
}
