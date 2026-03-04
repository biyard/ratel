mod creator;
mod viewer;

use crate::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    // TODO: Replace with real role check via use_user_role
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            creator::CreatorPage { space_id }
        }
    } else {
        rsx! {
            viewer::ViewerPage { space_id }
        }
    }
}
