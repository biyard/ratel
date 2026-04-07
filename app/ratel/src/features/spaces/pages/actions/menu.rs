use super::*;
use icons::game::Thunder;

pub fn get_nav_item(
    space: &SpaceResponse,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    // Creator always sees the actions tab.
    if role != SpaceUserRole::Creator {
        match space.status {
            Some(SpaceStatus::Open) => {
                // During Open, only participants in the prerequisite flow can see Actions.
                if !space.has_prerequisite {
                    return None;
                }

                if !matches!(
                    role,
                    SpaceUserRole::Candidate | SpaceUserRole::Participant
                ) {
                    return None;
                }
            }
            Some(SpaceStatus::Ongoing) => {
                if matches!(role, SpaceUserRole::Viewer) && space.join_anytime {
                    return None;
                }
            }
            Some(SpaceStatus::Processing)
            | Some(SpaceStatus::Finished) => {}
            _ => {
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
