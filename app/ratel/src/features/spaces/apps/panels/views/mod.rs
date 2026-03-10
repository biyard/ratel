mod creator;
mod viewer;

use super::*;
use crate::features::spaces::space_common::hooks::use_space_role;
use creator::PanelCreatorPage;
use viewer::PanelViewerPage;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    if role == SpaceUserRole::Creator {
        rsx! {
            PanelCreatorPage { space_id }
        }
    } else {
        rsx! {
            PanelViewerPage { space_id }
        }
    }
}
