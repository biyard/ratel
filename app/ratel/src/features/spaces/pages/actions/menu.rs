use super::*;
use icons::game::Thunder;

pub fn get_nav_item(
    space: &SpaceResponse,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    // Creator always sees the actions tab
    if role != SpaceUserRole::Creator {
        match space.status {
            Some(SpaceStatus::InProgress) => {
                // During InProgress, non-creators only see tab if prerequisite actions exist
                if !space.has_prerequisite {
                    return None;
                }
            }
            Some(SpaceStatus::Started) | Some(SpaceStatus::Finished) => {
                // Always show for Started/Finished
            }
            _ => {
                // Draft or other states — hide for non-creators
                return None;
            }
        }
    }

    Some((
        icon(),
        SpacePage::Actions,
        Route::SpaceActionsPage {
            space_id: space.id.clone(),
        }
        .into(),
    ))
}

#[component]
pub fn icon() -> Element {
    rsx! {
        Thunder {
            width: "20",
            height: "20",
            class: "[&>path]:fill-none [&>path]:stroke-[#737373]",
        }
    }
}
