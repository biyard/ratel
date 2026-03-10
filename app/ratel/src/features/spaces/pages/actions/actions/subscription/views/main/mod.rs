use super::*;
mod creator;
use creator::SubscriptionCreatorPage;

mod viewer;
use viewer::SubscriptionViewerPage;

use crate::features::spaces::space_common::hooks::use_space_role;
use viewer::*;

#[component]
pub fn MainPage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();

    match role {
        SpaceUserRole::Creator => rsx! {
            SubscriptionCreatorPage { space_id }
        },
        _ => rsx! {
            SubscriptionViewerPage { space_id }
        },
    }
}
