use super::creator_page::*;
use crate::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            CreatorPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex items-center justify-center w-full h-full text-font-primary",
                "No permission"
            }
        }
    }
}
