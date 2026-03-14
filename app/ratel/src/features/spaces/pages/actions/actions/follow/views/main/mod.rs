use super::*;
mod creator;
use creator::FollowCreatorPage;

mod viewer;
use viewer::FollowViewerPage;

use crate::features::spaces::space_common::hooks::use_space_role;
use viewer::*;

#[component]
pub fn FollowActionPage(space_id: SpacePartition, follow_id: SpaceActionFollowEntityType) -> Element {
    let role = use_space_role()();

    match role {
        SpaceUserRole::Creator => rsx! {
            FollowCreatorPage { space_id, follow_id }
        },
        _ => rsx! {
            FollowViewerPage { space_id }
        },
    }
}
