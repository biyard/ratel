use super::*;
use icons::game::Thunder;

pub fn get_nav_item(
    space: &SpaceResponse,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    if role == SpaceUserRole::Viewer
        && !matches!(
            space.status,
            Some(SpaceStatus::Ongoing) | Some(SpaceStatus::Finished)
        )
    {
        return None;
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
