use crate::*;
mod creator;
use creator::SubscriptionCreatorPage;

mod viewer;
use viewer::SubscriptionViewerPage;

use space_common::hooks::use_user_role;
use viewer::*;

#[component]
pub fn MainPage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    match role {
        SpaceUserRole::Creator => rsx! {
            SubscriptionCreatorPage { space_id }
        },
        _ => rsx! {
            SubscriptionViewerPage { space_id }
        },
    }
}
